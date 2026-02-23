use crate::core::Shape;

#[derive(Clone, Debug)]
pub struct ShapeEntry {
    pub id: usize,
    pub name: String,
    pub w: String,
    pub h: String,
}

impl ShapeEntry {
    pub fn to_shape(&self, color_idx: usize) -> Option<Shape> {
        let w = self.w.parse::<f64>().ok()?;
        let h = self.h.parse::<f64>().ok()?;
        if w <= 0.0 || h <= 0.0 {
            return None;
        }
        Some(Shape { id: self.id, name: self.name.clone(), w, h, color_idx })
    }
}
