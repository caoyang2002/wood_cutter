use crate::core::placement::Placement;

#[derive(Clone, Debug, Default)]
pub struct Board {
    pub placements: Vec<Placement>,
}
