mod algorithms;
mod app;
mod core;
mod steps;
mod types;
mod ui;
mod utils;

use app::WoodCutterApp;
use utils::load_icon;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("木板分割优化 Pro")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([900.0, 600.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "木板分割优化",
        native_options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(WoodCutterApp::default()))
        }),
    )
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "noto_cjk".into(),
        egui::FontData::from_static(include_bytes!("../assets/SourceHanSerifCN-Regular-1.otf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_insert_with(Vec::new)
        .insert(0, "noto_cjk".into());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_insert_with(Vec::new)
        .insert(0, "noto_cjk".into());
    ctx.set_fonts(fonts);
}
