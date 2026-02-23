use eframe::egui::Color32;

pub const PALETTE: &[Color32] = &[
    Color32::from_rgb(231, 76, 60),
    Color32::from_rgb(230, 126, 34),
    Color32::from_rgb(241, 196, 15),
    Color32::from_rgb(46, 204, 113),
    Color32::from_rgb(26, 188, 156),
    Color32::from_rgb(52, 152, 219),
    Color32::from_rgb(155, 89, 182),
    Color32::from_rgb(233, 30, 99),
    Color32::from_rgb(255, 87, 34),
    Color32::from_rgb(0, 188, 212),
    Color32::from_rgb(139, 195, 74),
    Color32::from_rgb(255, 152, 0),
    Color32::from_rgb(103, 58, 183),
    Color32::from_rgb(0, 150, 136),
    Color32::from_rgb(240, 98, 146),
    Color32::from_rgb(66, 165, 245),
    Color32::from_rgb(102, 187, 106),
    Color32::from_rgb(255, 167, 38),
    Color32::from_rgb(171, 71, 188),
    Color32::from_rgb(38, 198, 218),
    Color32::from_rgb(239, 154, 154),
    Color32::from_rgb(255, 204, 128),
    Color32::from_rgb(197, 225, 165),
    Color32::from_rgb(128, 222, 234),
];

pub fn gc(idx: usize) -> Color32 {
    PALETTE[idx % PALETTE.len()]
}

// Dark theme colors
pub const BG: Color32 = Color32::from_rgb(12, 12, 12);
pub const SF: Color32 = Color32::from_rgb(21, 21, 21);
pub const SF2: Color32 = Color32::from_rgb(28, 28, 28);
pub const BD: Color32 = Color32::from_rgb(42, 42, 42);
pub const ACC: Color32 = Color32::from_rgb(212, 168, 83);
pub const TX: Color32 = Color32::from_rgb(224, 216, 204);
pub const TX2: Color32 = Color32::from_rgb(122, 106, 88);
pub const ERR: Color32 = Color32::from_rgb(192, 57, 43);
pub const OK: Color32 = Color32::from_rgb(39, 174, 96);
pub const INFO: Color32 = Color32::from_rgb(41, 128, 185);
