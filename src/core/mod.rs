pub mod board;
pub mod placement;
pub mod shape;

pub use board::Board;
pub use placement::Placement;
pub use shape::Shape;

/// Internal rect used by bin algorithms
#[derive(Clone, Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

pub fn overlaps(a: &Rect, b: &Rect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

pub fn contains(outer: &Rect, inner: &Rect) -> bool {
    outer.x <= inner.x
        && outer.y <= inner.y
        && outer.x + outer.w >= inner.x + inner.w
        && outer.y + outer.h >= inner.y + inner.h
}
