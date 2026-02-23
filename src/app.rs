use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use std::time::Instant;

use crate::algorithms::{pack_ga, pack_sa, pack_sorted, pack_svgnest, BinType, PackResult};
use crate::steps::{generate_steps, Step};
use crate::types::{AlgoId, ComputeState, ShapeEntry};
use crate::ui::{
accent_btn, danger_btn, draw_board_canvas, draw_convergence_chart, field_row, gc, secondary_btn,
section_header, ACC, BD, BG, ERR, INFO, OK, SF, SF2, TX, TX2,
};
use crate::utils::parse_positive_f64;

use std::sync::mpsc;
use std::thread;

// 添加进度更新枚举
enum ProgressUpdate {
Progress(f64, f64),  // (进度, 适应度)
Complete(PackResult, Vec<f64>, u128, f64, f64), // (result, history, elapsed, bw, bh)
Error(String),
}

pub struct WoodCutterApp {
// Board config
pub board_w: String,
pub board_h: String,
pub kerf: String,
pub allow_rotate: bool,

// Algo
pub algo: AlgoId,
pub p1: String,
pub p2: String,

// Shapes
pub shapes: Vec<ShapeEntry>,
pub shape_id_cnt: usize,

// Running
pub progress_rx: Option<mpsc::Receiver<ProgressUpdate>>,
pub ctx: Option<egui::Context>,

// Results
pub state: ComputeState,
pub progress: f64,
pub progress_msg: String,
pub solution_boards: Vec<crate::core::Board>,
pub solution_bw: f64,
pub solution_bh: f64,
pub solution_algo: AlgoId,
pub steps: Vec<Step>,
pub cur_step: usize,
pub gen_history: Vec<f64>,
pub unfittable: Vec<String>,

// Stats
pub stat_boards: String,
pub stat_util: String,
pub stat_shapes: String,
pub stat_waste: String,
pub stat_time: String,

// Tooltip
pub tooltip_text: Option<String>,

// Compute start time
pub compute_start: Option<Instant>,

// Error message
pub error_msg: Option<String>,
}

impl Default for WoodCutterApp {
fn default() -> Self {
let mut app = Self {
board_w: "2440".into(),
board_h: "1220".into(),
kerf: "3".into(),
allow_rotate: true,
algo: AlgoId::MaxRects,
p1: "30".into(),
p2: "100".into(),
shapes: vec![],
shape_id_cnt: 1,
state: ComputeState::Idle,
progress: 0.0,
progress_msg: String::new(),
solution_boards: vec![],
solution_bw: 2440.0,
solution_bh: 1220.0,
solution_algo: AlgoId::MaxRects,
steps: vec![],
cur_step: 0,
gen_history: vec![],
unfittable: vec![],
stat_boards: "—".into(),
stat_util: "—".into(),
stat_shapes: "—".into(),
stat_waste: "—".into(),
stat_time: "—".into(),
tooltip_text: None,
compute_start: None,
error_msg: None,
progress_rx: None,
ctx: None,
};
app.load_example();
app
}
}

impl WoodCutterApp {
fn handle_computation_result(&mut self, result: PackResult, hist: Vec<f64>, elapsed: u128, bw: f64, bh: f64) {
self.gen_history = hist;

let PackResult { boards, unfittable } = result;
self.unfittable = unfittable
.iter()
.map(|u| format!("• {} ({}×{}mm)", u.name, u.w, u.h))
.collect();

if boards.is_empty() {
self.state = ComputeState::Idle;
self.error_msg = Some("没有生成任何排样结果".into());
return;
}

// 重新计算总面积
let total_area: f64 = self.shapes
.iter()
.filter_map(|s| s.to_shape(0))
.map(|s| s.w * s.h)
.sum();
let total_board_area = boards.len() as f64 * bw * bh;

self.stat_boards = boards.len().to_string();
self.stat_util = format!("{:.1}%", total_area / total_board_area * 100.0);
self.stat_shapes = self.shapes.len().to_string();
self.stat_waste = format!("{:.4}m²", (total_board_area - total_area) / 1e6);
self.stat_time = format!("{}ms", elapsed);

self.solution_boards = boards.clone();
self.solution_bw = bw;
self.solution_bh = bh;
self.solution_algo = self.algo;
self.steps = generate_steps(&boards, bw, bh, self.algo.short());
self.cur_step = self.steps.len().saturating_sub(1);
self.state = ComputeState::Done;
}

pub fn add_shape(&mut self, name: &str, w: &str, h: &str) {
let id = self.shape_id_cnt;
self.shape_id_cnt += 1;
self.shapes.push(ShapeEntry {
id,
name: if name.is_empty() { format!("图形{}", id) } else { name.into() },
w: w.into(),
h: h.into(),
});
}

pub fn load_example(&mut self) {
self.shapes.clear();
self.shape_id_cnt = 1;
let data = [
("门板A", "800", "2000"),
("门板B", "800", "2000"),
("侧板L", "600", "2200"),
("侧板R", "600", "2200"),
("顶板", "1000", "600"),
("底板", "1000", "600"),
("抽屉面A", "400", "280"),
("抽屉面B", "400", "280"),
("搁板1", "900", "250"),
("搁板2", "900", "250"),
("搁板3", "500", "250"),
("背板", "980", "2180"),
];
for (n, w, h) in &data {
self.add_shape(n, w, h);
}
self.board_w = "2440".into();
self.board_h = "1220".into();
}

// pub fn run_compute(&mut self) {
//     let bw = match parse_positive_f64(&self.board_w, "木板长度") {
//         Ok(v) => v,
//         Err(e) => { self.error_msg = Some(e); return; }
//     };
//     let bh = match parse_positive_f64(&self.board_h, "木板宽度") {
//         Ok(v) => v,
//         Err(e) => { self.error_msg = Some(e); return; }
//     };
//     let kerf: f64 = self.kerf.parse().unwrap_or(0.0);
//     let ar = self.allow_rotate;

//     let tagged: Vec<crate::core::Shape> = self
//         .shapes
//         .iter()
//         .enumerate()
//         .filter_map(|(i, s)| s.to_shape(i))
//         .collect();

//     if tagged.is_empty() {
//         self.error_msg = Some("请至少添加一个有效图形".into());
//         return;
//     }

//     self.state = ComputeState::Running;
//     self.progress = 0.0;
//     self.error_msg = None;
//     self.compute_start = Some(Instant::now());

//     let algo = self.algo;
//     let p1: f64 = self.p1.parse().unwrap_or(algo.default_p1());
//     let p2: f64 = self.p2.parse().unwrap_or(algo.default_p2());

//     let t0 = Instant::now();

//     let (result, hist) = match algo {
//         AlgoId::MaxRects => (pack_sorted(BinType::MaxRects, &tagged, bw, bh, kerf, ar), vec![]),
//         AlgoId::Guillotine => (pack_sorted(BinType::Guillotine, &tagged, bw, bh, kerf, ar), vec![]),
//         AlgoId::BottomLeft => (pack_sorted(BinType::BottomLeft, &tagged, bw, bh, kerf, ar), vec![]),
//         AlgoId::NfpGreedy => (pack_sorted(BinType::NFP, &tagged, bw, bh, kerf, ar), vec![]),
//         AlgoId::GA => {
//             let (res, history) = pack_ga(
//                 &tagged, bw, bh, kerf, ar,
//                 p1 as usize, p2 as usize,
//                 &mut |_pct, _fit, _hist| {},
//             );
//             (res, history)
//         }
//         AlgoId::SA => {
//             let (res, history) = pack_sa(
//                 &tagged, bw, bh, kerf, ar,
//                 p1, p2 as usize,
//                 &mut |_pct, _fit, _hist| {},
//             );
//             (res, history)
//         }
//         AlgoId::SVGNest => {
//             let (res, history) = pack_svgnest(
//                 &tagged, bw, bh, kerf, ar,
//                 p1 as usize, p2 as usize,
//                 &mut |_pct, _fit, _hist| {},
//             );
//             (res, history)
//         }
//     };

//     let elapsed = t0.elapsed().as_millis();
//     self.gen_history = hist;

//     let PackResult { boards, unfittable } = result;
//     self.unfittable = unfittable
//         .iter()
//         .map(|u| format!("• {} ({}×{}mm)", u.name, u.w, u.h))
//         .collect();

//     if boards.is_empty() {
//         self.state = ComputeState::Idle;
//         self.error_msg = Some("没有生成任何排样结果".into());
//         return;
//     }

//     let total_area: f64 = tagged.iter().map(|s| s.w * s.h).sum();
//     let total_board_area = boards.len() as f64 * bw * bh;

//     self.stat_boards = boards.len().to_string();
//     self.stat_util = format!("{:.1}%", total_area / total_board_area * 100.0);
//     self.stat_shapes = tagged.len().to_string();
//     self.stat_waste = format!("{:.4}m²", (total_board_area - total_area) / 1e6);
//     self.stat_time = format!("{}ms", elapsed);

//     self.solution_boards = boards.clone();
//     self.solution_bw = bw;
//     self.solution_bh = bh;
//     self.solution_algo = algo;
//     self.steps = generate_steps(&boards, bw, bh, algo.short());
//     self.cur_step = self.steps.len().saturating_sub(1);
//     self.state = ComputeState::Done;
// }
//
pub fn run_compute(&mut self, ctx: egui::Context) {
// 解析参数
let bw = match parse_positive_f64(&self.board_w, "木板长度") {
Ok(v) => v,
Err(e) => { self.error_msg = Some(e); return; }
};
let bh = match parse_positive_f64(&self.board_h, "木板宽度") {
Ok(v) => v,
Err(e) => { self.error_msg = Some(e); return; }
};
let kerf: f64 = self.kerf.parse().unwrap_or(0.0);
let ar = self.allow_rotate;
let algo = self.algo;
let p1: f64 = self.p1.parse().unwrap_or(algo.default_p1());
let p2: f64 = self.p2.parse().unwrap_or(algo.default_p2());

let tagged: Vec<crate::core::Shape> = self
.shapes
.iter()
.enumerate()
.filter_map(|(i, s)| s.to_shape(i))
.collect();

if tagged.is_empty() {
self.error_msg = Some("请至少添加一个有效图形".into());
return;
}

// 设置运行状态
self.state = ComputeState::Running;
self.progress = 0.0;
self.progress_msg = "准备计算...".to_string();
self.error_msg = None;
self.compute_start = Some(Instant::now());

// 创建通道
let (tx, rx) = mpsc::channel();
self.progress_rx = Some(rx);
self.ctx = Some(ctx.clone());

// 在后台线程中执行计算
thread::spawn(move || {
let t0 = Instant::now();

// 根据算法类型执行计算
let (result, hist) = match algo {
AlgoId::MaxRects | AlgoId::Guillotine | AlgoId::BottomLeft | AlgoId::NfpGreedy => {
// 快速算法，直接执行
let bin_type = match algo {
AlgoId::MaxRects => BinType::MaxRects,
AlgoId::Guillotine => BinType::Guillotine,
AlgoId::BottomLeft => BinType::BottomLeft,
AlgoId::NfpGreedy => BinType::NFP,
_ => unreachable!(),
};

// 模拟进度更新（这些算法很快，但我们可以显示简单的进度）
for i in 0..5 {
tx.send(ProgressUpdate::Progress(i as f64 / 5.0, 0.0)).unwrap_or(());
thread::sleep(std::time::Duration::from_millis(10));
}

let result = pack_sorted(bin_type, &tagged, bw, bh, kerf, ar);
(result, vec![])
}

AlgoId::GA => {
let (res, history) = pack_ga(
&tagged, bw, bh, kerf, ar,
p1 as usize, p2 as usize,
&mut |pct, fit, _hist| {
// 发送进度更新
tx.send(ProgressUpdate::Progress(pct, fit)).unwrap_or(());
},
);
(res, history)
}

AlgoId::SA => {
let (res, history) = pack_sa(
&tagged, bw, bh, kerf, ar,
p1, p2 as usize,
&mut |pct, fit, _hist| {
tx.send(ProgressUpdate::Progress(pct, fit)).unwrap_or(());
},
);
(res, history)
}

AlgoId::SVGNest => {
    let (res, history) = pack_svgnest(
        &tagged, bw, bh, kerf, ar,
        p1 as usize, p2 as usize,
        &mut |pct, fit, _hist| {
            tx.send(ProgressUpdate::Progress(pct, fit)).unwrap_or(());
        },
    );
    (res, history)
}
};

let elapsed = t0.elapsed().as_millis();

// 发送完成消息
tx.send(ProgressUpdate::Complete(result, hist, elapsed, bw, bh)).unwrap_or(());
});
}

pub fn reset_all(&mut self) {
    self.state = ComputeState::Idle;
    self.solution_boards.clear();
    self.steps.clear();
    self.cur_step = 0;
    self.gen_history.clear();
    self.stat_boards = "—".into();
    self.stat_util = "—".into();
    self.stat_shapes = "—".into();
    self.stat_waste = "—".into();
    self.stat_time = "—".into();
}
}

impl eframe::App for WoodCutterApp {
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
// 检查是否有进度更新 - 使用 take() 获取所有权
if let Some(rx) = self.progress_rx.take() {
    // 创建一个本地变量来存储接收到的更新
    let mut updates = Vec::new();

    // 收集所有可用的更新
    while let Ok(update) = rx.try_recv() {
        updates.push(update);
    }

    // 处理收集到的更新
    for update in updates {
        match update {
            ProgressUpdate::Progress(pct, fit) => {
                self.progress = pct;
                self.progress_msg = format!("适应度: {:.4}", fit);
                ctx.request_repaint();
            }
            ProgressUpdate::Complete(result, hist, elapsed, bw, bh) => {
                // 处理完成结果
                self.handle_computation_result(result, hist, elapsed, bw, bh);
                // 注意：这里不重新设置 progress_rx，因为计算已完成
                ctx.request_repaint();
            }
            ProgressUpdate::Error(err) => {
                self.error_msg = Some(err);
                self.state = ComputeState::Idle;
                ctx.request_repaint();
            }
        }
    }

    // 如果计算还在进行中（没有收到 Complete 或 Error），重新设置 receiver
    if self.state == ComputeState::Running {
        self.progress_rx = Some(rx);
    }
}

// Apply dark style
let mut style = (*ctx.style()).clone();
style.visuals.dark_mode = true;
style.visuals.window_fill = BG;
style.visuals.panel_fill = SF;
style.visuals.extreme_bg_color = SF2;
style.visuals.widgets.noninteractive.bg_fill = SF2;
style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BD);
style.visuals.widgets.inactive.bg_fill = SF2;
style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(40, 40, 40);
style.visuals.widgets.active.bg_fill = Color32::from_rgb(50, 50, 50);
style.visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(212, 168, 83, 40);
style.visuals.hyperlink_color = ACC;
style.spacing.item_spacing = Vec2::new(6.0, 4.0);
style.spacing.window_margin = egui::Margin::same(0.0);
ctx.set_style(style);
crate::ui::draw_menu(ctx);
// ── Top Header ──
crate::ui::draw_header(
    ctx,
    &self.stat_boards,
    &self.stat_util,
    &self.stat_shapes,
    &self.stat_waste,
    &self.stat_time,
);

// ── Left Sidebar ──
egui::SidePanel::left("sidebar")
    .exact_width(330.0) // 侧边栏宽度
    .frame(egui::Frame {
        fill: SF, // 背景色
        inner_margin: egui::Margin::same(0.0), // 内边距
        stroke: Stroke::new(1.0, BD), // 边框颜色
        ..Default::default()
    })
    .show(ctx, |ui| {
        egui::ScrollArea::vertical()
            .id_salt("sb_scroll")

            .show(ui, |ui| {

                ui.add_space(10.0);
                let margin = egui::Margin::symmetric(13.0, 0.0);

                // Board config
                egui::Frame::none().inner_margin(margin).show(ui, |ui| {
                    section_header(ui, "木板规格（无限供应）");
                    egui::Grid::new("board_cfg")
                        .num_columns(2)
                        .spacing([6.0, 4.0])
                        .show(ui, |ui| {
                            field_row(ui, "长度 mm", &mut self.board_w);
                            ui.end_row();
                            field_row(ui, "宽度 mm", &mut self.board_h);
                            ui.end_row();
                            field_row(ui, "锯缝 mm", &mut self.kerf);
                            ui.end_row();
                            ui.label(RichText::new("允许旋转").size(9.0).color(TX2).monospace());
                            ui.checkbox(&mut self.allow_rotate, "");
                            ui.end_row();
                        });
                });

                ui.add_space(8.0);

                // Algorithm selector
                // 算法列表
                egui::Frame::none().inner_margin(margin).show(ui, |ui| {
                    section_header(ui, "排样算法");
                    let algos = [
                        AlgoId::MaxRects,
                        AlgoId::Guillotine,
                        AlgoId::BottomLeft,
                        AlgoId::NfpGreedy,
                        AlgoId::SA,
                        AlgoId::GA,
                        AlgoId::SVGNest,
                    ];
                    for algo in algos {
                        let selected = self.algo == algo;
                        let badge_color = match algo {
                            AlgoId::MaxRects | AlgoId::Guillotine | AlgoId::BottomLeft => OK,
                            AlgoId::SA | AlgoId::GA | AlgoId::NfpGreedy => INFO,
                            AlgoId::SVGNest => ACC,
                        };
                        let frame_stroke =
                            if selected { Stroke::new(1.0, ACC) } else { Stroke::new(1.0, BD) };
                        let frame_fill = if selected {
                            Color32::from_rgba_unmultiplied(212, 168, 83, 15)
                        } else {
                            Color32::TRANSPARENT
                        };

                        let resp = egui::Frame::none()
                            .fill(frame_fill)
                            .stroke(frame_stroke)
                            .inner_margin(egui::Margin::symmetric(9.0, 6.0))
                            .show(ui, |ui| {
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                    let (dot_rect, _) = ui.allocate_exact_size(
                                        Vec2::new(12.0, 12.0),
                                        egui::Sense::hover(),
                                    );
                                    let painter = ui.painter();
                                    painter.circle_stroke(
                                        dot_rect.center(),
                                        5.0,
                                        Stroke::new(1.5, if selected { ACC } else { TX2 }),
                                    );
                                    if selected {
                                        painter.circle_filled(dot_rect.center(), 2.5, ACC);
                                    }
                                    ui.add_space(4.0);
                                    ui.vertical(|ui| {
                                        ui.label(
                                            RichText::new(algo.label())
                                                .size(10.0)
                                                .color(TX)
                                                .strong()
                                                .monospace(),
                                        );
                                        ui.label(RichText::new(algo.desc()).size(8.0).color(TX2));
                                    });
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                        egui::Frame::none()
                                            .fill(Color32::from_rgba_unmultiplied(
                                                badge_color.r(), badge_color.g(), badge_color.b(), 30,
                                            ))
                                            .stroke(Stroke::new(
                                                1.0,
                                                Color32::from_rgba_unmultiplied(
                                                    badge_color.r(), badge_color.g(), badge_color.b(), 80,
                                                ),
                                            ))
                                            .inner_margin(egui::Margin::symmetric(5.0, 1.0))
                                            .rounding(Rounding::same(2.0))
                                            .show(ui, |ui| {
                                                ui.label(
                                                    RichText::new(algo.badge())
                                                        .size(7.0)
                                                        .color(badge_color)
                                                        .monospace(),
                                                );
                                            });
                                    });
                                });
                            });

                        if ui
                            .interact(
                                resp.response.rect,
                                egui::Id::new(format!("algo_{:?}", algo)),
                                egui::Sense::click(),
                            )
                            .clicked()
                        {
                            self.algo = algo;
                            self.p1 = algo.default_p1().to_string();
                            self.p2 = algo.default_p2().to_string();
                        }
                        ui.add_space(3.0);
                    }

                    // Meta params
                    if self.algo.has_meta() {
                        ui.add_space(4.0);
                        egui::Frame::none()
                            .fill(SF2)
                            .stroke(Stroke::new(1.0, BD))
                            .inner_margin(egui::Margin::symmetric(9.0, 7.0))
                            .show(ui, |ui| {
                                section_header(ui, "参数");
                                egui::Grid::new("meta_params")
                                    .num_columns(2)
                                    .spacing([6.0, 4.0])
                                    .show(ui, |ui| {
                                        field_row(ui, self.algo.p1_label(), &mut self.p1);
                                        ui.end_row();
                                        field_row(ui, self.algo.p2_label(), &mut self.p2);
                                        ui.end_row();
                                    });
                            });
                    }
                });

                ui.add_space(8.0);

                // Shapes list
                // 创建一个无内外边距的框架，使用指定的外边距
                // 图形列表
                egui::Frame::none().inner_margin(margin).show(ui, |ui| {
                // 水平布局显示标题和图形数量
                ui.horizontal(|ui| {
                    section_header(ui, "所需图形");  // 显示"所需图形"标题
                    if !self.shapes.is_empty() {
                        // 如果图形列表不为空，显示图形数量（用小号灰色文字）
                        ui.label(
                            RichText::new(format!("({})", self.shapes.len()))
                                .size(7.0)           // 字号7
                                .color(TX2),          // 使用主题灰色
                        );
                    }
                });

                // 定义操作状态变量（使用Option表示是否有操作需要执行）
                let mut to_remove: Option<usize> = None;  // 待删除的图形索引
                let mut move_up: Option<usize> = None;    // 待上移的图形索引
                let mut move_dn: Option<usize> = None;    // 待下移的图形索引

                // 创建垂直滚动区域，用于显示图形列表
                egui::ScrollArea::vertical()
                    .id_salt("shapes_scroll")     // 设置滚动区域的唯一ID
                    .max_height(220.0)             // 最大高度220像素
                    .show(ui, |ui| {
                // 遍历所有图形，idx为索引，s为可变引用
                for (idx, s) in self.shapes.iter_mut().enumerate() {
                let color = gc(idx);    // 根据索引获取主题颜色

                // 为每个图形项创建一个带边框的框架
                egui::Frame::none()
                    .fill(SF2)                    // 填充色（深色背景）
                    .stroke(Stroke::new(1.0, BD)) // 边框（1像素，主题边框色）
                    .show(ui, |ui| {
                // 设置水平布局，垂直居中对齐
                ui.with_layout(
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                // 左侧彩色条纹：分配一个3x32像素的区域作为颜色标识
                let (strip_rect, _) = ui.allocate_exact_size(
                    Vec2::new(3.0, 32.0),
                    egui::Sense::hover(),  // 仅用于悬停感应
                );
                // 绘制彩色条纹
                ui.painter().rect_filled(
                    strip_rect,
                    Rounding::ZERO,        // 无圆角
                    color,
                );

                ui.add_space(4.0);  // 添加间距

                // 垂直布局：名称标签和输入框
                ui.vertical(|ui| {
                    // 名称标签（小号灰色文字）
                    ui.label(
                        RichText::new("名称")
                            .size(7.0)
                            .color(TX2)
                            .monospace(),  // 等宽字体
                    );
                    // 名称输入框（单行文本编辑）
                    ui.add(
                        egui::TextEdit::singleline(&mut s.name)
                            .desired_width(90.0)           // 宽度90像素
                            .font(egui::FontId::monospace(11.0)),  // 等宽字体11号
                    );
                });

                ui.separator();  // 添加分隔线

                // 垂直布局：宽度标签和输入框
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("宽mm")
                            .size(7.0)
                            .color(TX2)
                            .monospace(),
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut s.w)
                            .desired_width(48.0)           // 较窄的输入框
                            .font(egui::FontId::monospace(11.0)),
                    );
                });

                ui.separator();  // 添加分隔线

                // 垂直布局：高度标签和输入框
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("高mm")
                            .size(7.0)
                            .color(TX2)
                            .monospace(),
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut s.h)
                            .desired_width(48.0)
                            .font(egui::FontId::monospace(11.0)),
                    );
                });

                // 右侧按钮区域（从右向左布局）
                ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                // 下移按钮（↓）
                if ui
                    .small_button(
                        RichText::new("↓").color(TX2),
                    )
                    .clicked()
                {
                    move_dn = Some(idx);  // 记录要下移的索引
                }
                // 删除按钮（✕）
                if ui
                    .small_button(
                        RichText::new("✕").color(ERR),  // 使用错误主题色（红色）
                    )
                    .clicked()
                {
                    to_remove = Some(idx);  // 记录要删除的索引
                }
                // 上移按钮（↑）
                if ui
                    .small_button(
                        RichText::new("↑").color(TX2),
                    )
                    .clicked()
                {
                move_up = Some(idx);  // 记录要上移的索引
                }
                },
                );
                },
                );
                let rect = ui.max_rect();
                println!("Element {} height: {}", idx, rect.height());
                });
                ui.add_space(2.0);  // 每个图形项之间的间距
                }
                });

                // 执行操作（在循环结束后统一处理，避免在迭代中修改集合）
                if let Some(i) = to_remove {
                self.shapes.remove(i);  // 删除指定索引的图形
                }
                if let Some(i) = move_up {
                if i > 0 {
                self.shapes.swap(i, i - 1);  // 与上一个元素交换位置（上移）
                }
                }
                if let Some(i) = move_dn {
                if i + 1 < self.shapes.len() {
                self.shapes.swap(i, i + 1);  // 与下一个元素交换位置（下移）
                }
                }

                ui.add_space(4.0);  // 添加间距

                // 底部按钮区域（水平布局）
                ui.horizontal(|ui| {
                // 添加按钮（使用强调色）
                if accent_btn(ui, "＋ 添加").clicked() {
                let id = self.shape_id_cnt;           // 获取新ID
                self.shape_id_cnt += 1;                // ID计数器自增
                // 创建新的图形条目（名称默认"图形X"，宽高为空）
                self.shapes.push(ShapeEntry {
                id,
                name: format!("图形{}", id),
                w: "".into(),
                h: "".into(),
                });
                }
                // 清空按钮（使用危险色）
                if danger_btn(ui, "清空").clicked() {
                self.shapes.clear();  // 清空所有图形
                }
                });
                });
                ui.add_space(8.0);

                // Import
                egui::Frame::none().inner_margin(margin).show(ui, |ui| {
                section_header(ui, "导入数据");
                if secondary_btn(ui, "📂 导入 .txt（名称,宽,高 每行）").clicked() {
                let path = rfd::FileDialog::new()
                .add_filter("文本文件", &["txt", "csv"])
                .pick_file();
                if let Some(p) = path {
                if let Ok(content) = std::fs::read_to_string(&p) {
                let mut n = 0;
                for line in content.lines() {
                let t = line.trim();
                if t.is_empty() || t.starts_with('#') { continue; }
                let parts: Vec<&str> = t
                .split([',', ';', '\t'])
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .collect();
                if parts.len() >= 3 {
                self.add_shape(parts[0], parts[1], parts[2]);
                n += 1;
                } else if parts.len() == 2 {
                self.add_shape("", parts[0], parts[1]);
                n += 1;
                }
                }
                if n > 0 {
                self.error_msg = Some(format!("✅ 成功导入 {} 个图形", n));
                }
                }
                }
                }
                });

                ui.add_space(8.0);

        // Actions
        egui::Frame::none()
            .inner_margin(egui::Margin::symmetric(13.0, 10.0))
            .show(ui, |ui| {
                // let running = self.state == ComputeState::Running;
                ui.add_enabled_ui(self.state != ComputeState::Running, |ui| {
                    if ui.add_sized(
                        [ui.available_width(), 36.0],
                        egui::Button::new(
                            egui::RichText::new("▶ 开始优化计算")
                                .size(13.0)
                                .color(egui::Color32::BLACK)
                                .strong(),)
                                .fill(ACC),)
                                .clicked() {
                                    self.run_compute(ctx.clone());  // 传入ctx
                                }
                        });

                        // 动态显示进度条
                        if self.state != ComputeState::Running {
                            ui.add(egui::ProgressBar::new(self.progress as f32)
                            .animate(true)
                            .text(format!("计算进度: {:.1}%", self.progress as f32 * 100.0)));
                        }


                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            if secondary_btn(ui, "示例数据").clicked() {
                                self.load_example();
                            }
                            if secondary_btn(ui, "清除结果").clicked() {
                                self.reset_all();
                            }
                        });

if let Some(ref msg) = self.error_msg.clone() {
ui.add_space(4.0);
let color = if msg.starts_with('✅') { OK } else { ERR };
ui.label(RichText::new(msg).size(9.0).color(color).monospace());
}

if !self.unfittable.is_empty() {
ui.add_space(2.0);
ui.label(
RichText::new("⚠ 无法放置:").size(9.0).color(ERR).monospace(),
);
for s in &self.unfittable {
ui.label(RichText::new(s).size(8.0).color(TX2).monospace());
}
}
});

ui.add_space(20.0);
});
});

// ── Central Panel ──
egui::CentralPanel::default()
.frame(egui::Frame { fill: BG, ..Default::default() })
.show(ctx, |ui| {
// Step navigation bar
if !self.steps.is_empty() {
egui::TopBottomPanel::top("ctrl_bar")
.frame(egui::Frame {
fill: SF,
inner_margin: egui::Margin::symmetric(16.0, 8.0),
stroke: Stroke::new(1.0, BD),
..Default::default()
})
.show_inside(ui, |ui| {
ui.horizontal(|ui| {
if secondary_btn(ui, "⏮").clicked() {
self.cur_step = 0;
}
if ui
.add_enabled(
self.cur_step > 0,
egui::Button::new(
RichText::new("◀ 上一步").monospace().size(9.0),
),
)
.clicked()
{
if self.cur_step > 0 { self.cur_step -= 1; }
}

let pct = if self.steps.len() > 1 {
self.cur_step as f32 / (self.steps.len() - 1) as f32
} else {
1.0
};
let (bar_rect, _) = ui.allocate_exact_size(
Vec2::new(ui.available_width() - 160.0, 4.0),
egui::Sense::hover(),
);
ui.painter().rect_filled(bar_rect, Rounding::same(2.0), BD);
let fill_w = bar_rect.width() * pct;
ui.painter().rect_filled(
egui::Rect::from_min_size(
bar_rect.min,
Vec2::new(fill_w, bar_rect.height()),
),
Rounding::same(2.0),
ACC,
);

if ui
.add_enabled(
self.cur_step < self.steps.len() - 1,
egui::Button::new(
RichText::new("下一步 ▶").monospace().size(9.0),
),
)
.clicked()
{
self.cur_step += 1;
}
if secondary_btn(ui, "⏭").clicked() {
self.cur_step = self.steps.len() - 1;
}
ui.label(
RichText::new(format!(
"步骤 {}/{}",
self.cur_step + 1,
self.steps.len()
))
.monospace()
.size(8.0)
.color(TX2),
);
});
});
}

egui::ScrollArea::vertical().id_salt("content_scroll").show(ui, |ui| {
ui.add_space(12.0);

if self.steps.is_empty() {
ui.add_space(80.0);
ui.vertical_centered(|ui| {
ui.label(RichText::new("🪵").size(48.0));
ui.add_space(10.0);
ui.label(
RichText::new("选择算法，配置木板与图形").size(11.0).color(TX2).monospace(),
);
ui.label(
RichText::new("点击「开始优化计算」").size(11.0).color(TX2).monospace(),
);
ui.add_space(6.0);
ui.label(
RichText::new(
"GA · SA · SVGNest · NFP · MaxRects · Guillotine · BL",
)
.size(9.0)
.color(ACC)
.monospace(),
);
});
return;
}

let step = &self.steps[self.cur_step];
let snap = step.snap.clone();
let hl = step.highlight;
let boards = self.solution_boards.clone();
let bw = self.solution_bw;
let bh = self.solution_bh;

// Step log
egui::Frame::none()
.inner_margin(egui::Margin::symmetric(16.0, 0.0))
.show(ui, |ui| {
egui::Frame::none()
.fill(SF2)
.stroke(Stroke::new(1.0, BD))
.inner_margin(egui::Margin { left: 12.0, right: 12.0, top: 8.0, bottom: 8.0 })
.show(ui, |ui| {
ui.label(RichText::new(&step.msg).monospace().size(9.0).color(TX));
});
});

ui.add_space(8.0);

// Convergence chart
let gen_history = self.gen_history.clone();
if gen_history.len() > 2 && self.cur_step == self.steps.len() - 1 {
egui::Frame::none()
.inner_margin(egui::Margin::symmetric(16.0, 0.0))
.show(ui, |ui| {
egui::Frame::none()
.fill(SF2)
.stroke(Stroke::new(1.0, BD))
.inner_margin(egui::Margin::same(8.0))
.show(ui, |ui| {
ui.label(
RichText::new(
"收敛曲线 — 适应度 vs 代/步数（越低越好）",
)
.size(8.0)
.color(TX2)
.monospace(),
);
let (chart_rect, _) = ui.allocate_exact_size(
Vec2::new(ui.available_width(), 80.0),
egui::Sense::hover(),
);
draw_convergence_chart(ui.painter(), chart_rect, &gen_history);
});
});
ui.add_space(8.0);
}

// Board cards
for (bi, board) in boards.iter().enumerate() {
let count = *snap.get(bi).unwrap_or(&0);
if count == 0 && bi > step.board_idx { continue; }

let used: f64 = board.placements.iter().take(count).map(|p| p.w * p.h).sum();
let util = used / (bw * bh) * 100.0;
let is_active = bi == step.board_idx;

let card_stroke =
if is_active { Stroke::new(1.0, ACC) } else { Stroke::new(1.0, BD) };

egui::Frame::none()
.inner_margin(egui::Margin::symmetric(16.0, 0.0))
.show(ui, |ui| {
egui::Frame::none().fill(SF).stroke(card_stroke).show(ui, |ui| {
// Card header
egui::Frame::none()
.fill(SF2)
.stroke(Stroke::new(1.0, BD))
.inner_margin(egui::Margin::symmetric(10.0, 6.0))
.show(ui, |ui| {
ui.horizontal(|ui| {
ui.label(
RichText::new(format!("木板 #{}", bi + 1))
.size(9.0)
.color(ACC)
.monospace()
.strong(),
);
ui.add_space(8.0);
let (bar_rect, _) = ui.allocate_exact_size(
Vec2::new(50.0, 2.0),
egui::Sense::hover(),
);
ui.painter().rect_filled(
bar_rect, Rounding::same(1.0), BD,
);
if count == board.placements.len() && count > 0 {
let fw = bar_rect.width() * (util / 100.0) as f32;
ui.painter().rect_filled(
egui::Rect::from_min_size(
bar_rect.min,
Vec2::new(fw, bar_rect.height()),
),
Rounding::same(1.0),
OK,
);
}
ui.with_layout(
egui::Layout::right_to_left(egui::Align::Center),
|ui| {
ui.label(
RichText::new(format!(
"{}×{}mm", bw, bh
))
.size(8.0)
.color(TX2)
.monospace(),
);
ui.add_space(8.0);
ui.label(
RichText::new(if count == board.placements.len() {
format!("{:.1}%", util)
} else {
"—".into()
})
.size(8.0)
.color(TX2)
.monospace(),
);
ui.add_space(8.0);
ui.label(
RichText::new(format!(
"{}/{} 件",
count,
board.placements.len()
))
.size(8.0)
.color(TX2)
.monospace(),
);
},
);
});
});

// Board canvas
egui::Frame::none()
.inner_margin(egui::Margin::same(9.0))
.show(ui, |ui| {
let avail_w = ui.available_width();
let scale = (avail_w as f64 / bw)
.min(320.0 / bh)
.min(0.3);
let canvas_w = (bw * scale) as f32;
let canvas_h = (bh * scale) as f32;

let (canvas_rect, canvas_resp) = ui.allocate_exact_size(
Vec2::new(canvas_w, canvas_h),
egui::Sense::hover(),
);

let hl_for_board =
if let Some((hi, hx, hy, hw, hh)) = hl {
if hi == bi { Some((hx, hy, hw, hh)) } else { None }
} else {
None
};

draw_board_canvas(
ui.painter(),
canvas_rect,
board,
count,
bw,
bh,
hl_for_board,
);

// Tooltip
if let Some(hover_pos) = canvas_resp.hover_pos() {
let sc = (canvas_rect.width() as f64 / bw)
.min(canvas_rect.height() as f64 / bh);
let mx = (hover_pos.x - canvas_rect.min.x) as f64 / sc;
let my = (hover_pos.y - canvas_rect.min.y) as f64 / sc;

let mut tip_text: Option<String> = None;
for p in board.placements.iter().take(count) {
if mx >= p.x && mx <= p.x + p.w
&& my >= p.y && my <= p.y + p.h
{
tip_text = Some(format!(
"名称: {}\n尺寸: {}×{}mm{}\n位置: ({}, {})\n面积: {:.4}m²",
p.name, p.w, p.h,
if p.rotated { " [旋转]" } else { "" },
p.x.round() as i64, p.y.round() as i64,
p.w * p.h / 1e6
));
break;
}
}
if let Some(tip) = tip_text {
canvas_resp.on_hover_text(
RichText::new(tip).monospace().size(10.0),
);
}
}

// Legend
egui::Frame::none().show(ui, |ui| {
ui.horizontal_wrapped(|ui| {
for p in board.placements.iter().take(count) {
let c = gc(p.color_idx);
ui.horizontal(|ui| {
let (dot, _) = ui.allocate_exact_size(
Vec2::new(7.0, 7.0),
egui::Sense::hover(),
);
ui.painter().rect_filled(
dot, Rounding::same(1.0), c,
);
ui.label(
RichText::new(&p.name)
.size(7.0)
.color(TX2)
.monospace(),
);
});
}
});
});
});
});
});

ui.add_space(10.0);
}

ui.add_space(20.0);
});
});
}
}
