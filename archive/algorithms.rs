// use rand::prelude::*;

// // ─────────────────────────────────────────────
// //  Data Structures
// // ─────────────────────────────────────────────

// #[derive(Clone, Debug)]
// pub struct Shape {
//     pub id: usize,
//     pub name: String,
//     pub w: f64,
//     pub h: f64,
//     pub color_idx: usize,
// }

// #[derive(Clone, Debug)]
// pub struct Placement {
//     pub x: f64,
//     pub y: f64,
//     pub w: f64,
//     pub h: f64,
//     pub rotated: bool,
//     pub name: String,
//     pub color_idx: usize,
//     pub shape_id: usize,
// }

// #[derive(Clone, Debug, Default)]
// pub struct Board {
//     pub placements: Vec<Placement>,
// }

// #[derive(Clone, Debug)]
// pub struct PackResult {
//     pub boards: Vec<Board>,
//     pub unfittable: Vec<Shape>,
// }

// #[derive(Clone, Debug)]
// struct Rect {
//     x: f64,
//     y: f64,
//     w: f64,
//     h: f64,
// }

// fn overlaps(a: &Rect, b: &Rect) -> bool {
//     a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
// }
// fn contains(outer: &Rect, inner: &Rect) -> bool {
//     outer.x <= inner.x
//         && outer.y <= inner.y
//         && outer.x + outer.w >= inner.x + inner.w
//         && outer.y + outer.h >= inner.y + inner.h
// }

// // ─────────────────────────────────────────────
// //  MaxRects Bin
// // ─────────────────────────────────────────────
// struct MaxRectsBin {
//     bw: f64,
//     bh: f64,
//     kerf: f64,
//     free: Vec<Rect>,
//     pub placements: Vec<Placement>,
// }

// impl MaxRectsBin {
//     fn new(bw: f64, bh: f64, kerf: f64) -> Self {
//         Self {
//             bw,
//             bh,
//             kerf,
//             free: vec![Rect { x: 0.0, y: 0.0, w: bw, h: bh }],
//             placements: vec![],
//         }
//     }

//     fn insert(&mut self, shape: &Shape, allow_rotate: bool) -> Option<Placement> {
//         let mut best: Option<(f64, f64, f64, bool)> = None; // (x,y, score, rotated)
//         let mut best_score = f64::INFINITY;

//         let try_orient = |iw: f64, ih: f64, rotated: bool, free: &Vec<Rect>,
//                           best_score: &mut f64, best: &mut Option<(f64, f64, f64, bool)>| {
//             for fr in free.iter() {
//                 if fr.w >= iw && fr.h >= ih {
//                     let sc = (fr.w - iw).min(fr.h - ih) * 1e8 + (fr.w - iw).max(fr.h - ih);
//                     if sc < *best_score {
//                         *best_score = sc;
//                         *best = Some((fr.x, fr.y, sc, rotated));
//                     }
//                 }
//             }
//         };

//         try_orient(shape.w, shape.h, false, &self.free, &mut best_score, &mut best);
//         if allow_rotate && (shape.w - shape.h).abs() > f64::EPSILON {
//             try_orient(shape.h, shape.w, true, &self.free, &mut best_score, &mut best);
//         }

//         let (bx, by, _, rotated) = best?;
//         let (iw, ih) = if rotated { (shape.h, shape.w) } else { (shape.w, shape.h) };

//         let placed = Rect { x: bx, y: by, w: iw, h: ih };
//         self.split(&placed);
//         self.prune();

//         let p = Placement {
//             x: bx,
//             y: by,
//             w: iw,
//             h: ih,
//             rotated,
//             name: shape.name.clone(),
//             color_idx: shape.color_idx,
//             shape_id: shape.id,
//         };
//         self.placements.push(p.clone());
//         Some(p)
//     }

//     fn split(&mut self, pl: &Rect) {
//         let k = self.kerf;
//         let mut nf = vec![];
//         for fr in self.free.drain(..) {
//             if !overlaps(pl, &fr) {
//                 nf.push(fr);
//                 continue;
//             }
//             let rw = fr.x + fr.w - pl.x - pl.w - k;
//             if rw > 0.0 { nf.push(Rect { x: pl.x + pl.w + k, y: fr.y, w: rw, h: fr.h }); }
//             if pl.x > fr.x { nf.push(Rect { x: fr.x, y: fr.y, w: pl.x - fr.x, h: fr.h }); }
//             let bh = fr.y + fr.h - pl.y - pl.h - k;
//             if bh > 0.0 { nf.push(Rect { x: fr.x, y: pl.y + pl.h + k, w: fr.w, h: bh }); }
//             if pl.y > fr.y { nf.push(Rect { x: fr.x, y: fr.y, w: fr.w, h: pl.y - fr.y }); }
//         }
//         self.free = nf.into_iter().filter(|r| r.w > 0.0 && r.h > 0.0).collect();
//     }

//     fn prune(&mut self) {
//         let free = self.free.clone();
//         self.free
//             .retain(|a| !free.iter().enumerate().any(|(_, b)| !std::ptr::eq(a, b) && contains(b, a)));
//         // simpler: n^2 filter
//         let n = self.free.len();
//         let keep: Vec<bool> = (0..n)
//             .map(|i| !(0..n).any(|j| i != j && contains(&self.free[j], &self.free[i])))
//             .collect();
//         let f = self.free.clone();
//         self.free = f.into_iter().zip(keep).filter(|(_, k)| *k).map(|(r, _)| r).collect();
//     }
// }

// // ─────────────────────────────────────────────
// //  Guillotine Bin
// // ─────────────────────────────────────────────
// struct GuillotineBin {
//     bw: f64,
//     bh: f64,
//     kerf: f64,
//     free: Vec<Rect>,
//     pub placements: Vec<Placement>,
// }

// impl GuillotineBin {
//     fn new(bw: f64, bh: f64, kerf: f64) -> Self {
//         Self {
//             bw,
//             bh,
//             kerf,
//             free: vec![Rect { x: 0.0, y: 0.0, w: bw, h: bh }],
//             placements: vec![],
//         }
//     }

//     fn insert(&mut self, shape: &Shape, allow_rotate: bool) -> Option<Placement> {
//         let mut best_fr_idx: Option<usize> = None;
//         let mut best_iw = 0.0_f64;
//         let mut best_ih = 0.0_f64;
//         let mut best_rotated = false;
//         let mut best_score = f64::INFINITY;

//         for (fi, fr) in self.free.iter().enumerate() {
//             let try_o = |iw: f64, ih: f64, _rot: bool| -> Option<f64> {
//                 if fr.w >= iw && fr.h >= ih {
//                     Some(fr.w * fr.h - iw * ih)
//                 } else {
//                     None
//                 }
//             };
//             if let Some(sc) = try_o(shape.w, shape.h, false) {
//                 if sc < best_score {
//                     best_score = sc;
//                     best_fr_idx = Some(fi);
//                     best_iw = shape.w;
//                     best_ih = shape.h;
//                     best_rotated = false;
//                 }
//             }
//             if allow_rotate && (shape.w - shape.h).abs() > f64::EPSILON {
//                 if let Some(sc) = try_o(shape.h, shape.w, true) {
//                     if sc < best_score {
//                         best_score = sc;
//                         best_fr_idx = Some(fi);
//                         best_iw = shape.h;
//                         best_ih = shape.w;
//                         best_rotated = true;
//                     }
//                 }
//             }
//         }

//         let fi = best_fr_idx?;
//         let bfr = self.free.remove(fi);
//         let k = self.kerf;
//         let bx = bfr.x;
//         let by = bfr.y;
//         let iw = best_iw;
//         let ih = best_ih;

//         let right_w = bfr.w - iw - k;
//         let bot_h = bfr.h - ih - k;
//         if right_w >= bot_h {
//             if right_w > 0.0 {
//                 self.free.push(Rect { x: bx + iw + k, y: bfr.y, w: right_w, h: bfr.h });
//             }
//             if bot_h > 0.0 {
//                 self.free.push(Rect { x: bfr.x, y: by + ih + k, w: iw, h: bot_h });
//             }
//         } else {
//             if bot_h > 0.0 {
//                 self.free.push(Rect { x: bfr.x, y: by + ih + k, w: bfr.w, h: bot_h });
//             }
//             if right_w > 0.0 {
//                 self.free.push(Rect { x: bx + iw + k, y: bfr.y, w: right_w, h: ih });
//             }
//         }

//         let p = Placement {
//             x: bx,
//             y: by,
//             w: iw,
//             h: ih,
//             rotated: best_rotated,
//             name: shape.name.clone(),
//             color_idx: shape.color_idx,
//             shape_id: shape.id,
//         };
//         self.placements.push(p.clone());
//         Some(p)
//     }
// }

// // ─────────────────────────────────────────────
// //  Bottom-Left Skyline
// // ─────────────────────────────────────────────
// #[derive(Clone)]
// struct SkylineSeg {
//     x: f64,
//     y: f64,
//     w: f64,
// }

// struct BottomLeftBin {
//     bw: f64,
//     bh: f64,
//     kerf: f64,
//     skyline: Vec<SkylineSeg>,
//     pub placements: Vec<Placement>,
// }

// impl BottomLeftBin {
//     fn new(bw: f64, bh: f64, kerf: f64) -> Self {
//         Self {
//             bw,
//             bh,
//             kerf,
//             skyline: vec![SkylineSeg { x: 0.0, y: 0.0, w: bw }],
//             placements: vec![],
//         }
//     }

//     fn sky_y(&self, x: f64, w: f64) -> f64 {
//         let mut max_y = 0.0_f64;
//         for seg in &self.skyline {
//             if seg.x + seg.w <= x || seg.x >= x + w {
//                 continue;
//             }
//             if seg.y > max_y {
//                 max_y = seg.y;
//             }
//         }
//         max_y
//     }

//     fn insert(&mut self, shape: &Shape, allow_rotate: bool) -> Option<Placement> {
//         let mut best: Option<(f64, f64, f64, f64, bool)> = None;
//         let mut best_y = f64::INFINITY;
//         let mut best_x = f64::INFINITY;

//         let mut try_orient = |iw: f64, ih: f64, rotated: bool| {
//             for seg in self.skyline.clone().iter() {
//                 for xi in 0..2 {
//                     let cx = if xi == 0 {
//                         seg.x
//                     } else {
//                         (seg.x + seg.w - iw).max(0.0)
//                     };
//                     if cx < 0.0 || cx + iw > self.bw {
//                         continue;
//                     }
//                     let max_y = self.sky_y(cx, iw);
//                     if max_y + ih > self.bh {
//                         continue;
//                     }
//                     if max_y < best_y || (max_y == best_y && cx < best_x) {
//                         best_y = max_y;
//                         best_x = cx;
//                         best = Some((cx, max_y, iw, ih, rotated));
//                     }
//                 }
//             }
//         };

//         try_orient(shape.w, shape.h, false);
//         if allow_rotate && (shape.w - shape.h).abs() > f64::EPSILON {
//             try_orient(shape.h, shape.w, true);
//         }

//         let (bx, by, iw, ih, rotated) = best?;
//         self.update_skyline(bx, by, iw, ih);

//         let p = Placement {
//             x: bx,
//             y: by,
//             w: iw,
//             h: ih,
//             rotated,
//             name: shape.name.clone(),
//             color_idx: shape.color_idx,
//             shape_id: shape.id,
//         };
//         self.placements.push(p.clone());
//         Some(p)
//     }

//     fn update_skyline(&mut self, nx: f64, ny: f64, nw: f64, nh: f64) {
//         let k = self.kerf;
//         let ne = nx + nw;
//         let new_y = ny + nh + k;
//         let mut new_sky: Vec<SkylineSeg> = vec![];
//         for seg in &self.skyline {
//             let ex = seg.x + seg.w;
//             if ex <= nx || seg.x >= ne {
//                 new_sky.push(seg.clone());
//                 continue;
//             }
//             if seg.x < nx {
//                 new_sky.push(SkylineSeg { x: seg.x, y: seg.y, w: nx - seg.x });
//             }
//             if ex > ne {
//                 new_sky.push(SkylineSeg { x: ne, y: seg.y, w: ex - ne });
//             }
//         }
//         new_sky.push(SkylineSeg { x: nx, y: new_y, w: nw });
//         new_sky.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

//         // Merge adjacent same-y segments
//         let mut merged: Vec<SkylineSeg> = vec![];
//         for s in new_sky {
//             if let Some(last) = merged.last_mut() {
//                 if (last.y - s.y).abs() < f64::EPSILON && (last.x + last.w - s.x).abs() < f64::EPSILON {
//                     last.w += s.w;
//                     continue;
//                 }
//             }
//             merged.push(s);
//         }
//         self.skyline = merged;
//     }
// }

// // ─────────────────────────────────────────────
// //  NFP Bin (rectangle NFP contact-point)
// // ─────────────────────────────────────────────
// struct NFPBin {
//     bw: f64,
//     bh: f64,
//     kerf: f64,
//     pub placements: Vec<Placement>,
// }

// impl NFPBin {
//     fn new(bw: f64, bh: f64, kerf: f64) -> Self {
//         Self { bw, bh, kerf, placements: vec![] }
//     }

//     fn insert(&mut self, shape: &Shape, allow_rotate: bool) -> Option<Placement> {
//         let mut best: Option<(f64, f64, f64, f64, bool)> = None;
//         let mut best_score = f64::INFINITY;

//         let mut try_orient = |iw: f64, ih: f64, rotated: bool| {
//             if iw > self.bw || ih > self.bh {
//                 return;
//             }
//             let ifp_x2 = self.bw - iw;
//             let ifp_y2 = self.bh - ih;
//             let cands = self.candidates(iw, ih, ifp_x2, ifp_y2);
//             for (cx, cy) in cands {
//                 if cx < 0.0 || cy < 0.0 || cx > ifp_x2 || cy > ifp_y2 {
//                     continue;
//                 }
//                 if self.collides(cx, cy, iw, ih) {
//                     continue;
//                 }
//                 let score = cy * self.bw * 2.0 + cx;
//                 if score < best_score {
//                     best_score = score;
//                     best = Some((cx, cy, iw, ih, rotated));
//                 }
//             }
//         };

//         try_orient(shape.w, shape.h, false);
//         if allow_rotate && (shape.w - shape.h).abs() > f64::EPSILON {
//             try_orient(shape.h, shape.w, true);
//         }

//         let (bx, by, iw, ih, rotated) = best?;
//         let p = Placement {
//             x: bx,
//             y: by,
//             w: iw,
//             h: ih,
//             rotated,
//             name: shape.name.clone(),
//             color_idx: shape.color_idx,
//             shape_id: shape.id,
//         };
//         self.placements.push(p.clone());
//         Some(p)
//     }

//     fn candidates(&self, iw: f64, ih: f64, ifp_x2: f64, ifp_y2: f64) -> Vec<(f64, f64)> {
//         let k = self.kerf;
//         let mut cands = vec![
//             (0.0, 0.0),
//             (ifp_x2, 0.0),
//             (0.0, ifp_y2),
//             (ifp_x2, ifp_y2),
//         ];
//         for pl in &self.placements {
//             let nfp_x1 = pl.x - iw;
//             let nfp_x2 = pl.x + pl.w + k;
//             let nfp_y1 = pl.y - ih;
//             let nfp_y2 = pl.y + pl.h + k;
//             let x_vals = [nfp_x1, nfp_x2, pl.x - iw, pl.x + pl.w];
//             let y_vals = [nfp_y1, nfp_y2, pl.y - ih, pl.y + pl.h];
//             for &x in &x_vals {
//                 for &y in &y_vals {
//                     cands.push((x, y));
//                     cands.push((x.max(0.0).min(ifp_x2), y.max(0.0).min(ifp_y2)));
//                 }
//             }
//             let nfp_x_span = [pl.x - iw, pl.x, pl.x + pl.w + k - iw, pl.x + pl.w + k];
//             let nfp_y_span = [pl.y - ih, pl.y, pl.y + pl.h + k - ih, pl.y + pl.h + k];
//             for &x in &[nfp_x1.max(0.0).min(ifp_x2), nfp_x2.max(0.0).min(ifp_x2)] {
//                 for &y in &nfp_y_span {
//                     cands.push((x, y.max(0.0).min(ifp_y2)));
//                 }
//             }
//             for &y in &[nfp_y1.max(0.0).min(ifp_y2), nfp_y2.max(0.0).min(ifp_y2)] {
//                 for &x in &nfp_x_span {
//                     cands.push((x.max(0.0).min(ifp_x2), y));
//                 }
//             }
//         }
//         cands
//     }

//     fn collides(&self, x: f64, y: f64, iw: f64, ih: f64) -> bool {
//         let k = self.kerf;
//         for pl in &self.placements {
//             if x < pl.x + pl.w + k && x + iw > pl.x && y < pl.y + pl.h + k && y + ih > pl.y {
//                 return true;
//             }
//         }
//         false
//     }
// }

// // ─────────────────────────────────────────────
// //  BinType enum for generic dispatch
// // ─────────────────────────────────────────────
// #[derive(Clone, Copy, PartialEq, Debug)]
// pub enum BinType {
//     MaxRects,
//     Guillotine,
//     BottomLeft,
//     NFP,
// }

// fn pack_single_bin(
//     bin_type: BinType,
//     shapes: &[Shape],
//     bw: f64,
//     bh: f64,
//     kerf: f64,
//     allow_rotate: bool,
// ) -> (Board, Vec<Shape>) {
//     let mut remaining: Vec<Shape> = vec![];
//     let mut board = Board::default();

//     match bin_type {
//         BinType::MaxRects => {
//             let mut bin = MaxRectsBin::new(bw, bh, kerf);
//             for s in shapes {
//                 if bin.insert(s, allow_rotate).is_none() {
//                     remaining.push(s.clone());
//                 }
//             }
//             board.placements = bin.placements;
//         }
//         BinType::Guillotine => {
//             let mut bin = GuillotineBin::new(bw, bh, kerf);
//             for s in shapes {
//                 if bin.insert(s, allow_rotate).is_none() {
//                     remaining.push(s.clone());
//                 }
//             }
//             board.placements = bin.placements;
//         }
//         BinType::BottomLeft => {
//             let mut bin = BottomLeftBin::new(bw, bh, kerf);
//             for s in shapes {
//                 if bin.insert(s, allow_rotate).is_none() {
//                     remaining.push(s.clone());
//                 }
//             }
//             board.placements = bin.placements;
//         }
//         BinType::NFP => {
//             let mut bin = NFPBin::new(bw, bh, kerf);
//             for s in shapes {
//                 if bin.insert(s, allow_rotate).is_none() {
//                     remaining.push(s.clone());
//                 }
//             }
//             board.placements = bin.placements;
//         }
//     }

//     (board, remaining)
// }

// pub fn pack_ordered(
//     bin_type: BinType,
//     items: &[Shape],
//     bw: f64,
//     bh: f64,
//     kerf: f64,
//     allow_rotate: bool,
// ) -> PackResult {
//     let (fittable, unfittable): (Vec<Shape>, Vec<Shape>) = items.iter().cloned().partition(|s| {
//         (s.w <= bw && s.h <= bh) || (allow_rotate && s.h <= bw && s.w <= bh)
//     });

//     let mut boards = vec![];
//     let mut remaining = fittable;

//     while !remaining.is_empty() {
//         let (board, leftover) = pack_single_bin(bin_type, &remaining, bw, bh, kerf, allow_rotate);
//         if leftover.len() == remaining.len() {
//             break;
//         }
//         boards.push(board);
//         remaining = leftover;
//     }

//     PackResult { boards, unfittable }
// }

// pub fn pack_sorted(
//     bin_type: BinType,
//     items: &[Shape],
//     bw: f64,
//     bh: f64,
//     kerf: f64,
//     allow_rotate: bool,
// ) -> PackResult {
//     let mut sorted = items.to_vec();
//     sorted.sort_by(|a, b| (b.w * b.h).partial_cmp(&(a.w * a.h)).unwrap());
//     pack_ordered(bin_type, &sorted, bw, bh, kerf, allow_rotate)
// }

// // ─────────────────────────────────────────────
// //  Fitness function (lower = better)
// // ─────────────────────────────────────────────
// pub fn fitness(boards: &[Board], bw: f64, bh: f64) -> f64 {
//     if boards.is_empty() {
//         return 1e9;
//     }
//     let board_area = bw * bh;
//     let used: f64 = boards
//         .iter()
//         .flat_map(|b| b.placements.iter())
//         .map(|p| p.w * p.h)
//         .sum();
//     let waste_ratio = 1.0 - used / (boards.len() as f64 * board_area);
//     boards.len() as f64 + waste_ratio
// }

// // ─────────────────────────────────────────────
// //  Genetic Algorithm
// // ─────────────────────────────────────────────
// pub fn pack_ga(
//     items: &[Shape],
//     bw: f64,
//     bh: f64,
//     kerf: f64,
//     allow_rotate: bool,
//     pop_size: usize,
//     generations: usize,
//     progress_cb: &mut dyn FnMut(f64, f64, &[f64]),
// ) -> (PackResult, Vec<f64>) {
//     let n = items.len();
//     if n == 0 {
//         return (PackResult { boards: vec![], unfittable: vec![] }, vec![]);
//     }
//     let mut rng = thread_rng();

//     let eval_perm = |perm: &[usize]| -> PackResult {
//         let ordered: Vec<Shape> = perm.iter().map(|&i| items[i].clone()).collect();
//         pack_ordered(BinType::MaxRects, &ordered, bw, bh, kerf, allow_rotate)
//     };

//     let area_sorted: Vec<usize> = {
//         let mut v: Vec<usize> = (0..n).collect();
//         v.sort_by(|&a, &b| (items[b].w * items[b].h).partial_cmp(&(items[a].w * items[a].h)).unwrap());
//         v
//     };

//     let rand_perm = |rng: &mut ThreadRng| -> Vec<usize> {
//         let mut v: Vec<usize> = (0..n).collect();
//         v.shuffle(rng);
//         v
//     };

//     // OX crossover
//     let ox = |p1: &[usize], p2: &[usize], rng: &mut ThreadRng| -> Vec<usize> {
//         if n <= 1 {
//             return p1.to_vec();
//         }
//         let a = rng.gen_range(0..n);
//         let len = 1 + rng.gen_range(0..n);
//         let seg_set: std::collections::HashSet<usize> =
//             (0..len).map(|k| p1[(a + k) % n]).collect();
//         let mut child = vec![usize::MAX; n];
//         for k in 0..len {
//             child[(a + k) % n] = p1[(a + k) % n];
//         }
//         let remaining: Vec<usize> = p2.iter().copied().filter(|v| !seg_set.contains(v)).collect();
//         let mut ri = 0;
//         for k in 0..n {
//             let pos = (a + len + k) % n;
//             if child[pos] == usize::MAX {
//                 child[pos] = remaining[ri];
//                 ri += 1;
//             }
//         }
//         child
//     };

//     let mutate = |perm: &[usize], rng: &mut ThreadRng| -> Vec<usize> {
//         let mut p = perm.to_vec();
//         let nm = 1 + rng.gen_range(0..2_usize);
//         for _ in 0..nm {
//             let a = rng.gen_range(0..n);
//             let b = rng.gen_range(0..n);
//             p.swap(a, b);
//         }
//         p
//     };

//     let pop: Vec<Vec<usize>> = std::iter::once(area_sorted.clone())
//         .chain((0..pop_size - 1).map(|_| rand_perm(&mut rng)))
//         .collect();

//     let mut evaluated: Vec<(Vec<usize>, PackResult, f64)> = pop
//         .iter()
//         .map(|perm| {
//             let res = eval_perm(perm);
//             let f = fitness(&res.boards, bw, bh);
//             (perm.clone(), res, f)
//         })
//         .collect();
//     evaluated.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

//     let mut best_fit = evaluated[0].2;
//     let mut best_res = evaluated[0].1.clone();
//     let mut history = vec![best_fit];

//     for g in 0..generations {
//         if g % 5 == 0 {
//             progress_cb(g as f64 / generations as f64, best_fit, &history);
//         }

//         let mut next_pop: Vec<Vec<usize>> = vec![evaluated[0].0.clone()];
//         if evaluated.len() > 1 {
//             next_pop.push(evaluated[1].0.clone());
//         }
//         while next_pop.len() < pop_size {
//             let p1 = &evaluated[rng.gen_range(0..3.min(evaluated.len()))].0;
//             let p2 = &evaluated[rng.gen_range(0..evaluated.len())].0;
//             let mut child = ox(p1, p2, &mut rng);
//             if rng.r#gen::<f64>() < 0.15 {
//                 child = mutate(&child, &mut rng);
//             }
//             next_pop.push(child);
//         }

//         evaluated = next_pop
//             .iter()
//             .map(|perm| {
//                 let res = eval_perm(perm);
//                 let f = fitness(&res.boards, bw, bh);
//                 (perm.clone(), res, f)
//             })
//             .collect();
//         evaluated.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

//         if evaluated[0].2 < best_fit {
//             best_fit = evaluated[0].2;
//             best_res = evaluated[0].1.clone();
//         }
//         history.push(best_fit);
//     }

//     (best_res, history)
// }

// // ─────────────────────────────────────────────
// //  Simulated Annealing
// // ─────────────────────────────────────────────
// pub fn pack_sa(
//     items: &[Shape],
//     bw: f64,
//     bh: f64,
//     kerf: f64,
//     allow_rotate: bool,
//     t0: f64,
//     max_steps: usize,
//     progress_cb: &mut dyn FnMut(f64, f64, &[f64]),
// ) -> (PackResult, Vec<f64>) {
//     let n = items.len();
//     if n == 0 {
//         return (PackResult { boards: vec![], unfittable: vec![] }, vec![]);
//     }
//     let mut rng = thread_rng();

//     let eval_perm = |perm: &[usize]| -> PackResult {
//         let ordered: Vec<Shape> = perm.iter().map(|&i| items[i].clone()).collect();
//         pack_ordered(BinType::MaxRects, &ordered, bw, bh, kerf, allow_rotate)
//     };

//     let mut perm: Vec<usize> = {
//         let mut v: Vec<usize> = (0..n).collect();
//         v.sort_by(|&a, &b| (items[b].w * items[b].h).partial_cmp(&(items[a].w * items[a].h)).unwrap());
//         v
//     };

//     let mut cur_res = eval_perm(&perm);
//     let mut cur_fit = fitness(&cur_res.boards, bw, bh);
//     let mut best_fit = cur_fit;
//     let mut best_res = cur_res.clone();
//     let mut history = vec![best_fit];

//     let alpha = 0.001_f64.powf(1.0 / max_steps as f64);
//     let mut t = t0;

//     for i in 0..max_steps {
//         if i % 50 == 0 {
//             progress_cb(i as f64 / max_steps as f64, best_fit, &history);
//         }

//         let mut new_perm = perm.clone();
//         if rng.r#gen::<f64>() < 0.7 {
//             let a = rng.gen_range(0..n);
//             let b = rng.gen_range(0..n);
//             new_perm.swap(a, b);
//         } else {
//             let a = rng.gen_range(0..n);
//             let b = rng.gen_range(0..n);
//             let lo = a.min(b);
//             let hi = a.max(b);
//             new_perm[lo..=hi].reverse();
//         }

//         let new_res = eval_perm(&new_perm);
//         let new_fit = fitness(&new_res.boards, bw, bh);
//         let delta = new_fit - cur_fit;

//         if delta < 0.0 || rng.r#gen::<f64>() < (-delta / t.max(1e-10)).exp() {
//             perm = new_perm;
//             cur_res = new_res;
//             cur_fit = new_fit;
//             if cur_fit < best_fit {
//                 best_fit = cur_fit;
//                 best_res = cur_res.clone();
//             }
//         }

//         t *= alpha;
//         if i % 50 == 0 {
//             history.push(best_fit);
//         }
//     }

//     (best_res, history)
// }

// // ─────────────────────────────────────────────
// //  SVGNest
// // ─────────────────────────────────────────────
// fn svgnest_fitness(boards: &[Board], bw: f64, bh: f64) -> f64 {
//     if boards.is_empty() {
//         return 1e9;
//     }
//     let board_area = bw * bh;
//     let mut gravity = 0.0_f64;
//     let mut count = 0usize;
//     for b in boards {
//         for p in &b.placements {
//             gravity += (p.y + p.h / 2.0) / bh;
//             count += 1;
//         }
//     }
//     let grav_score = if count > 0 { gravity / count as f64 } else { 0.0 };
//     let used: f64 = boards.iter().flat_map(|b| b.placements.iter()).map(|p| p.w * p.h).sum();
//     let waste_ratio = 1.0 - used / (boards.len() as f64 * board_area);
//     boards.len() as f64 + waste_ratio * 0.7 + grav_score * 0.3
// }

// pub fn pack_svgnest(
//     items: &[Shape],
//     bw: f64,
//     bh: f64,
//     kerf: f64,
//     allow_rotate: bool,
//     pop_size: usize,
//     generations: usize,
//     progress_cb: &mut dyn FnMut(f64, f64, &[f64]),
// ) -> (PackResult, Vec<f64>) {
//     let n = items.len();
//     if n == 0 {
//         return (PackResult { boards: vec![], unfittable: vec![] }, vec![]);
//     }
//     let mut rng = thread_rng();

//     let eval_perm = |perm: &[usize]| -> PackResult {
//         let ordered: Vec<Shape> = perm.iter().map(|&i| items[i].clone()).collect();
//         pack_ordered(BinType::NFP, &ordered, bw, bh, kerf, allow_rotate)
//     };

//     let area_sorted: Vec<usize> = {
//         let mut v: Vec<usize> = (0..n).collect();
//         v.sort_by(|&a, &b| (items[b].w * items[b].h).partial_cmp(&(items[a].w * items[a].h)).unwrap());
//         v
//     };

//     let rand_perm = |rng: &mut ThreadRng| -> Vec<usize> {
//         let mut v: Vec<usize> = (0..n).collect();
//         v.shuffle(rng);
//         v
//     };

//     let ox = |p1: &[usize], p2: &[usize], rng: &mut ThreadRng| -> Vec<usize> {
//         if n <= 1 {
//             return p1.to_vec();
//         }
//         let a = rng.gen_range(0..n);
//         let len = 1 + rng.gen_range(0..n);
//         let seg_set: std::collections::HashSet<usize> =
//             (0..len).map(|k| p1[(a + k) % n]).collect();
//         let mut child = vec![usize::MAX; n];
//         for k in 0..len {
//             child[(a + k) % n] = p1[(a + k) % n];
//         }
//         let remaining: Vec<usize> = p2.iter().copied().filter(|v| !seg_set.contains(v)).collect();
//         let mut ri = 0;
//         for k in 0..n {
//             let pos = (a + len + k) % n;
//             if child[pos] == usize::MAX {
//                 child[pos] = remaining[ri];
//                 ri += 1;
//             }
//         }
//         child
//     };

//     let mutate_svgnest = |perm: &[usize], rng: &mut ThreadRng| -> Vec<usize> {
//         let mut p = perm.to_vec();
//         if rng.r#gen::<f64>() < 0.5 {
//             let a = rng.gen_range(0..n);
//             let b = rng.gen_range(0..n);
//             p.swap(a, b);
//         } else {
//             let len = 1 + rng.gen_range(0..3.min(n - 1).max(1));
//             let from = rng.gen_range(0..n.saturating_sub(len).max(1));
//             let to = rng.gen_range(0..n.saturating_sub(len).max(1));
//             let seg: Vec<usize> = p.drain(from..from + len.min(p.len() - from)).collect();
//             let ins = to.min(p.len());
//             for (i, v) in seg.into_iter().enumerate() {
//                 p.insert(ins + i, v);
//             }
//         }
//         p
//     };

//     let pop: Vec<Vec<usize>> = std::iter::once(area_sorted.clone())
//         .chain((0..pop_size - 1).map(|_| rand_perm(&mut rng)))
//         .collect();

//     let mut evaluated: Vec<(Vec<usize>, PackResult, f64)> = pop
//         .iter()
//         .map(|perm| {
//             let res = eval_perm(perm);
//             let f = svgnest_fitness(&res.boards, bw, bh);
//             (perm.clone(), res, f)
//         })
//         .collect();
//     evaluated.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

//     let mut best_fit = evaluated[0].2;
//     let mut best_res = evaluated[0].1.clone();
//     let mut history = vec![best_fit];

//     let elite_count = (pop_size as f64 * 0.3).max(1.0) as usize;

//     for g in 0..generations {
//         if g % 3 == 0 {
//             progress_cb(g as f64 / generations as f64, best_fit, &history);
//         }

//         let mut next_pop: Vec<Vec<usize>> = vec![evaluated[0].0.clone()];
//         if evaluated.len() > 1 {
//             next_pop.push(evaluated[1].0.clone());
//         }
//         while next_pop.len() < pop_size {
//             let p1 = &evaluated[rng.gen_range(0..elite_count.min(evaluated.len()))].0;
//             let p2 = &evaluated[rng.gen_range(0..evaluated.len())].0;
//             let mut child = ox(p1, p2, &mut rng);
//             if rng.r#gen::<f64>() < 0.2 {
//                 child = mutate_svgnest(&child, &mut rng);
//             }
//             next_pop.push(child);
//         }

//         evaluated = next_pop
//             .iter()
//             .map(|perm| {
//                 let res = eval_perm(perm);
//                 let f = svgnest_fitness(&res.boards, bw, bh);
//                 (perm.clone(), res, f)
//             })
//             .collect();
//         evaluated.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

//         if evaluated[0].2 < best_fit {
//             best_fit = evaluated[0].2;
//             best_res = evaluated[0].1.clone();
//         }
//         history.push(best_fit);
//     }

//     (best_res, history)
// }
