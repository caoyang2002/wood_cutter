use rand::prelude::*;
use crate::core::Shape;
use crate::algorithms::{fitness, pack_ordered, BinType, PackResult};

pub fn pack_sa(
    items: &[Shape],
    bw: f64,
    bh: f64,
    kerf: f64,
    allow_rotate: bool,
    t0: f64,
    max_steps: usize,
    progress_cb: &mut dyn FnMut(f64, f64, &[f64]),
) -> (PackResult, Vec<f64>) {
    let n = items.len();
    if n == 0 {
        return (PackResult { boards: vec![], unfittable: vec![] }, vec![]);
    }
    let mut rng = thread_rng();

    let eval_perm = |perm: &[usize]| -> PackResult {
        let ordered: Vec<Shape> = perm.iter().map(|&i| items[i].clone()).collect();
        pack_ordered(BinType::MaxRects, &ordered, bw, bh, kerf, allow_rotate)
    };

    let mut perm: Vec<usize> = {
        let mut v: Vec<usize> = (0..n).collect();
        v.sort_by(|&a, &b| {
            (items[b].w * items[b].h).partial_cmp(&(items[a].w * items[a].h)).unwrap()
        });
        v
    };

    let mut cur_res = eval_perm(&perm);
    let mut cur_fit = fitness(&cur_res.boards, bw, bh);
    let mut best_fit = cur_fit;
    let mut best_res = cur_res.clone();
    let mut history = vec![best_fit];

    let alpha = 0.001_f64.powf(1.0 / max_steps as f64);
    let mut t = t0;

    for i in 0..max_steps {
        if i % 50 == 0 {
            progress_cb(i as f64 / max_steps as f64, best_fit, &history);
        }

        let mut new_perm = perm.clone();
        if rng.r#gen::<f64>() < 0.7 {
            let a = rng.gen_range(0..n);
            let b = rng.gen_range(0..n);
            new_perm.swap(a, b);
        } else {
            let a = rng.gen_range(0..n);
            let b = rng.gen_range(0..n);
            let lo = a.min(b);
            let hi = a.max(b);
            new_perm[lo..=hi].reverse();
        }

        let new_res = eval_perm(&new_perm);
        let new_fit = fitness(&new_res.boards, bw, bh);
        let delta = new_fit - cur_fit;

        if delta < 0.0 || rng.r#gen::<f64>() < (-delta / t.max(1e-10)).exp() {
            perm = new_perm;
            cur_res = new_res;
            cur_fit = new_fit;
            if cur_fit < best_fit {
                best_fit = cur_fit;
                best_res = cur_res.clone();
            }
        }

        t *= alpha;
        if i % 50 == 0 {
            history.push(best_fit);
        }
    }

    (best_res, history)
}
