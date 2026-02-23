use rand::prelude::*;
use crate::core::Shape;
use crate::algorithms::{fitness, pack_ordered, BinType, PackResult};

pub fn pack_ga(
    items: &[Shape],
    bw: f64,
    bh: f64,
    kerf: f64,
    allow_rotate: bool,
    pop_size: usize,
    generations: usize,
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

    let area_sorted: Vec<usize> = {
        let mut v: Vec<usize> = (0..n).collect();
        v.sort_by(|&a, &b| {
            (items[b].w * items[b].h).partial_cmp(&(items[a].w * items[a].h)).unwrap()
        });
        v
    };

    let rand_perm = |rng: &mut ThreadRng| -> Vec<usize> {
        let mut v: Vec<usize> = (0..n).collect();
        v.shuffle(rng);
        v
    };

    let ox = |p1: &[usize], p2: &[usize], rng: &mut ThreadRng| -> Vec<usize> {
        if n <= 1 {
            return p1.to_vec();
        }
        let a = rng.gen_range(0..n);
        let len = 1 + rng.gen_range(0..n);
        let seg_set: std::collections::HashSet<usize> =
            (0..len).map(|k| p1[(a + k) % n]).collect();
        let mut child = vec![usize::MAX; n];
        for k in 0..len {
            child[(a + k) % n] = p1[(a + k) % n];
        }
        let remaining: Vec<usize> =
            p2.iter().copied().filter(|v| !seg_set.contains(v)).collect();
        let mut ri = 0;
        for k in 0..n {
            let pos = (a + len + k) % n;
            if child[pos] == usize::MAX {
                child[pos] = remaining[ri];
                ri += 1;
            }
        }
        child
    };

    let mutate = |perm: &[usize], rng: &mut ThreadRng| -> Vec<usize> {
        let mut p = perm.to_vec();
        let nm = 1 + rng.gen_range(0..2_usize);
        for _ in 0..nm {
            let a = rng.gen_range(0..n);
            let b = rng.gen_range(0..n);
            p.swap(a, b);
        }
        p
    };

    let pop: Vec<Vec<usize>> = std::iter::once(area_sorted.clone())
        .chain((0..pop_size - 1).map(|_| rand_perm(&mut rng)))
        .collect();

    let mut evaluated: Vec<(Vec<usize>, PackResult, f64)> = pop
        .iter()
        .map(|perm| {
            let res = eval_perm(perm);
            let f = fitness(&res.boards, bw, bh);
            (perm.clone(), res, f)
        })
        .collect();
    evaluated.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut best_fit = evaluated[0].2;
    let mut best_res = evaluated[0].1.clone();
    let mut history = vec![best_fit];

    for g in 0..generations {
        if g % 5 == 0 {
            progress_cb(g as f64 / generations as f64, best_fit, &history);
        }

        let mut next_pop: Vec<Vec<usize>> = vec![evaluated[0].0.clone()];
        if evaluated.len() > 1 {
            next_pop.push(evaluated[1].0.clone());
        }
        while next_pop.len() < pop_size {
            let p1 = &evaluated[rng.gen_range(0..3.min(evaluated.len()))].0;
            let p2 = &evaluated[rng.gen_range(0..evaluated.len())].0;
            let mut child = ox(p1, p2, &mut rng);
            if rng.r#gen::<f64>() < 0.15 {
                child = mutate(&child, &mut rng);
            }
            next_pop.push(child);
        }

        evaluated = next_pop
            .iter()
            .map(|perm| {
                let res = eval_perm(perm);
                let f = fitness(&res.boards, bw, bh);
                (perm.clone(), res, f)
            })
            .collect();
        evaluated.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

        if evaluated[0].2 < best_fit {
            best_fit = evaluated[0].2;
            best_res = evaluated[0].1.clone();
        }
        history.push(best_fit);
    }

    (best_res, history)
}
