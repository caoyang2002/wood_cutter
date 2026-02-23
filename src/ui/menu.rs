use crate::ui::theme::{ACC, BD, OK, SF, TX, TX2};

pub fn draw_menu(
     ctx: &egui::Context,
) {
egui::TopBottomPanel::top("menu_bar")
     .frame(egui::Frame {
         fill: SF,  // 使用您的主题颜色
         inner_margin: egui::Margin::same(4.0),
         ..Default::default()
     })
     .show(ctx, |ui| {
         egui::menu::bar(ui, |ui| {
             ui.menu_button("文件", |ui| {
                 if ui.button("新建").clicked() {
                     // 处理新建逻辑
                 }
                 if ui.button("打开").clicked() {
                     // 处理打开逻辑
                 }
                 ui.separator();
                 if ui.button("退出").clicked() {
                     std::process::exit(0);
                 }
             });

             ui.menu_button("编辑", |ui| {
                 if ui.button("撤销").clicked() {
                     // 处理撤销逻辑
                 }
                 if ui.button("重做").clicked() {
                     // 处理重做逻辑
                 }
             });

             ui.menu_button("视图", |ui| {
                 // ui.checkbox(&mut self.show_grid, "显示网格");
                 // ui.checkbox(&mut self.show_labels, "显示标签");
             });

             ui.menu_button("帮助", |ui| {
                 if ui.button("关于").clicked() {
                     // self.show_about = true;
                 }
             });
         });
     });
}
