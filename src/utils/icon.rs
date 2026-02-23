pub fn load_icon() -> eframe::egui::IconData {
    let size = 32usize;
    let mut pixels = Vec::with_capacity(size * size * 4);
    for y in 0..size {
        for x in 0..size {
            let in_circle = {
                let cx = x as f32 - 16.0;
                let cy = y as f32 - 16.0;
                cx * cx + cy * cy <= 14.0 * 14.0
            };
            if in_circle {
                pixels.extend_from_slice(&[212, 168, 83, 255]);
            } else {
                pixels.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }
    eframe::egui::IconData {
        rgba: pixels,
        width: size as u32,
        height: size as u32,
    }
}
