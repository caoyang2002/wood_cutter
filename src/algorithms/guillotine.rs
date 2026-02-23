use crate::core::{Board, Placement, Rect, Shape};

pub struct GuillotineBin {
    bw: f64,
    bh: f64,
    kerf: f64,
    free: Vec<Rect>,
    pub placements: Vec<Placement>,
}

impl GuillotineBin {
    pub fn new(bw: f64, bh: f64, kerf: f64) -> Self {
        Self {
            bw,
            bh,
            kerf,
            free: vec![Rect { x: 0.0, y: 0.0, w: bw, h: bh }],
            placements: vec![],
        }
    }

    pub fn insert(&mut self, shape: &Shape, allow_rotate: bool) -> Option<Placement> {
        let mut best_fr_idx: Option<usize> = None;
        let mut best_iw = 0.0_f64;
        let mut best_ih = 0.0_f64;
        let mut best_rotated = false;
        let mut best_score = f64::INFINITY;

        for (fi, fr) in self.free.iter().enumerate() {
            let try_o = |iw: f64, ih: f64| -> Option<f64> {
                if fr.w >= iw && fr.h >= ih {
                    Some(fr.w * fr.h - iw * ih)
                } else {
                    None
                }
            };
            if let Some(sc) = try_o(shape.w, shape.h) {
                if sc < best_score {
                    best_score = sc;
                    best_fr_idx = Some(fi);
                    best_iw = shape.w;
                    best_ih = shape.h;
                    best_rotated = false;
                }
            }
            if allow_rotate && (shape.w - shape.h).abs() > f64::EPSILON {
                if let Some(sc) = try_o(shape.h, shape.w) {
                    if sc < best_score {
                        best_score = sc;
                        best_fr_idx = Some(fi);
                        best_iw = shape.h;
                        best_ih = shape.w;
                        best_rotated = true;
                    }
                }
            }
        }

        let fi = best_fr_idx?;
        let bfr = self.free.remove(fi);
        let k = self.kerf;
        let bx = bfr.x;
        let by = bfr.y;
        let iw = best_iw;
        let ih = best_ih;

        let right_w = bfr.w - iw - k;
        let bot_h = bfr.h - ih - k;
        if right_w >= bot_h {
            if right_w > 0.0 {
                self.free.push(Rect { x: bx + iw + k, y: bfr.y, w: right_w, h: bfr.h });
            }
            if bot_h > 0.0 {
                self.free.push(Rect { x: bfr.x, y: by + ih + k, w: iw, h: bot_h });
            }
        } else {
            if bot_h > 0.0 {
                self.free.push(Rect { x: bfr.x, y: by + ih + k, w: bfr.w, h: bot_h });
            }
            if right_w > 0.0 {
                self.free.push(Rect { x: bx + iw + k, y: bfr.y, w: right_w, h: ih });
            }
        }

        let p = Placement {
            x: bx,
            y: by,
            w: iw,
            h: ih,
            rotated: best_rotated,
            name: shape.name.clone(),
            color_idx: shape.color_idx,
            shape_id: shape.id,
        };
        self.placements.push(p.clone());
        Some(p)
    }
}

pub fn pack_guillotine(shapes: &[Shape], bw: f64, bh: f64, kerf: f64, allow_rotate: bool) -> (Board, Vec<Shape>) {
    let mut remaining = vec![];
    let mut board = Board::default();
    let mut bin = GuillotineBin::new(bw, bh, kerf);
    for s in shapes {
        if bin.insert(s, allow_rotate).is_none() {
            remaining.push(s.clone());
        }
    }
    board.placements = bin.placements;
    (board, remaining)
}
