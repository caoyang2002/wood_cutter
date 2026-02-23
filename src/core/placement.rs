#[derive(Clone, Debug)]
pub struct Placement {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub rotated: bool,
    pub name: String,
    pub color_idx: usize,
    pub shape_id: usize,
}
