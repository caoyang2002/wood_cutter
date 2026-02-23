use eframe::egui::{self, Color32, Pos2, Rect, RichText, Rounding, Stroke, Vec2};
use crate::ui::theme::{ACC, BD, OK, SF, TX, TX2};

/// Header height — tall enough for two-line title without clipping
pub const HEADER_HEIGHT: f32 = 52.0;

/// Data for a single stat cell
pub struct StatCell<'a> {
    pub label: &'static str,
    pub value: &'a str,
    /// true = value is a "real" result (use accent colour), false = placeholder "—"
    pub active: bool,
}

impl<'a> StatCell<'a> {
    pub fn new(label: &'static str, value: &'a str) -> Self {
        Self { label, value, active: value != "—" }
    }
}

/// Draw the entire top header bar.
///
/// # Layout (left → right)
/// ```
/// [  LOGO BLOCK (fixed 160px)  ] [ ··· flex gap ··· ] [ STAT×5 (each fixed 80px) ]
/// ```
///
/// * The logo block holds the app title on line-1 and the algo tagline on line-2.
///   Both are clipped to the block — no overlap with stats.
/// * Each stat cell is allocated exactly `STAT_W` pixels so the layout never
///   shifts when values change from "—" to real numbers.
pub fn draw_header(
    ctx: &egui::Context,
    boards: &str,
    util: &str,
    shapes: &str,
    waste: &str,
    time: &str,
) {
    egui::TopBottomPanel::top("header")
        .exact_height(HEADER_HEIGHT)
        .frame(egui::Frame {
            fill: SF,
            inner_margin: egui::Margin::symmetric(16.0, 0.0),
            stroke: Stroke::new(1.0, BD),
            ..Default::default()
        })
        .show(ctx, |ui| {
            // Total available width for content
            let avail_w = ui.available_width();

            // Fixed geometry constants
            const LOGO_W: f32 = 170.0;   // left title block
            const STAT_W: f32 = 78.0;    // each stat cell (fixed, never changes)
            const N_STATS: f32 = 5.0;
            const SEP_W: f32 = 1.0;      // separator width
            const GAP: f32 = 6.0;        // gap between stat cells

            let stats_total = N_STATS * STAT_W + (N_STATS - 1.0) * (SEP_W + GAP * 2.0);
            let flex_gap = (avail_w - LOGO_W - stats_total).max(8.0);

            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                // ── Logo block ──────────────────────────────────────────
                let (logo_rect, _) = ui.allocate_exact_size(
                    Vec2::new(LOGO_W, HEADER_HEIGHT),
                    egui::Sense::hover(),
                );
                // Title line
                ui.painter().text(
                    Pos2::new(logo_rect.min.x, logo_rect.center().y - 10.0),
                    egui::Align2::LEFT_CENTER,
                    "木板分割优化",
                    egui::FontId::proportional(16.0),
                    ACC,
                );
                // Tagline line (smaller, muted)
                ui.painter().text(
                    Pos2::new(logo_rect.min.x, logo_rect.center().y + 10.0),
                    egui::Align2::LEFT_CENTER,
                    "NFP · GA · SA · SVGNest · MaxRects",
                    egui::FontId::monospace(8.0),
                    TX2,
                );

                // ── Flex gap ─────────────────────────────────────────────
                ui.add_space(flex_gap);

                // ── Stat cells ───────────────────────────────────────────
                let stat_data: &[(&'static str, &str)] = &[
                    ("木板数", boards),
                    ("利用率", util),
                    ("图形数", shapes),
                    ("废料m²", waste),
                    ("耗时ms", time),
                ];

                for (i, (label, value)) in stat_data.iter().enumerate() {
                    // Thin vertical separator before each cell (except first)
                    if i > 0 {
                        ui.add_space(GAP);
                        let sep_top = ui.cursor().min.y + 10.0;
                        let sep_bot = sep_top + HEADER_HEIGHT - 20.0;
                        let sep_x = ui.cursor().min.x;
                        ui.painter().line_segment(
                            [Pos2::new(sep_x, sep_top), Pos2::new(sep_x, sep_bot)],
                            Stroke::new(1.0, BD),
                        );
                        ui.add_space(GAP);
                    }

                    // Allocate a fixed-width cell — value changes never shift layout
                    let (cell_rect, _) = ui.allocate_exact_size(
                        Vec2::new(STAT_W, HEADER_HEIGHT),
                        egui::Sense::hover(),
                    );

                    let is_active = *value != "—";
                    let val_color = if is_active { ACC } else { TX2 };

                    // Value (larger, monospace, vertically centered slightly above)
                    ui.painter().text(
                        Pos2::new(cell_rect.center().x, cell_rect.center().y - 9.0),
                        egui::Align2::CENTER_CENTER,
                        value,
                        egui::FontId::monospace(13.0),
                        val_color,
                    );

                    // Label (smaller, always muted)
                    ui.painter().text(
                        Pos2::new(cell_rect.center().x, cell_rect.center().y + 9.0),
                        egui::Align2::CENTER_CENTER,
                        label,
                        egui::FontId::monospace(8.0),
                        TX2,
                    );

                    // Subtle underline when active
                    if is_active {
                        let uy = cell_rect.max.y - 3.0;
                        let uw = STAT_W * 0.5;
                        let ux = cell_rect.center().x - uw / 2.0;
                        ui.painter().line_segment(
                            [Pos2::new(ux, uy), Pos2::new(ux + uw, uy)],
                            Stroke::new(1.5, Color32::from_rgba_unmultiplied(
                                ACC.r(), ACC.g(), ACC.b(), 120,
                            )),
                        );
                    }
                }
            });
        });
}
