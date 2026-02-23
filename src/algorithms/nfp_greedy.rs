use crate::core::{Board, Placement, Shape};
// 采用"最左最下"(Bottom-Left)的启发式规则：优先将零件放置在y坐标最小（靠下）的位置，同y时再考虑x坐标最小（靠左）
// 通过评分函数 score = cy * 宽度 * 2 + cx 实现这一优先级

/// 基于NFP（临界多边形）算法的排料容器
/// 负责管理单个板材上的零件排布
pub struct NFPBin {
    bw: f64,          // 板材宽度
    bh: f64,          // 板材高度
    kerf: f64,        // 切割缝隙（刀具补偿）
    pub placements: Vec<Placement>,  // 已放置的零件列表
}

impl NFPBin {
    /// 创建一个新的NFP排料容器
    pub fn new(bw: f64, bh: f64, kerf: f64) -> Self {
        Self { bw, bh, kerf, placements: vec![] }
    }

    /// 尝试插入一个零件到板材中
    /// shape: 待插入的零件形状
    /// allow_rotate: 是否允许旋转90度
    /// 返回值：如果插入成功返回零件位置信息，否则返回None
    pub fn insert(&mut self, shape: &Shape, allow_rotate: bool) -> Option<Placement> {
        let mut best: Option<(f64, f64, f64, f64, bool)> = None;  // 最优位置参数 (x, y, 宽度, 高度, 是否旋转)
        let mut best_score = f64::INFINITY;  // 最优位置的得分

        // 尝试特定方向的插入
        let mut try_orient = |iw: f64, ih: f64, rotated: bool| {
            // 检查零件尺寸是否超过板材边界
            if iw > self.bw || ih > self.bh {
                return;
            }
            // 计算零件可放置的右边界和下边界
            let ifp_x2 = self.bw - iw;
            let ifp_y2 = self.bh - ih;
            // 获取所有候选放置位置
            let cands = self.candidates(iw, ih, ifp_x2, ifp_y2);
            for (cx, cy) in cands {
                // 检查候选位置是否在有效范围内
                if cx < 0.0 || cy < 0.0 || cx > ifp_x2 || cy > ifp_y2 {
                    continue;
                }
                // 检查是否与已放置零件碰撞
                if self.collides(cx, cy, iw, ih) {
                    continue;
                }
                // 计算得分（基于左下角优先原则，优先考虑y坐标较小的位置）
                let score = cy * self.bw * 2.0 + cx;
                if score < best_score {
                    best_score = score;
                    best = Some((cx, cy, iw, ih, rotated));
                }
            }
        };

        // 尝试不旋转的原始方向
        try_orient(shape.w, shape.h, false);
        // 如果允许旋转且长宽不相等，尝试旋转后的方向
        if allow_rotate && (shape.w - shape.h).abs() > f64::EPSILON {
            try_orient(shape.h, shape.w, true);
        }

        // 如果没有找到合适位置，返回None
        let (bx, by, iw, ih, rotated) = best?;
        // 创建放置信息并添加到容器中
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
        self.placements.push(p.clone());
        Some(p)
    }

    /// 生成候选放置位置
    /// iw, ih: 待放置零件的宽度和高度
    /// ifp_x2, ifp_y2: 零件可放置的右边界和下边界
    /// 返回值：候选位置的坐标列表
    fn candidates(&self, iw: f64, ih: f64, ifp_x2: f64, ifp_y2: f64) -> Vec<(f64, f64)> {
        let k = self.kerf;
        // 初始化候选位置：四个角点
        let mut cands = vec![
            (0.0, 0.0),           // 左上角
            (ifp_x2, 0.0),        // 右上角
            (0.0, ifp_y2),        // 左下角
            (ifp_x2, ifp_y2),     // 右下角
        ];

        // 基于已放置的零件生成候选位置（NFP关键点）
        for pl in &self.placements {
            // 计算NFP的边界
            let nfp_x1 = pl.x - iw;              // 左边界
            let nfp_x2 = pl.x + pl.w + k;         // 右边界（考虑切割缝隙）
            let nfp_y1 = pl.y - ih;              // 上边界
            let nfp_y2 = pl.y + pl.h + k;         // 下边界（考虑切割缝隙）

            // 生成基于边界点的候选位置
            let x_vals = [nfp_x1, nfp_x2, pl.x - iw, pl.x + pl.w];
            let y_vals = [nfp_y1, nfp_y2, pl.y - ih, pl.y + pl.h];
            for &x in &x_vals {
                for &y in &y_vals {
                    cands.push((x, y));
                    // 添加边界约束后的位置
                    cands.push((x.max(0.0).min(ifp_x2), y.max(0.0).min(ifp_y2)));
                }
            }

            // 生成基于跨度范围的候选位置
            let nfp_x_span = [pl.x - iw, pl.x, pl.x + pl.w + k - iw, pl.x + pl.w + k];
            let nfp_y_span = [pl.y - ih, pl.y, pl.y + pl.h + k - ih, pl.y + pl.h + k];

            // 在x边界上生成y跨度范围内的候选点
            for &x in &[nfp_x1.max(0.0).min(ifp_x2), nfp_x2.max(0.0).min(ifp_x2)] {
                for &y in &nfp_y_span {
                    cands.push((x, y.max(0.0).min(ifp_y2)));
                }
            }

            // 在y边界上生成x跨度范围内的候选点
            for &y in &[nfp_y1.max(0.0).min(ifp_y2), nfp_y2.max(0.0).min(ifp_y2)] {
                for &x in &nfp_x_span {
                    cands.push((x.max(0.0).min(ifp_x2), y));
                }
            }
        }
        cands
    }

    /// 检查在指定位置放置零件是否会与已放置零件发生碰撞
    /// x, y: 待放置位置的坐标
    /// iw, ih: 待放置零件的宽度和高度
    /// 返回值：true表示会发生碰撞，false表示安全
    fn collides(&self, x: f64, y: f64, iw: f64, ih: f64) -> bool {
        let k = self.kerf;
        for pl in &self.placements {
            // 检查两个矩形是否重叠（考虑切割缝隙）
            if x < pl.x + pl.w + k && x + iw > pl.x &&
               y < pl.y + pl.h + k && y + ih > pl.y {
                return true;
            }
        }
        false
    }
}

/// 对一组零件执行NFP排料算法
/// shapes: 待排料的零件列表
/// bw, bh: 板材的宽度和高度
/// kerf: 切割缝隙
/// allow_rotate: 是否允许旋转
/// 返回值：包含已放置零件的板材和未放置的零件列表
pub fn pack_nfp(shapes: &[Shape], bw: f64, bh: f64, kerf: f64, allow_rotate: bool) -> (Board, Vec<Shape>) {
    let mut remaining = vec![];  // 未能放置的零件
    let mut board = Board::default();  // 板材对象
    let mut bin = NFPBin::new(bw, bh, kerf);  // 创建NFP容器

    // 尝试放置每个零件
    for s in shapes {
        if bin.insert(s, allow_rotate).is_none() {
            remaining.push(s.clone());  // 放置失败，加入剩余列表
        }
    }

    board.placements = bin.placements;  // 将放置结果转移到板材对象
    (board, remaining)
}
