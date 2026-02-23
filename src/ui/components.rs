use eframe::egui::{self, Color32, FontId, RichText, Stroke};
use crate::ui::theme::{ACC, BD, ERR, TX2};

pub fn section_header(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).size(8.0).color(ACC).monospace().strong());
    ui.add(egui::Separator::default().spacing(5.0));
    ui.add_space(3.0);
}

pub fn field_row(ui: &mut egui::Ui, label: &str, value: &mut String) {
    ui.label(RichText::new(label).size(9.0).color(TX2).monospace());
    ui.add(egui::TextEdit::singleline(value).desired_width(80.0).font(FontId::monospace(11.0)));
}

pub fn accent_btn(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(label).monospace().size(9.0).color(Color32::BLACK))
            .fill(ACC),
    )
}

pub fn secondary_btn(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(label).monospace().size(9.0).color(TX2))
            .fill(Color32::TRANSPARENT)
            .stroke(Stroke::new(1.0, BD)),
    )
}

pub fn danger_btn(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(label).monospace().size(9.0).color(ERR))
            .fill(Color32::TRANSPARENT)
            .stroke(Stroke::new(1.0, Color32::from_rgb(122, 37, 32))),
    )
}
