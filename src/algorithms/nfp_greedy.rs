use crate::core::{Board, Placement, Shape};
use nfp::continuous::{NfpResult, nfp_convex_polygons, nfp_non_convex_polygons};
use geo::{Polygon, polygon, Coord, LineString};

pub struct NFPBin {
    bw: f64,
    bh: f64,
    kerf: f64,
    pub placements: Vec<Placement>,
    // 缓存已放置形状的多边形表示
    placed_polygons: Vec<Polygon<f64>>,
}

impl NFPBin {
    pub fn new(bw: f64, bh: f64, kerf: f64) -> Self {
        Self {
            bw,
            bh,
            kerf,
            placements: vec![],
            placed_polygons: vec![],
        }
    }

    /// 将形状转换为多边形（考虑锯缝）
    fn shape_to_polygon(&self, x: f64, y: f64, w: f64, h: f64) -> Polygon<f64> {
        let k = self.kerf / 2.0; // 锯缝均匀扩展到四周
        polygon![
            Coord { x: x - k, y: y - k },
            Coord { x: x + w + k, y: y - k },
            Coord { x: x + w + k, y: y + h + k },
            Coord { x: x - k, y: y + h + k },
        ]
    }

    /// 将板材转换为多边形（考虑边界）
    fn bin_to_polygon(&self) -> Polygon<f64> {
        polygon![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: self.bw, y: 0.0 },
            Coord { x: self.bw, y: self.bh },
            Coord { x: 0.0, y: self.bh },
        ]
    }

    pub fn insert(&mut self, shape: &Shape, allow_rotate: bool) -> Option<Placement> {
        let mut best: Option<(f64, f64, f64, f64, bool)> = None;
        let mut best_score = f64::INFINITY;

        // 尝试两种方向
        let orientations = [
            (shape.w, shape.h, false),
            (shape.h, shape.w, true),
        ];

        for (iw, ih, rotated) in orientations.iter() {
            if !allow_rotate && *rotated && (shape.w - shape.h).abs() > f64::EPSILON {
                continue;
            }

            if *iw > self.bw || *ih > self.bh {
                continue;
            }

            // 获取候选位置
            let cands = self.candidates(*iw, *ih);

            for (cx, cy) in cands {
                if cx < 0.0 || cy < 0.0 || cx + *iw > self.bw || cy + *ih > self.bh {
                    continue;
                }

                // 使用 NFP 库检查碰撞
                if self.collides_nfp(cx, cy, *iw, *ih) {
                    continue;
                }

                // 评分策略：优先放在左下角（类似 Shelf 算法）
                // 可以调整评分策略以获得更好的 packing 效果
                let score = cy * self.bw * 2.0 + cx;
                if score < best_score {
                    best_score = score;
                    best = Some((cx, cy, *iw, *ih, *rotated));
                }
            }
        }

        let (bx, by, iw, ih, rotated) = best?;

        // 创建放置记录
        let p = Placement {
            x: bx,
            y: by,
            w: iw,
            h: ih,
            rotated,
            name: shape.name.clone(),
            color_idx: shape.color_idx,
            shape_id: shape.id,
        };

        // 添加放置的多边形到缓存
        self.placed_polygons.push(self.shape_to_polygon(bx, by, iw, ih));
        self.placements.push(p.clone());

        Some(p)
    }

    /// 使用 NFP 库进行碰撞检测
    fn collides_nfp(&self, x: f64, y: f64, iw: f64, ih: f64) -> bool {
        let new_poly = self.shape_to_polygon(x, y, iw, ih);
        let bin_poly = self.bin_to_polygon();

        // 检查是否超出边界
        if !bin_poly.contains(&new_poly) {
            return true;
        }

        // 检查与已放置形状的碰撞
        for placed in &self.placed_polygons {
            // 使用 NFP 检查两个多边形是否相交
            // 首先尝试凸多边形 NFP（更快）
            let result = nfp_convex_polygons(&new_poly, placed);

            match result {
                NfpResult::Intersecting => return true,
                NfpResult::Touching => {
                    // 如果只是接触（有锯缝允许），需要检查是否在锯缝范围内
                    if self.overlaps_with_kerf(&new_poly, placed) {
                        return true;
                    }
                }
                NfpResult::Separate => continue,
            }
        }

        false
    }

    /// 检查两个形状是否在锯缝范围内重叠
    fn overlaps_with_kerf(&self, poly1: &Polygon<f64>, poly2: &Polygon<f64>) -> bool {
        // 如果形状只是接触，但在锯缝范围内可能仍有重叠
        // 可以通过膨胀多边形来检查
        let expanded1 = self.expand_polygon(poly1, self.kerf / 2.0);
        let expanded2 = self.expand_polygon(poly2, self.kerf / 2.0);

        matches!(
            nfp_convex_polygons(&expanded1, &expanded2),
            NfpResult::Intersecting | NfpResult::Touching
        )
    }

    /// 膨胀多边形（模拟锯缝）
    fn expand_polygon(&self, poly: &Polygon<f64>, distance: f64) -> Polygon<f64> {
        // 简单的膨胀实现 - 实际项目中可能需要使用更精确的缓冲区算法
        let coords: Vec<Coord<f64>> = poly.exterior().points().map(|p| {
            Coord {
                x: p.x() + distance.copysign(p.x()),
                y: p.y() + distance.copysign(p.y()),
            }
        }).collect();

        Polygon::new(LineString::from(coords), vec![])
    }

    fn candidates(&self, iw: f64, ih: f64) -> Vec<(f64, f64)> {
        let mut cands = vec![
            (0.0, 0.0),           // 左下角
            (self.bw - iw, 0.0),  // 右下角
            (0.0, self.bh - ih),  // 左上角
            (self.bw - iw, self.bh - ih), // 右上角
        ];

        // 从已放置形状的边缘生成候选位置
        for pl in &self.placements {
            // 右侧放置
            cands.push((pl.x + pl.w + self.kerf, pl.y));
            // 上方放置
            cands.push((pl.x, pl.y + pl.h + self.kerf));
            // 角落放置
            cands.push((pl.x + pl.w + self.kerf, pl.y + pl.h + self.kerf));

            // 左对齐
            cands.push((pl.x - iw - self.kerf, pl.y));
            cands.push((pl.x - iw - self.kerf, pl.y + pl.h - ih));

            // 下对齐
            cands.push((pl.x, pl.y - ih - self.kerf));
            cands.push((pl.x + pl.w - iw, pl.y - ih - self.kerf));
        }

        // 过滤掉超出边界的候选位置
        cands.into_iter()
            .filter(|(x, y)| {
                *x >= 0.0 && *y >= 0.0 && *x + iw <= self.bw && *y + ih <= self.bh
            })
            .collect()
    }
}

pub fn pack_nfp(shapes: &[Shape], bw: f64, bh: f64, kerf: f64, allow_rotate: bool) -> (Board, Vec<Shape>) {
    let mut remaining = vec![];
    let mut board = Board::default();
    let mut bin = NFPBin::new(bw, bh, kerf);

    // 可以添加形状排序策略以获得更好的 packing 效果
    let mut sorted_shapes: Vec<&Shape> = shapes.iter().collect();
    sorted_shapes.sort_by(|a, b| {
        // 按面积降序排列（先放大的形状）
        let area_a = a.w * a.h;
        let area_b = b.w * b.h;
        area_b.partial_cmp(&area_a).unwrap_or(std::cmp::Ordering::Equal)
    });

    for s in sorted_shapes {
        if bin.insert(s, allow_rotate).is_none() {
            remaining.push(s.clone());
        }
    }

    board.placements = bin.placements;
    (board, remaining)
}
