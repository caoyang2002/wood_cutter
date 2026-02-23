pub mod bottom_left;
pub mod ga;
pub mod guillotine;
pub mod maxrects;
pub mod nfp_greedy;
pub mod sa;
pub mod svgnest;

pub use ga::pack_ga;
pub use sa::pack_sa;
pub use svgnest::pack_svgnest;

use crate::core::{Board, Shape};

// ─────────────────────────────────────────────
//  PackResult
// ─────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct PackResult {
    pub boards: Vec<Board>,
    pub unfittable: Vec<Shape>,
}

// ─────────────────────────────────────────────
//  BinType enum for generic dispatch
// ─────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BinType {
    MaxRects,
    Guillotine,
    BottomLeft,
    NFP,
}

// ─────────────────────────────────────────────
//  Generic single-board packer
// ─────────────────────────────────────────────
pub fn pack_single_bin(
    bin_type: BinType,
    shapes: &[Shape],
    bw: f64,
    bh: f64,
    kerf: f64,
    allow_rotate: bool,
) -> (Board, Vec<Shape>) {
    match bin_type {
        BinType::MaxRects => maxrects::pack_maxrects(shapes, bw, bh, kerf, allow_rotate),
        BinType::Guillotine => guillotine::pack_guillotine(shapes, bw, bh, kerf, allow_rotate),
        BinType::BottomLeft => bottom_left::pack_bottom_left(shapes, bw, bh, kerf, allow_rotate),
        BinType::NFP => nfp_greedy::pack_nfp(shapes, bw, bh, kerf, allow_rotate),
    }
}

// ─────────────────────────────────────────────
//  Multi-board packing loop
// ─────────────────────────────────────────────
pub fn pack_ordered(
    bin_type: BinType,
    items: &[Shape],
    bw: f64,
    bh: f64,
    kerf: f64,
    allow_rotate: bool,
) -> PackResult {
    let (fittable, unfittable): (Vec<Shape>, Vec<Shape>) =
        items.iter().cloned().partition(|s| {
            (s.w <= bw && s.h <= bh) || (allow_rotate && s.h <= bw && s.w <= bh)
        });

    let mut boards = vec![];
    let mut remaining = fittable;

    while !remaining.is_empty() {
        let (board, leftover) =
            pack_single_bin(bin_type, &remaining, bw, bh, kerf, allow_rotate);
        if leftover.len() == remaining.len() {
            break;
        }
        boards.push(board);
        remaining = leftover;
    }

    PackResult { boards, unfittable }
}

pub fn pack_sorted(
    bin_type: BinType,
    items: &[Shape],
    bw: f64,
    bh: f64,
    kerf: f64,
    allow_rotate: bool,
) -> PackResult {
    let mut sorted = items.to_vec();
    sorted.sort_by(|a, b| (b.w * b.h).partial_cmp(&(a.w * a.h)).unwrap());
    pack_ordered(bin_type, &sorted, bw, bh, kerf, allow_rotate)
}

// ─────────────────────────────────────────────
//  Fitness function (lower = better)
// ─────────────────────────────────────────────
pub fn fitness(boards: &[Board], bw: f64, bh: f64) -> f64 {
    if boards.is_empty() {
        return 1e9;
    }
    let board_area = bw * bh;
    let used: f64 = boards
        .iter()
        .flat_map(|b| b.placements.iter())
        .map(|p| p.w * p.h)
        .sum();
    let waste_ratio = 1.0 - used / (boards.len() as f64 * board_area);
    boards.len() as f64 + waste_ratio
}
