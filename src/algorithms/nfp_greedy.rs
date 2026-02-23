use crate::core::{Board, Placement, Shape};

pub struct NFPBin {
    bw: f64,
    bh: f64,
    kerf: f64,
    pub placements: Vec<Placement>,
}

impl NFPBin {
    pub fn new(bw: f64, bh: f64, kerf: f64) -> Self {
        Self { bw, bh, kerf, placements: vec![] }
    }

    pub fn insert(&mut self, shape: &Shape, allow_rotate: bool) -> Option<Placement> {
        let mut best: Option<(f64, f64, f64, f64, bool)> = None;
        let mut best_score = f64::INFINITY;

        let mut try_orient = |iw: f64, ih: f64, rotated: bool| {
            if iw > self.bw || ih > self.bh {
                return;
            }
            let ifp_x2 = self.bw - iw;
            let ifp_y2 = self.bh - ih;
            let cands = self.candidates(iw, ih, ifp_x2, ifp_y2);
            for (cx, cy) in cands {
                if cx < 0.0 || cy < 0.0 || cx > ifp_x2 || cy > ifp_y2 {
                    continue;
                }
                if self.collides(cx, cy, iw, ih) {
                    continue;
                }
                let score = cy * self.bw * 2.0 + cx;
                if score < best_score {
                    best_score = score;
                    best = Some((cx, cy, iw, ih, rotated));
                }
            }
        };

        try_orient(shape.w, shape.h, false);
        if allow_rotate && (shape.w - shape.h).abs() > f64::EPSILON {
            try_orient(shape.h, shape.w, true);
        }

        let (bx, by, iw, ih, rotated) = best?;
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

    fn candidates(&self, iw: f64, ih: f64, ifp_x2: f64, ifp_y2: f64) -> Vec<(f64, f64)> {
        let k = self.kerf;
        let mut cands = vec![
            (0.0, 0.0),
            (ifp_x2, 0.0),
            (0.0, ifp_y2),
            (ifp_x2, ifp_y2),
        ];
        for pl in &self.placements {
            let nfp_x1 = pl.x - iw;
            let nfp_x2 = pl.x + pl.w + k;
            let nfp_y1 = pl.y - ih;
            let nfp_y2 = pl.y + pl.h + k;
            let x_vals = [nfp_x1, nfp_x2, pl.x - iw, pl.x + pl.w];
            let y_vals = [nfp_y1, nfp_y2, pl.y - ih, pl.y + pl.h];
            for &x in &x_vals {
                for &y in &y_vals {
                    cands.push((x, y));
                    cands.push((x.max(0.0).min(ifp_x2), y.max(0.0).min(ifp_y2)));
                }
            }
            let nfp_x_span = [pl.x - iw, pl.x, pl.x + pl.w + k - iw, pl.x + pl.w + k];
            let nfp_y_span = [pl.y - ih, pl.y, pl.y + pl.h + k - ih, pl.y + pl.h + k];
            for &x in &[nfp_x1.max(0.0).min(ifp_x2), nfp_x2.max(0.0).min(ifp_x2)] {
                for &y in &nfp_y_span {
                    cands.push((x, y.max(0.0).min(ifp_y2)));
                }
            }
            for &y in &[nfp_y1.max(0.0).min(ifp_y2), nfp_y2.max(0.0).min(ifp_y2)] {
                for &x in &nfp_x_span {
                    cands.push((x.max(0.0).min(ifp_x2), y));
                }
            }
        }
        cands
    }

    fn collides(&self, x: f64, y: f64, iw: f64, ih: f64) -> bool {
        let k = self.kerf;
        for pl in &self.placements {
            if x < pl.x + pl.w + k && x + iw > pl.x && y < pl.y + pl.h + k && y + ih > pl.y {
                return true;
            }
        }
        false
    }
}

pub fn pack_nfp(shapes: &[Shape], bw: f64, bh: f64, kerf: f64, allow_rotate: bool) -> (Board, Vec<Shape>) {
    let mut remaining = vec![];
    let mut board = Board::default();
    let mut bin = NFPBin::new(bw, bh, kerf);
    for s in shapes {
        if bin.insert(s, allow_rotate).is_none() {
            remaining.push(s.clone());
        }
    }
    board.placements = bin.placements;
    (board, remaining)
}
