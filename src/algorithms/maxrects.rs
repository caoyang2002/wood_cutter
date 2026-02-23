use crate::core::{contains, overlaps, Board, Placement, Rect, Shape};

pub struct MaxRectsBin {
    bw: f64,
    bh: f64,
    kerf: f64,
    free: Vec<Rect>,
    pub placements: Vec<Placement>,
}

impl MaxRectsBin {
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
        let mut best: Option<(f64, f64, f64, bool)> = None;
        let mut best_score = f64::INFINITY;

        let try_orient = |iw: f64, ih: f64, rotated: bool, free: &Vec<Rect>,
                          best_score: &mut f64, best: &mut Option<(f64, f64, f64, bool)>| {
            for fr in free.iter() {
                if fr.w >= iw && fr.h >= ih {
                    let sc = (fr.w - iw).min(fr.h - ih) * 1e8 + (fr.w - iw).max(fr.h - ih);
                    if sc < *best_score {
                        *best_score = sc;
                        *best = Some((fr.x, fr.y, sc, rotated));
                    }
                }
            }
        };

        try_orient(shape.w, shape.h, false, &self.free, &mut best_score, &mut best);
        if allow_rotate && (shape.w - shape.h).abs() > f64::EPSILON {
            try_orient(shape.h, shape.w, true, &self.free, &mut best_score, &mut best);
        }

        let (bx, by, _, rotated) = best?;
        let (iw, ih) = if rotated { (shape.h, shape.w) } else { (shape.w, shape.h) };

        let placed = Rect { x: bx, y: by, w: iw, h: ih };
        self.split(&placed);
        self.prune();

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

    fn split(&mut self, pl: &Rect) {
        let k = self.kerf;
        let mut nf = vec![];
        for fr in self.free.drain(..) {
            if !overlaps(pl, &fr) {
                nf.push(fr);
                continue;
            }
            let rw = fr.x + fr.w - pl.x - pl.w - k;
            if rw > 0.0 {
                nf.push(Rect { x: pl.x + pl.w + k, y: fr.y, w: rw, h: fr.h });
            }
            if pl.x > fr.x {
                nf.push(Rect { x: fr.x, y: fr.y, w: pl.x - fr.x, h: fr.h });
            }
            let bh = fr.y + fr.h - pl.y - pl.h - k;
            if bh > 0.0 {
                nf.push(Rect { x: fr.x, y: pl.y + pl.h + k, w: fr.w, h: bh });
            }
            if pl.y > fr.y {
                nf.push(Rect { x: fr.x, y: fr.y, w: fr.w, h: pl.y - fr.y });
            }
        }
        self.free = nf.into_iter().filter(|r| r.w > 0.0 && r.h > 0.0).collect();
    }

    fn prune(&mut self) {
        let n = self.free.len();
        let keep: Vec<bool> = (0..n)
            .map(|i| !(0..n).any(|j| i != j && contains(&self.free[j], &self.free[i])))
            .collect();
        let f = self.free.clone();
        self.free = f.into_iter().zip(keep).filter(|(_, k)| *k).map(|(r, _)| r).collect();
    }
}

pub fn pack_maxrects(shapes: &[Shape], bw: f64, bh: f64, kerf: f64, allow_rotate: bool) -> (Board, Vec<Shape>) {
    let mut remaining = vec![];
    let mut board = Board::default();
    let mut bin = MaxRectsBin::new(bw, bh, kerf);
    for s in shapes {
        if bin.insert(s, allow_rotate).is_none() {
            remaining.push(s.clone());
        }
    }
    board.placements = bin.placements;
    (board, remaining)
}
