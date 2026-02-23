use crate::core::{Board, Placement, Shape};

#[derive(Clone)]
struct SkylineSeg {
    x: f64,
    y: f64,
    w: f64,
}

pub struct BottomLeftBin {
    bw: f64,
    bh: f64,
    kerf: f64,
    skyline: Vec<SkylineSeg>,
    pub placements: Vec<Placement>,
}

impl BottomLeftBin {
    pub fn new(bw: f64, bh: f64, kerf: f64) -> Self {
        Self {
            bw,
            bh,
            kerf,
            skyline: vec![SkylineSeg { x: 0.0, y: 0.0, w: bw }],
            placements: vec![],
        }
    }

    fn sky_y(&self, x: f64, w: f64) -> f64 {
        let mut max_y = 0.0_f64;
        for seg in &self.skyline {
            if seg.x + seg.w <= x || seg.x >= x + w {
                continue;
            }
            if seg.y > max_y {
                max_y = seg.y;
            }
        }
        max_y
    }

    pub fn insert(&mut self, shape: &Shape, allow_rotate: bool) -> Option<Placement> {
        let mut best: Option<(f64, f64, f64, f64, bool)> = None;
        let mut best_y = f64::INFINITY;
        let mut best_x = f64::INFINITY;

        let mut try_orient = |iw: f64, ih: f64, rotated: bool| {
            for seg in self.skyline.clone().iter() {
                for xi in 0..2 {
                    let cx = if xi == 0 {
                        seg.x
                    } else {
                        (seg.x + seg.w - iw).max(0.0)
                    };
                    if cx < 0.0 || cx + iw > self.bw {
                        continue;
                    }
                    let max_y = self.sky_y(cx, iw);
                    if max_y + ih > self.bh {
                        continue;
                    }
                    if max_y < best_y || (max_y == best_y && cx < best_x) {
                        best_y = max_y;
                        best_x = cx;
                        best = Some((cx, max_y, iw, ih, rotated));
                    }
                }
            }
        };

        try_orient(shape.w, shape.h, false);
        if allow_rotate && (shape.w - shape.h).abs() > f64::EPSILON {
            try_orient(shape.h, shape.w, true);
        }

        let (bx, by, iw, ih, rotated) = best?;
        self.update_skyline(bx, by, iw, ih);

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

    fn update_skyline(&mut self, nx: f64, ny: f64, nw: f64, nh: f64) {
        let k = self.kerf;
        let ne = nx + nw;
        let new_y = ny + nh + k;
        let mut new_sky: Vec<SkylineSeg> = vec![];
        for seg in &self.skyline {
            let ex = seg.x + seg.w;
            if ex <= nx || seg.x >= ne {
                new_sky.push(seg.clone());
                continue;
            }
            if seg.x < nx {
                new_sky.push(SkylineSeg { x: seg.x, y: seg.y, w: nx - seg.x });
            }
            if ex > ne {
                new_sky.push(SkylineSeg { x: ne, y: seg.y, w: ex - ne });
            }
        }
        new_sky.push(SkylineSeg { x: nx, y: new_y, w: nw });
        new_sky.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        let mut merged: Vec<SkylineSeg> = vec![];
        for s in new_sky {
            if let Some(last) = merged.last_mut() {
                if (last.y - s.y).abs() < f64::EPSILON
                    && (last.x + last.w - s.x).abs() < f64::EPSILON
                {
                    last.w += s.w;
                    continue;
                }
            }
            merged.push(s);
        }
        self.skyline = merged;
    }
}

pub fn pack_bottom_left(shapes: &[Shape], bw: f64, bh: f64, kerf: f64, allow_rotate: bool) -> (Board, Vec<Shape>) {
    let mut remaining = vec![];
    let mut board = Board::default();
    let mut bin = BottomLeftBin::new(bw, bh, kerf);
    for s in shapes {
        if bin.insert(s, allow_rotate).is_none() {
            remaining.push(s.clone());
        }
    }
    board.placements = bin.placements;
    (board, remaining)
}
