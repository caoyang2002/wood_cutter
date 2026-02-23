use crate::core::Board;
use super::{Step, StepType};

pub fn generate_steps(boards: &[Board], bw: f64, bh: f64, algo_label: &str) -> Vec<Step> {
    let mut out = vec![];
    let total: usize = boards.iter().map(|b| b.placements.len()).sum();
    let mut placed = 0usize;

    for (bi, board) in boards.iter().enumerate() {
        let open_snap = mk_snap(boards, bi, 0);
        out.push(Step {
            step_type: StepType::Open,
            board_idx: bi,
            msg: format!(
                "📋 开启第 {} 块木板  [{}×{}mm]  算法: {}\n{}",
                bi + 1,
                bw,
                bh,
                algo_label,
                if bi > 0 {
                    format!("前一块空间不足，自动添加第 {} 块", bi + 1)
                } else {
                    "开始排列第一块木板".to_string()
                }
            ),
            snap: open_snap,
            highlight: None,
        });

        for (pi, p) in board.placements.iter().enumerate() {
            placed += 1;
            let used: f64 = board.placements[..=pi].iter().map(|x| x.w * x.h).sum();
            let util = used / (bw * bh) * 100.0;
            out.push(Step {
                step_type: StepType::Place,
                board_idx: bi,
                msg: format!(
                    "✂️  放置: {}\n   位置: ({}, {}) mm\n   尺寸: {}×{}mm{}\n   木板利用率: {:.1}%   总进度: {}/{}",
                    p.name,
                    p.x.round(),
                    p.y.round(),
                    p.w,
                    p.h,
                    if p.rotated { " [旋转90°]" } else { "" },
                    util,
                    placed,
                    total
                ),
                snap: mk_snap(boards, bi, pi + 1),
                highlight: Some((bi, p.x, p.y, p.w, p.h)),
            });
        }

        let used: f64 = board.placements.iter().map(|p| p.w * p.h).sum();
        let util = used / (bw * bh) * 100.0;
        let done_snap = mk_snap(boards, bi, board.placements.len());
        out.push(Step {
            step_type: StepType::Done,
            board_idx: bi,
            msg: format!(
                "✅ 第 {} 块完成  利用率: {:.1}%  废料: {:.4}m²\n{}",
                bi + 1,
                util,
                (bw * bh - used) / 1e6,
                if bi < boards.len() - 1 {
                    format!("→ 仍有图形未完，自动开启第 {} 块", bi + 2)
                } else {
                    format!("→ 全部 {} 个图形切割完成！", total)
                }
            ),
            snap: done_snap,
            highlight: None,
        });
    }
    out
}

pub fn mk_snap(boards: &[Board], up_to_bi: usize, up_to_pi: usize) -> Vec<usize> {
    boards
        .iter()
        .enumerate()
        .map(|(bi, b)| {
            if bi < up_to_bi {
                b.placements.len()
            } else if bi == up_to_bi {
                up_to_pi
            } else {
                0
            }
        })
        .collect()
}
