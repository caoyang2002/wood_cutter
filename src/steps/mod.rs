pub mod generator;

pub use generator::{generate_steps, mk_snap};

#[derive(Clone, Debug, PartialEq)]
pub enum StepType {
    Open,
    Place,
    Done,
}

#[derive(Clone, Debug)]
pub struct Step {
    pub step_type: StepType,
    pub board_idx: usize,
    pub msg: String,
    /// snap[bi] = how many placements to show on board bi at this step
    pub snap: Vec<usize>,
    /// Highlighted rect: (board_idx, x, y, w, h)
    pub highlight: Option<(usize, f64, f64, f64, f64)>,
}
