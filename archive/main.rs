// mod algorithms;
// mod steps;

// use algorithms::*;
// use eframe::egui;
// use eframe::egui::{
//     Color32, FontId, Painter, Pos2, Rect, RichText, Rounding, Stroke, Vec2,
// };
// use steps::*;
// use std::time::Instant;

// // ─────────────────────────────────────────────
// //  Color palette
// // ─────────────────────────────────────────────
// const PALETTE: &[Color32] = &[
//     Color32::from_rgb(231, 76, 60),
//     Color32::from_rgb(230, 126, 34),
//     Color32::from_rgb(241, 196, 15),
//     Color32::from_rgb(46, 204, 113),
//     Color32::from_rgb(26, 188, 156),
//     Color32::from_rgb(52, 152, 219),
//     Color32::from_rgb(155, 89, 182),
//     Color32::from_rgb(233, 30, 99),
//     Color32::from_rgb(255, 87, 34),
//     Color32::from_rgb(0, 188, 212),
//     Color32::from_rgb(139, 195, 74),
//     Color32::from_rgb(255, 152, 0),
//     Color32::from_rgb(103, 58, 183),
//     Color32::from_rgb(0, 150, 136),
//     Color32::from_rgb(240, 98, 146),
//     Color32::from_rgb(66, 165, 245),
//     Color32::from_rgb(102, 187, 106),
//     Color32::from_rgb(255, 167, 38),
//     Color32::from_rgb(171, 71, 188),
//     Color32::from_rgb(38, 198, 218),
//     Color32::from_rgb(239, 154, 154),
//     Color32::from_rgb(255, 204, 128),
//     Color32::from_rgb(197, 225, 165),
//     Color32::from_rgb(128, 222, 234),
// ];

// fn gc(idx: usize) -> Color32 {
//     PALETTE[idx % PALETTE.len()]
// }

// // ─────────────────────────────────────────────
// //  Algo selector
// // ─────────────────────────────────────────────
// #[derive(Clone, Copy, PartialEq, Debug)]
// enum AlgoId {
//     MaxRects,
//     Guillotine,
//     BottomLeft,
//     NfpGreedy,
//     SA,
//     GA,
//     SVGNest,
// }

// impl AlgoId {
//     fn label(&self) -> &'static str {
//         match self {
//             AlgoId::MaxRects => "MaxRects 最大矩形",
//             AlgoId::Guillotine => "Guillotine 断头台",
//             AlgoId::BottomLeft => "Bottom-Left 天际线",
//             AlgoId::NfpGreedy => "NFP 临界多边形贪心",
//             AlgoId::SA => "SA 模拟退火",
//             AlgoId::GA => "GA 遗传算法",
//             AlgoId::SVGNest => "SVGNest 算法",
//         }
//     }
//     fn short(&self) -> &'static str {
//         match self {
//             AlgoId::MaxRects => "MaxRects",
//             AlgoId::Guillotine => "Guillotine",
//             AlgoId::BottomLeft => "Bottom-Left",
//             AlgoId::NfpGreedy => "NFP贪心",
//             AlgoId::SA => "模拟退火SA",
//             AlgoId::GA => "遗传算法GA",
//             AlgoId::SVGNest => "SVGNest",
//         }
//     }
//     fn badge(&self) -> &'static str {
//         match self {
//             AlgoId::MaxRects | AlgoId::Guillotine | AlgoId::BottomLeft => "通用/快速",
//             AlgoId::NfpGreedy => "极限利用率",
//             AlgoId::SA => "中等规模优化",
//             AlgoId::GA => "大规模搜索",
//             AlgoId::SVGNest => "复杂图形约束",
//         }
//     }
//     fn desc(&self) -> &'static str {
//         match self {
//             AlgoId::MaxRects => "BSSF贪心，维护所有自由矩形，工业级标准算法",
//             AlgoId::Guillotine => "模拟锯切，每刀产生两个矩形区域，贴近实际切割",
//             AlgoId::BottomLeft => "图形靠左下紧凑放置，维护天际线高度图",
//             AlgoId::NfpGreedy => "精确计算No-Fit Polygon边界，从所有接触点选最优",
//             AlgoId::SA => "以概率接受较差解跳出局部最优，在MaxRects基础上搜索更优排列",
//             AlgoId::GA => "OX交叉+精英保留，使用MaxRects评估适应度，搜索全局最优排列",
//             AlgoId::SVGNest => "GA进化顺序 + NFP精确接触定位 + 最低重心评分",
//         }
//     }
//     fn has_meta(&self) -> bool {
//         matches!(self, AlgoId::SA | AlgoId::GA | AlgoId::SVGNest)
//     }
//     fn p1_label(&self) -> &'static str {
//         match self {
//             AlgoId::SA => "初始温度",
//             _ => "种群大小",
//         }
//     }
//     fn p2_label(&self) -> &'static str {
//         match self {
//             AlgoId::SA => "迭代步数",
//             _ => "进化代数",
//         }
//     }
//     fn default_p1(&self) -> f64 {
//         match self {
//             AlgoId::SA => 1000.0,
//             AlgoId::GA => 30.0,
//             AlgoId::SVGNest => 20.0,
//             _ => 20.0,
//         }
//     }
//     fn default_p2(&self) -> f64 {
//         match self {
//             AlgoId::SA => 500.0,
//             AlgoId::GA => 100.0,
//             AlgoId::SVGNest => 60.0,
//             _ => 80.0,
//         }
//     }
// }

// // ─────────────────────────────────────────────
// //  Shape entry (for UI)
// // ─────────────────────────────────────────────
// #[derive(Clone, Debug)]
// struct ShapeEntry {
//     id: usize,
//     name: String,
//     w: String,
//     h: String,
// }

// impl ShapeEntry {
//     fn to_shape(&self, color_idx: usize) -> Option<Shape> {
//         let w = self.w.parse::<f64>().ok()?;
//         let h = self.h.parse::<f64>().ok()?;
//         if w <= 0.0 || h <= 0.0 {
//             return None;
//         }
//         Some(Shape { id: self.id, name: self.name.clone(), w, h, color_idx })
//     }
// }

// // ─────────────────────────────────────────────
// //  App State
// // ─────────────────────────────────────────────
// #[derive(PartialEq)]
// enum ComputeState {
//     Idle,
//     Running,
//     Done,
// }

// struct WoodCutterApp {
//     // Board config
//     board_w: String,
//     board_h: String,
//     kerf: String,
//     allow_rotate: bool,

//     // Algo
//     algo: AlgoId,
//     p1: String,
//     p2: String,

//     // Shapes
//     shapes: Vec<ShapeEntry>,
//     shape_id_cnt: usize,

//     // Results
//     state: ComputeState,
//     progress: f64,
//     progress_msg: String,
//     solution_boards: Vec<algorithms::Board>,
//     solution_bw: f64,
//     solution_bh: f64,
//     solution_algo: AlgoId,
//     steps: Vec<Step>,
//     cur_step: usize,
//     gen_history: Vec<f64>,
//     unfittable: Vec<String>,

//     // Stats
//     stat_boards: String,
//     stat_util: String,
//     stat_shapes: String,
//     stat_waste: String,
//     stat_time: String,

//     // Tooltip
//     tooltip_text: Option<String>,

//     // Compute start time (for elapsed)
//     compute_start: Option<Instant>,

//     // Error message
//     error_msg: Option<String>,
// }

// impl Default for WoodCutterApp {
//     fn default() -> Self {
//         let mut app = Self {
//             board_w: "2440".into(),
//             board_h: "1220".into(),
//             kerf: "3".into(),
//             allow_rotate: true,
//             algo: AlgoId::MaxRects,
//             p1: "30".into(),
//             p2: "100".into(),
//             shapes: vec![],
//             shape_id_cnt: 1,
//             state: ComputeState::Idle,
//             progress: 0.0,
//             progress_msg: String::new(),
//             solution_boards: vec![],
//             solution_bw: 2440.0,
//             solution_bh: 1220.0,
//             solution_algo: AlgoId::MaxRects,
//             steps: vec![],
//             cur_step: 0,
//             gen_history: vec![],
//             unfittable: vec![],
//             stat_boards: "—".into(),
//             stat_util: "—".into(),
//             stat_shapes: "—".into(),
//             stat_waste: "—".into(),
//             stat_time: "—".into(),
//             tooltip_text: None,
//             compute_start: None,
//             error_msg: None,
//         };
//         app.load_example();
//         app
//     }
// }

// impl WoodCutterApp {
//     fn add_shape(&mut self, name: &str, w: &str, h: &str) {
//         let id = self.shape_id_cnt;
//         self.shape_id_cnt += 1;
//         self.shapes.push(ShapeEntry {
//             id,
//             name: if name.is_empty() { format!("图形{}", id) } else { name.into() },
//             w: w.into(),
//             h: h.into(),
//         });
//     }

//     fn load_example(&mut self) {
//         self.shapes.clear();
//         self.shape_id_cnt = 1;
//         let data = [
//             ("门板A", "800", "2000"),
//             ("门板B", "800", "2000"),
//             ("侧板L", "600", "2200"),
//             ("侧板R", "600", "2200"),
//             ("顶板", "1000", "600"),
//             ("底板", "1000", "600"),
//             ("抽屉面A", "400", "280"),
//             ("抽屉面B", "400", "280"),
//             ("搁板1", "900", "250"),
//             ("搁板2", "900", "250"),
//             ("搁板3", "500", "250"),
//             ("背板", "980", "2180"),
//         ];
//         for (n, w, h) in &data {
//             self.add_shape(n, w, h);
//         }
//         self.board_w = "2440".into();
//         self.board_h = "1220".into();
//     }

//     fn run_compute(&mut self) {
//         let bw: f64 = match self.board_w.parse() {
//             Ok(v) if v > 0.0 => v,
//             _ => { self.error_msg = Some("请输入有效木板长度".into()); return; }
//         };
//         let bh: f64 = match self.board_h.parse() {
//             Ok(v) if v > 0.0 => v,
//             _ => { self.error_msg = Some("请输入有效木板宽度".into()); return; }
//         };
//         let kerf: f64 = self.kerf.parse().unwrap_or(0.0);
//         let ar = self.allow_rotate;

//         let tagged: Vec<Shape> = self.shapes.iter().enumerate().filter_map(|(i, s)| s.to_shape(i)).collect();
//         if tagged.is_empty() {
//             self.error_msg = Some("请至少添加一个有效图形".into());
//             return;
//         }

//         self.state = ComputeState::Running;
//         self.progress = 0.0;
//         self.error_msg = None;
//         self.compute_start = Some(Instant::now());

//         let algo = self.algo;
//         let p1: f64 = self.p1.parse().unwrap_or(algo.default_p1());
//         let p2: f64 = self.p2.parse().unwrap_or(algo.default_p2());

//         let t0 = Instant::now();

//         let (result, hist) = match algo {
//             AlgoId::MaxRects => (pack_sorted(BinType::MaxRects, &tagged, bw, bh, kerf, ar), vec![]),
//             AlgoId::Guillotine => (pack_sorted(BinType::Guillotine, &tagged, bw, bh, kerf, ar), vec![]),
//             AlgoId::BottomLeft => (pack_sorted(BinType::BottomLeft, &tagged, bw, bh, kerf, ar), vec![]),
//             AlgoId::NfpGreedy => (pack_sorted(BinType::NFP, &tagged, bw, bh, kerf, ar), vec![]),
//             AlgoId::GA => {
//                 let mut h = vec![];
//                 let (res, history) = pack_ga(
//                     &tagged, bw, bh, kerf, ar,
//                     p1 as usize, p2 as usize,
//                     &mut |pct, _fit, hist| {
//                         h = hist.to_vec();
//                         let _ = pct;
//                     },
//                 );
//                 (res, history)
//             }
//             AlgoId::SA => {
//                 let mut h = vec![];
//                 let (res, history) = pack_sa(
//                     &tagged, bw, bh, kerf, ar,
//                     p1, p2 as usize,
//                     &mut |_pct, _fit, hist| {
//                         h = hist.to_vec();
//                     },
//                 );
//                 (res, history)
//             }
//             AlgoId::SVGNest => {
//                 let mut h = vec![];
//                 let (res, history) = pack_svgnest(
//                     &tagged, bw, bh, kerf, ar,
//                     p1 as usize, p2 as usize,
//                     &mut |_pct, _fit, hist| {
//                         h = hist.to_vec();
//                     },
//                 );
//                 (res, history)
//             }
//         };

//         let elapsed = t0.elapsed().as_millis();
//         self.gen_history = hist;

//         let PackResult { boards, unfittable } = result;
//         self.unfittable = unfittable.iter().map(|u| format!("• {} ({}×{}mm)", u.name, u.w, u.h)).collect();

//         if boards.is_empty() {
//             self.state = ComputeState::Idle;
//             self.error_msg = Some("没有生成任何排样结果".into());
//             return;
//         }

//         let total_area: f64 = tagged.iter().map(|s| s.w * s.h).sum();
//         let total_board_area = boards.len() as f64 * bw * bh;

//         self.stat_boards = boards.len().to_string();
//         self.stat_util = format!("{:.1}%", total_area / total_board_area * 100.0);
//         self.stat_shapes = tagged.len().to_string();
//         self.stat_waste = format!("{:.4}m²", (total_board_area - total_area) / 1e6);
//         self.stat_time = format!("{}ms", elapsed);

//         self.solution_boards = boards.clone();
//         self.solution_bw = bw;
//         self.solution_bh = bh;
//         self.solution_algo = algo;
//         self.steps = generate_steps(&boards, bw, bh, algo.short());
//         self.cur_step = self.steps.len().saturating_sub(1);
//         self.state = ComputeState::Done;
//     }

//     fn reset_all(&mut self) {
//         self.state = ComputeState::Idle;
//         self.solution_boards.clear();
//         self.steps.clear();
//         self.cur_step = 0;
//         self.gen_history.clear();
//         self.stat_boards = "—".into();
//         self.stat_util = "—".into();
//         self.stat_shapes = "—".into();
//         self.stat_waste = "—".into();
//         self.stat_time = "—".into();
//     }
// }

// // ─────────────────────────────────────────────
// //  UI Drawing helpers
// // ─────────────────────────────────────────────

// fn draw_board_canvas(
//     painter: &Painter,
//     rect: Rect,
//     board: &algorithms::Board,
//     count: usize,
//     bw: f64,
//     bh: f64,
//     highlight: Option<(f64, f64, f64, f64)>,
// ) {
//     let scale_x = rect.width() as f64 / bw;
//     let scale_y = rect.height() as f64 / bh;
//     let scale = scale_x.min(scale_y);
//     let ox = rect.min.x;
//     let oy = rect.min.y;

//     // Background: dark wood color
//     painter.rect_filled(rect, Rounding::same(2.0), Color32::from_rgb(26, 15, 4));

//     // Subtle wood grain lines
//     let grain_color = Color32::from_rgba_unmultiplied(130, 70, 15, 8);
//     let mut gy = rect.min.y;
//     while gy < rect.max.y {
//         painter.line_segment(
//             [Pos2::new(rect.min.x, gy), Pos2::new(rect.max.x, gy)],
//             Stroke::new(0.5, grain_color),
//         );
//         gy += 5.0;
//     }

//     // Grid lines at 100mm intervals
//     let grid_color = Color32::from_rgba_unmultiplied(255, 255, 255, 8);
//     let step_px = (scale * 100.0) as f32;
//     let mut gx = rect.min.x;
//     while gx < rect.max.x {
//         painter.line_segment(
//             [Pos2::new(gx, rect.min.y), Pos2::new(gx, rect.max.y)],
//             Stroke::new(0.3, grid_color),
//         );
//         gx += step_px;
//     }
//     let mut gy = rect.min.y;
//     while gy < rect.max.y {
//         painter.line_segment(
//             [Pos2::new(rect.min.x, gy), Pos2::new(rect.max.x, gy)],
//             Stroke::new(0.3, grid_color),
//         );
//         gy += step_px;
//     }

//     // Draw placements
//     for p in board.placements.iter().take(count) {
//         let px = ox + (p.x * scale) as f32;
//         let py = oy + (p.y * scale) as f32;
//         let pw = (p.w * scale) as f32;
//         let ph = (p.h * scale) as f32;

//         let color = gc(p.color_idx);
//         let fill = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 220);
//         let piece_rect = Rect::from_min_size(Pos2::new(px, py), Vec2::new(pw, ph));

//         let is_hl = if let Some((hx, hy, _hw, _hh)) = highlight {
//             (p.x - hx).abs() < 0.5 && (p.y - hy).abs() < 0.5
//         } else {
//             false
//         };

//         painter.rect_filled(piece_rect, Rounding::same(1.0), fill);

//         // Inner texture lines
//         if ph > 6.0 {
//             let texture_color = Color32::from_rgba_unmultiplied(255, 255, 255, 12);
//             let mut ty = py + 7.0;
//             while ty < py + ph {
//                 painter.line_segment(
//                     [Pos2::new(px, ty), Pos2::new(px + pw, ty)],
//                     Stroke::new(0.4, texture_color),
//                 );
//                 ty += 7.0;
//             }
//         }

//         // Border
//         if is_hl {
//             painter.rect_stroke(
//                 piece_rect,
//                 Rounding::same(1.0),
//                 Stroke::new(2.0, Color32::WHITE),
//             );
//         } else {
//             painter.rect_stroke(
//                 piece_rect,
//                 Rounding::same(1.0),
//                 Stroke::new(0.6, Color32::from_rgba_unmultiplied(255, 255, 255, 55)),
//             );
//         }

//         // Label
//         if pw > 14.0 && ph > 9.0 {
//             let font_size = (pw / 5.0).min(ph / 2.2).min(11.0).max(6.0);
//             let cx = px + pw / 2.0;
//             let cy = py + ph / 2.0;
//             painter.text(
//                 Pos2::new(cx, cy),
//                 egui::Align2::CENTER_CENTER,
//                 &p.name,
//                 FontId::monospace(font_size),
//                 Color32::from_rgba_unmultiplied(255, 255, 255, 230),
//             );
//             // Dimensions below name
//             if ph > font_size * 2.8 && font_size >= 7.0 {
//                 painter.text(
//                     Pos2::new(cx, cy + font_size + 2.0),
//                     egui::Align2::CENTER_CENTER,
//                     &format!("{}×{}", p.w, p.h),
//                     FontId::monospace((font_size - 2.0).max(5.0)),
//                     Color32::from_rgba_unmultiplied(255, 255, 255, 90),
//                 );
//             }
//             // Rotation mark
//             if p.rotated && pw > 12.0 {
//                 painter.text(
//                     Pos2::new(px + 3.0, py + 2.0),
//                     egui::Align2::LEFT_TOP,
//                     "↻",
//                     FontId::proportional(9.0),
//                     Color32::from_rgb(255, 220, 60),
//                 );
//             }
//         }

//         // Highlight dashed border
//         if is_hl {
//             // Draw animated dashed border using small segments
//             let acc = Color32::from_rgb(212, 168, 83);
//             painter.rect_stroke(piece_rect.expand(1.5), Rounding::same(2.0), Stroke::new(2.0, acc));
//         }
//     }

//     // Outer board border
//     painter.rect_stroke(rect, Rounding::same(2.0), Stroke::new(1.5, Color32::from_rgb(72, 72, 72)));
// }

// fn draw_convergence_chart(
//     painter: &Painter,
//     rect: Rect,
//     history: &[f64],
// ) {
//     if history.len() < 2 {
//         return;
//     }
//     painter.rect_filled(rect, Rounding::same(2.0), Color32::from_rgb(28, 28, 28));

//     let min = history.iter().cloned().fold(f64::INFINITY, f64::min);
//     let max = history.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
//     let range = (max - min).max(1e-9);
//     let pad = 12.0_f32;
//     let w = rect.width() - 2.0 * pad;
//     let h = rect.height() - 2.0 * pad;

//     let to_pos = |i: usize, v: f64| -> Pos2 {
//         Pos2::new(
//             rect.min.x + pad + (i as f32 / (history.len() - 1) as f32) * w,
//             rect.min.y + pad + (1.0 - ((v - min) / range) as f32) * h,
//         )
//     };

//     // Fill area
//     let mut fill_pts: Vec<Pos2> = history.iter().enumerate().map(|(i, &v)| to_pos(i, v)).collect();
//     fill_pts.push(to_pos(history.len() - 1, min));
//     fill_pts.push(to_pos(0, min));
//     painter.add(egui::Shape::convex_polygon(
//         fill_pts,
//         Color32::from_rgba_unmultiplied(212, 168, 83, 30),
//         Stroke::NONE,
//     ));

//     // Line
//     let pts: Vec<Pos2> = history.iter().enumerate().map(|(i, &v)| to_pos(i, v)).collect();
//     painter.add(egui::Shape::line(pts, Stroke::new(1.5, Color32::from_rgb(212, 168, 83))));

//     // Labels
//     painter.text(
//         Pos2::new(rect.min.x + pad + 2.0, rect.min.y + pad + 3.0),
//         egui::Align2::LEFT_TOP,
//         &format!("最高:{:.3}", max),
//         FontId::monospace(9.0),
//         Color32::from_rgba_unmultiplied(122, 106, 88, 200),
//     );
//     painter.text(
//         Pos2::new(rect.min.x + pad + 2.0, rect.max.y - pad - 3.0),
//         egui::Align2::LEFT_BOTTOM,
//         &format!("最低:{:.3}", min),
//         FontId::monospace(9.0),
//         Color32::from_rgba_unmultiplied(122, 106, 88, 200),
//     );
// }

// // ─────────────────────────────────────────────
// //  Colors / theme
// // ─────────────────────────────────────────────
// const BG: Color32 = Color32::from_rgb(12, 12, 12);
// const SF: Color32 = Color32::from_rgb(21, 21, 21);
// const SF2: Color32 = Color32::from_rgb(28, 28, 28);
// const BD: Color32 = Color32::from_rgb(42, 42, 42);
// const ACC: Color32 = Color32::from_rgb(212, 168, 83);
// const ACC3: Color32 = Color32::from_rgb(240, 192, 96);
// const TX: Color32 = Color32::from_rgb(224, 216, 204);
// const TX2: Color32 = Color32::from_rgb(122, 106, 88);
// const ERR: Color32 = Color32::from_rgb(192, 57, 43);
// const OK: Color32 = Color32::from_rgb(39, 174, 96);
// const INFO: Color32 = Color32::from_rgb(41, 128, 185);

// // ─────────────────────────────────────────────
// //  eframe App impl
// // ─────────────────────────────────────────────
// impl eframe::App for WoodCutterApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         // Apply dark style
//         let mut style = (*ctx.style()).clone();
//         style.visuals.dark_mode = true;
//         style.visuals.window_fill = BG;
//         style.visuals.panel_fill = SF;
//         style.visuals.extreme_bg_color = SF2;
//         style.visuals.widgets.noninteractive.bg_fill = SF2;
//         style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BD);
//         style.visuals.widgets.inactive.bg_fill = SF2;
//         style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(40, 40, 40);
//         style.visuals.widgets.active.bg_fill = Color32::from_rgb(50, 50, 50);
//         style.visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(212, 168, 83, 40);
//         style.visuals.hyperlink_color = ACC;
//         style.spacing.item_spacing = Vec2::new(6.0, 4.0);
//         style.spacing.window_margin = egui::Margin::same(0.0);
//         ctx.set_style(style);

//         // Top header panel
//         egui::TopBottomPanel::top("header")
//             .exact_height(44.0)
//             .frame(egui::Frame {
//                 fill: SF,
//                 inner_margin: egui::Margin::symmetric(22.0, 0.0),
//                 stroke: Stroke::new(1.0, BD),
//                 ..Default::default()
//             })
//             .show(ctx, |ui| {
//                 ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
//                     ui.add_space(0.0);
//                     ui.label(RichText::new("木板分割优化").size(17.0).color(ACC).strong());
//                     ui.add_space(8.0);
//                     ui.label(RichText::new("NFP · GA · SA · SVGNest · MaxRects").size(9.0).color(TX2));

//                     ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
//                         let stats = [
//                             ("耗时ms", &self.stat_time),
//                             ("废料m²", &self.stat_waste),
//                             ("图形数", &self.stat_shapes),
//                             ("利用率", &self.stat_util),
//                             ("木板数", &self.stat_boards),
//                         ];
//                         for (label, val) in &stats {
//                             ui.add_space(8.0);
//                             ui.vertical(|ui| {
//                                 ui.add_space(6.0);
//                                 ui.label(RichText::new(*val).size(15.0).color(ACC).strong().monospace());
//                                 ui.label(RichText::new(*label).size(7.0).color(TX2).monospace());
//                             });
//                             ui.add_space(4.0);
//                             ui.separator();
//                         }
//                     });
//                 });
//             });

//         // Left sidebar
//         egui::SidePanel::left("sidebar")
//             .exact_width(330.0)
//             .frame(egui::Frame {
//                 fill: SF,
//                 inner_margin: egui::Margin::same(0.0),
//                 stroke: Stroke::new(1.0, BD),
//                 ..Default::default()
//             })
//             .show(ctx, |ui| {
//         // 滚动区域
//                 egui::ScrollArea::vertical()
//                     .id_salt("sb_scroll")
//                     .show(ui, |ui| {
//                         ui.add_space(10.0);
//                         let margin = egui::Margin::symmetric(13.0, 0.0);

//                         // ── Board Config ──
//                         egui::Frame::none().inner_margin(margin).show(ui, |ui| {
//                             section_header(ui, "木板规格（无限供应）");
//                             egui::Grid::new("board_cfg").num_columns(2).spacing([6.0, 4.0]).show(ui, |ui| {
//                                 field_row(ui, "长度 mm", &mut self.board_w);
//                                 ui.end_row();
//                                 field_row(ui, "宽度 mm", &mut self.board_h);
//                                 ui.end_row();
//                                 field_row(ui, "锯缝 mm", &mut self.kerf);
//                                 ui.end_row();
//                                 ui.label(RichText::new("允许旋转").size(9.0).color(TX2).monospace());
//                                 ui.checkbox(&mut self.allow_rotate, "");
//                                 ui.end_row();
//                             });
//                         });

//                         ui.add_space(8.0);

//                         // ── Algorithm ──
//                         egui::Frame::none().inner_margin(margin).show(ui, |ui| {
//                             section_header(ui, "排样算法");

//                             let algos = [
//                                 AlgoId::MaxRects,
//                                 AlgoId::Guillotine,
//                                 AlgoId::BottomLeft,
//                                 AlgoId::NfpGreedy,
//                                 AlgoId::SA,
//                                 AlgoId::GA,
//                                 AlgoId::SVGNest,
//                             ];
//                             for algo in algos {
//                                 let selected = self.algo == algo;
//                                 let badge_color = match algo {
//                                     AlgoId::MaxRects | AlgoId::Guillotine | AlgoId::BottomLeft => OK,
//                                     AlgoId::SA | AlgoId::GA | AlgoId::NfpGreedy => INFO,
//                                     AlgoId::SVGNest => ACC,
//                                 };
//                                 let frame_stroke = if selected {
//                                     Stroke::new(1.0, ACC)
//                                 } else {
//                                     Stroke::new(1.0, BD)
//                                 };
//                                 let frame_fill = if selected {
//                                     Color32::from_rgba_unmultiplied(212, 168, 83, 15)
//                                 } else {
//                                     Color32::TRANSPARENT
//                                 };

//                                 let resp = egui::Frame::none()
//                                     .fill(frame_fill)
//                                     .stroke(frame_stroke)
//                                     .inner_margin(egui::Margin::symmetric(9.0, 6.0))
//                                     .show(ui, |ui| {
//                                         ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
//                                             // Radio dot
//                                             let (dot_rect, _) = ui.allocate_exact_size(Vec2::new(12.0, 12.0), egui::Sense::hover());
//                                             let painter = ui.painter();
//                                             painter.circle_stroke(dot_rect.center(), 5.0, Stroke::new(1.5, if selected { ACC } else { TX2 }));
//                                             if selected {
//                                                 painter.circle_filled(dot_rect.center(), 2.5, ACC);
//                                             }

//                                             ui.add_space(4.0);
//                                             ui.vertical(|ui| {
//                                                 ui.label(RichText::new(algo.label()).size(10.0).color(TX).strong().monospace());
//                                                 ui.label(RichText::new(algo.desc()).size(8.0).color(TX2));
//                                             });

//                                             ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
//                                                 egui::Frame::none()
//                                                     .fill(Color32::from_rgba_unmultiplied(badge_color.r(), badge_color.g(), badge_color.b(), 30))
//                                                     .stroke(Stroke::new(1.0, Color32::from_rgba_unmultiplied(badge_color.r(), badge_color.g(), badge_color.b(), 80)))
//                                                     .inner_margin(egui::Margin::symmetric(5.0, 1.0))
//                                                     .rounding(Rounding::same(2.0))
//                                                     .show(ui, |ui| {
//                                                         ui.label(RichText::new(algo.badge()).size(7.0).color(badge_color).monospace());
//                                                     });
//                                             });
//                                         });
//                                     });

//                                 if ui.interact(resp.response.rect, egui::Id::new(format!("algo_{:?}", algo)), egui::Sense::click()).clicked() {
//                                     self.algo = algo;
//                                     self.p1 = algo.default_p1().to_string();
//                                     self.p2 = algo.default_p2().to_string();
//                                 }
//                                 ui.add_space(3.0);
//                             }

//                             // Meta params for SA/GA/SVGNest
//                             if self.algo.has_meta() {
//                                 ui.add_space(4.0);
//                                 egui::Frame::none()
//                                     .fill(SF2)
//                                     .stroke(Stroke::new(1.0, BD))
//                                     .inner_margin(egui::Margin::symmetric(9.0, 7.0))
//                                     .show(ui, |ui| {
//                                         section_header(ui, "参数");
//                                         egui::Grid::new("meta_params").num_columns(2).spacing([6.0, 4.0]).show(ui, |ui| {
//                                             field_row(ui, self.algo.p1_label(), &mut self.p1);
//                                             ui.end_row();
//                                             field_row(ui, self.algo.p2_label(), &mut self.p2);
//                                             ui.end_row();
//                                         });
//                                     });
//                             }
//                         });

//                         ui.add_space(8.0);

//                         // ── Shapes ──
//                         egui::Frame::none().inner_margin(margin).show(ui, |ui| {
//                             ui.horizontal(|ui| {
//                                 section_header(ui, "所需图形");
//                                 if !self.shapes.is_empty() {
//                                     ui.label(RichText::new(format!("({})", self.shapes.len())).size(7.0).color(TX2));
//                                 }
//                             });

//                             let mut to_remove: Option<usize> = None;
//                             let mut move_up: Option<usize> = None;
//                             let mut move_dn: Option<usize> = None;

//                             egui::ScrollArea::vertical()
//                                 .id_salt("shapes_scroll")
//                                 .max_height(220.0)
//                                 .show(ui, |ui| {
//                                     for (idx, s) in self.shapes.iter_mut().enumerate() {
//                                         let color = gc(idx);
//                                         egui::Frame::none()
//                                             .fill(SF2)
//                                             .stroke(Stroke::new(1.0, BD))
//                                             .show(ui, |ui| {
//                                                 ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
//                                                     // Color strip
//                                                     let (strip_rect, _) = ui.allocate_exact_size(Vec2::new(3.0, 32.0), egui::Sense::hover());
//                                                     ui.painter().rect_filled(strip_rect, Rounding::ZERO, color);

//                                                     ui.add_space(4.0);

//                                                     // Name
//                                                     ui.vertical(|ui| {
//                                                         ui.label(RichText::new("名称").size(7.0).color(TX2).monospace());
//                                                         ui.add(egui::TextEdit::singleline(&mut s.name).desired_width(90.0).font(FontId::monospace(11.0)));
//                                                     });

//                                                     ui.separator();

//                                                     // W
//                                                     ui.vertical(|ui| {
//                                                         ui.label(RichText::new("宽mm").size(7.0).color(TX2).monospace());
//                                                         ui.add(egui::TextEdit::singleline(&mut s.w).desired_width(48.0).font(FontId::monospace(11.0)));
//                                                     });

//                                                     ui.separator();

//                                                     // H
//                                                     ui.vertical(|ui| {
//                                                         ui.label(RichText::new("高mm").size(7.0).color(TX2).monospace());
//                                                         ui.add(egui::TextEdit::singleline(&mut s.h).desired_width(48.0).font(FontId::monospace(11.0)));
//                                                     });

//                                                     // Actions
//                                                     ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
//                                                         if ui.small_button(RichText::new("↓").color(TX2)).clicked() { move_dn = Some(idx); }
//                                                         if ui.small_button(RichText::new("✕").color(ERR)).clicked() { to_remove = Some(idx); }
//                                                         if ui.small_button(RichText::new("↑").color(TX2)).clicked() { move_up = Some(idx); }
//                                                     });
//                                                 });
//                                             });
//                                         ui.add_space(2.0);
//                                     }
//                                 });

//                             if let Some(i) = to_remove { self.shapes.remove(i); }
//                             if let Some(i) = move_up { if i > 0 { self.shapes.swap(i, i - 1); } }
//                             if let Some(i) = move_dn { if i + 1 < self.shapes.len() { self.shapes.swap(i, i + 1); } }

//                             ui.add_space(4.0);
//                             ui.horizontal(|ui| {
//                                 if accent_btn(ui, "＋ 添加").clicked() {
//                                     let id = self.shape_id_cnt;
//                                     self.shape_id_cnt += 1;
//                                     self.shapes.push(ShapeEntry { id, name: format!("图形{}", id), w: "".into(), h: "".into() });
//                                 }
//                                 if danger_btn(ui, "清空").clicked() {
//                                     self.shapes.clear();
//                                 }
//                             });
//                         });

//                         ui.add_space(8.0);

//                         // ── Import ──
//                         egui::Frame::none().inner_margin(margin).show(ui, |ui| {
//                             section_header(ui, "导入数据");
//                             if secondary_btn(ui, "📂 导入 .txt（名称,宽,高 每行）").clicked() {
//                                 let path = rfd::FileDialog::new()
//                                     .add_filter("文本文件", &["txt", "csv"])
//                                     .pick_file();
//                                 if let Some(p) = path {
//                                     if let Ok(content) = std::fs::read_to_string(&p) {
//                                         let mut n = 0;
//                                         for line in content.lines() {
//                                             let t = line.trim();
//                                             if t.is_empty() || t.starts_with('#') { continue; }
//                                             let parts: Vec<&str> = t.split([',', ';', '\t']).map(str::trim).filter(|s| !s.is_empty()).collect();
//                                             if parts.len() >= 3 {
//                                                 self.add_shape(parts[0], parts[1], parts[2]);
//                                                 n += 1;
//                                             } else if parts.len() == 2 {
//                                                 self.add_shape("", parts[0], parts[1]);
//                                                 n += 1;
//                                             }
//                                         }
//                                         if n > 0 {
//                                             self.error_msg = Some(format!("✅ 成功导入 {} 个图形", n));
//                                         }
//                                     }
//                                 }
//                             }
//                         });

//                          ui.add_space(8.0);

//                          egui::Frame::none()
//                              .inner_margin(egui::Margin::symmetric(13.0, 10.0))
//                              .fill(egui::Color32::from_rgb(250, 250, 250))
//                              .show(ui, |ui| {
//                                  let running = self.state == ComputeState::Running;
//                                  ui.add_enabled_ui(!running, |ui| {
//                                      if ui.add_sized(
//                                          [ui.available_width(), 36.0],
//                                          egui::Button::new(egui::RichText::new("▶  开始优化计算").size(13.0).color(egui::Color32::BLACK).strong())
//                                              .fill(ACC), // ACC 需要在上下文中定义
//                                      ).clicked() {
//                                          self.run_compute();
//                                      }
//                                  });

//                                  ui.add_space(4.0);

//                                  ui.horizontal(|ui| {
//                                      if secondary_btn(ui, "示例数据").clicked() {
//                                          self.load_example();
//                                      }
//                                      if secondary_btn(ui, "清除结果").clicked() {
//                                          self.reset_all();
//                                      }
//                                  });

//                                  if let Some(ref msg) = self.error_msg.clone() {
//                                      ui.add_space(4.0);
//                                      let color = if msg.starts_with('✅') { OK } else { ERR }; // OK, ERR 需要定义
//                                      ui.label(egui::RichText::new(msg).size(9.0).color(color).monospace());
//                                  }

//                                  if !self.unfittable.is_empty() {
//                                      ui.add_space(2.0);
//                                      ui.label(egui::RichText::new("⚠ 无法放置:").size(9.0).color(ERR).monospace());
//                                      for s in &self.unfittable {
//                                          ui.label(egui::RichText::new(s).size(8.0).color(TX2).monospace()); // TX2 需要定义
//                                      }
//                                  }
//                              });


//                         ui.add_space(20.0);
//                     });

//                 // Bottom footer actions
//                 // egui::TopBottomPanel::bottom("sidebar_bottom")
//                 //     .frame(egui::Frame {
//                 //         fill: SF,
//                 //         inner_margin: egui::Margin::symmetric(13.0, 10.0),
//                 //         stroke: Stroke::new(1.0, BD),
//                 //         ..Default::default()
//                 //     })
//                 //     .show_inside(ui, |ui| {
//                 //         let running = self.state == ComputeState::Running;
//                 //         ui.add_enabled_ui(!running, |ui| {
//                 //             if ui.add_sized(
//                 //                 [ui.available_width(), 36.0],
//                 //                 egui::Button::new(RichText::new("▶  开始优化计算").size(13.0).color(Color32::BLACK).strong())
//                 //                     .fill(ACC),
//                 //             ).clicked() {
//                 //                 self.run_compute();
//                 //             }
//                 //         });
//                 //         ui.add_space(4.0);
//                 //         ui.horizontal(|ui| {
//                 //             if secondary_btn(ui, "示例数据").clicked() {
//                 //                 self.load_example();
//                 //             }
//                 //             if secondary_btn(ui, "清除结果").clicked() {
//                 //                 self.reset_all();
//                 //             }
//                 //         });

//                 //         if let Some(ref msg) = self.error_msg.clone() {
//                 //             ui.add_space(4.0);
//                 //             let color = if msg.starts_with('✅') { OK } else { ERR };
//                 //             ui.label(RichText::new(msg).size(9.0).color(color).monospace());
//                 //         }
//                 //         if !self.unfittable.is_empty() {
//                 //             ui.add_space(2.0);
//                 //             ui.label(RichText::new("⚠ 无法放置:").size(9.0).color(ERR).monospace());
//                 //             for s in &self.unfittable {
//                 //                 ui.label(RichText::new(s).size(8.0).color(TX2).monospace());
//                 //             }
//                 //         }
//                 //     });

//                 });

//         // Central content
//         egui::CentralPanel::default()
//             .frame(egui::Frame {
//                 fill: BG,
//                 ..Default::default()
//             })
//             .show(ctx, |ui| {
//                 // Step navigation bar
//                 if !self.steps.is_empty() {
//                     egui::TopBottomPanel::top("ctrl_bar")
//                         .frame(egui::Frame {
//                             fill: SF,
//                             inner_margin: egui::Margin::symmetric(16.0, 8.0),
//                             stroke: Stroke::new(1.0, BD),
//                             ..Default::default()
//                         })
//                         .show_inside(ui, |ui| {
//                             ui.horizontal(|ui| {
//                                 if secondary_btn(ui, "⏮").clicked() { self.cur_step = 0; }
//                                 if ui.add_enabled(self.cur_step > 0, egui::Button::new(RichText::new("◀ 上一步").monospace().size(9.0))).clicked() {
//                                     if self.cur_step > 0 { self.cur_step -= 1; }
//                                 }

//                                 // Progress bar
//                                 let pct = if self.steps.len() > 1 {
//                                     self.cur_step as f32 / (self.steps.len() - 1) as f32
//                                 } else { 1.0 };
//                                 let (bar_rect, _) = ui.allocate_exact_size(
//                                     Vec2::new(ui.available_width() - 160.0, 4.0),
//                                     egui::Sense::hover(),
//                                 );
//                                 ui.painter().rect_filled(bar_rect, Rounding::same(2.0), BD);
//                                 let fill_w = bar_rect.width() * pct;
//                                 ui.painter().rect_filled(
//                                     Rect::from_min_size(bar_rect.min, Vec2::new(fill_w, bar_rect.height())),
//                                     Rounding::same(2.0),
//                                     ACC,
//                                 );

//                                 if ui.add_enabled(self.cur_step < self.steps.len() - 1, egui::Button::new(RichText::new("下一步 ▶").monospace().size(9.0))).clicked() {
//                                     self.cur_step += 1;
//                                 }
//                                 if secondary_btn(ui, "⏭").clicked() { self.cur_step = self.steps.len() - 1; }

//                                 ui.label(
//                                     RichText::new(format!("步骤 {}/{}", self.cur_step + 1, self.steps.len()))
//                                         .monospace()
//                                         .size(8.0)
//                                         .color(TX2),
//                                 );
//                             });
//                         });
//                 }

//                 egui::ScrollArea::vertical()
//                     .id_salt("content_scroll")
//                     .show(ui, |ui| {
//                         ui.add_space(12.0);

//                         if self.steps.is_empty() {
//                             // Empty state
//                             ui.add_space(80.0);
//                             ui.vertical_centered(|ui| {
//                                 ui.label(RichText::new("🪵").size(48.0));
//                                 ui.add_space(10.0);
//                                 ui.label(RichText::new("选择算法，配置木板与图形").size(11.0).color(TX2).monospace());
//                                 ui.label(RichText::new("点击「开始优化计算」").size(11.0).color(TX2).monospace());
//                                 ui.add_space(6.0);
//                                 ui.label(RichText::new("GA · SA · SVGNest · NFP · MaxRects · Guillotine · BL").size(9.0).color(ACC).monospace());
//                             });
//                             return;
//                         }

//                         let step = &self.steps[self.cur_step];
//                         let snap = step.snap.clone();
//                         let hl = step.highlight;
//                         let boards = self.solution_boards.clone();
//                         let bw = self.solution_bw;
//                         let bh = self.solution_bh;

//                         // Step log
//                         let lm = egui::Margin::symmetric(16.0, 0.0);
//                         egui::Frame::none().inner_margin(lm).show(ui, |ui| {
//                             egui::Frame::none()
//                                 .fill(SF2)
//                                 .stroke(Stroke::new(1.0, BD))
//                                 .inner_margin(egui::Margin { left: 12.0, right: 12.0, top: 8.0, bottom: 8.0 })
//                                 .show(ui, |ui| {
//                                     // Left accent bar
//                                     let (bar, _) = ui.allocate_exact_size(Vec2::new(3.0, 1.0), egui::Sense::hover());
//                                     // Draw it via painter after frame
//                                     ui.label(RichText::new(&step.msg).monospace().size(9.0).color(TX));
//                                 });
//                         });

//                         ui.add_space(8.0);

//                         // Convergence chart
//                         let gen_history = self.gen_history.clone();
//                         if gen_history.len() > 2 && self.cur_step == self.steps.len() - 1 {
//                             egui::Frame::none().inner_margin(egui::Margin::symmetric(16.0, 0.0)).show(ui, |ui| {
//                                 egui::Frame::none()
//                                     .fill(SF2)
//                                     .stroke(Stroke::new(1.0, BD))
//                                     .inner_margin(egui::Margin::same(8.0))
//                                     .show(ui, |ui| {
//                                         ui.label(RichText::new("收敛曲线 — 适应度 vs 代/步数（越低越好）").size(8.0).color(TX2).monospace());
//                                         let (chart_rect, _) = ui.allocate_exact_size(
//                                             Vec2::new(ui.available_width(), 80.0),
//                                             egui::Sense::hover(),
//                                         );
//                                         draw_convergence_chart(ui.painter(), chart_rect, &gen_history);
//                                     });
//                             });
//                             ui.add_space(8.0);
//                         }

//                         // Boards
//                         for (bi, board) in boards.iter().enumerate() {
//                             let count = *snap.get(bi).unwrap_or(&0);

//                             // Skip boards not yet visible
//                             if count == 0 && bi > step.board_idx { continue; }

//                             let used: f64 = board.placements.iter().take(count).map(|p| p.w * p.h).sum();
//                             let util = used / (bw * bh) * 100.0;
//                             let is_active = bi == step.board_idx;

//                             let card_stroke = if is_active { Stroke::new(1.0, ACC) } else { Stroke::new(1.0, BD) };

//                             egui::Frame::none()
//                                 .inner_margin(egui::Margin::symmetric(16.0, 0.0))
//                                 .show(ui, |ui| {
//                                     egui::Frame::none()
//                                         .fill(SF)
//                                         .stroke(card_stroke)
//                                         .show(ui, |ui| {
//                                             // Card header
//                                             egui::Frame::none()
//                                                 .fill(SF2)
//                                                 .stroke(Stroke::new(1.0, BD))
//                                                 .inner_margin(egui::Margin::symmetric(10.0, 6.0))
//                                                 .show(ui, |ui| {
//                                                     ui.horizontal(|ui| {
//                                                         ui.label(RichText::new(format!("木板 #{}", bi + 1)).size(9.0).color(ACC).monospace().strong());
//                                                         ui.add_space(8.0);

//                                                         // Utilization bar
//                                                         let (bar_rect, _) = ui.allocate_exact_size(Vec2::new(50.0, 2.0), egui::Sense::hover());
//                                                         ui.painter().rect_filled(bar_rect, Rounding::same(1.0), BD);
//                                                         if count == board.placements.len() && count > 0 {
//                                                             let fw = bar_rect.width() * (util / 100.0) as f32;
//                                                             ui.painter().rect_filled(
//                                                                 Rect::from_min_size(bar_rect.min, Vec2::new(fw, bar_rect.height())),
//                                                                 Rounding::same(1.0),
//                                                                 OK,
//                                                             );
//                                                         }

//                                                         ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
//                                                             ui.label(RichText::new(format!("{}×{}mm", bw, bh)).size(8.0).color(TX2).monospace());
//                                                             ui.add_space(8.0);
//                                                             ui.label(RichText::new(
//                                                                 if count == board.placements.len() { format!("{:.1}%", util) } else { "—".into() }
//                                                             ).size(8.0).color(TX2).monospace());
//                                                             ui.add_space(8.0);
//                                                             ui.label(RichText::new(format!("{}/{} 件", count, board.placements.len())).size(8.0).color(TX2).monospace());
//                                                         });
//                                                     });
//                                                 });

//                                             // Board canvas
//                                             egui::Frame::none()
//                                                 .inner_margin(egui::Margin::same(9.0))
//                                                 .show(ui, |ui| {
//                                                     let avail_w = ui.available_width();
//                                                     let scale = (avail_w as f64 / bw).min(320.0 / bh).min(0.3);
//                                                     let canvas_w = (bw * scale) as f32;
//                                                     let canvas_h = (bh * scale) as f32;

//                                                     let (canvas_rect, canvas_resp) = ui.allocate_exact_size(
//                                                         Vec2::new(canvas_w, canvas_h),
//                                                         egui::Sense::hover(),
//                                                     );

//                                                     let hl_for_board = if let Some((hi, hx, hy, hw, hh)) = hl {
//                                                         if hi == bi { Some((hx, hy, hw, hh)) } else { None }
//                                                     } else {
//                                                         None
//                                                     };

//                                                     draw_board_canvas(
//                                                         ui.painter(),
//                                                         canvas_rect,
//                                                         board,
//                                                         count,
//                                                         bw,
//                                                         bh,
//                                                         hl_for_board,
//                                                     );

//                                                     // Tooltip on hover
//                                                     if let Some(hover_pos) = canvas_resp.hover_pos() {
//                                                         let scale = (canvas_rect.width() as f64 / bw).min(canvas_rect.height() as f64 / bh);
//                                                         let mx = (hover_pos.x - canvas_rect.min.x) as f64 / scale;
//                                                         let my = (hover_pos.y - canvas_rect.min.y) as f64 / scale;

//                                                         let mut tip_text: Option<String> = None;
//                                                         for p in board.placements.iter().take(count) {
//                                                             if mx >= p.x && mx <= p.x + p.w && my >= p.y && my <= p.y + p.h {
//                                                                 tip_text = Some(format!(
//                                                                     "名称: {}\n尺寸: {}×{}mm{}\n位置: ({}, {})\n面积: {:.4}m²",
//                                                                     p.name, p.w, p.h,
//                                                                     if p.rotated { " [旋转]" } else { "" },
//                                                                     p.x.round() as i64, p.y.round() as i64,
//                                                                     p.w * p.h / 1e6
//                                                                 ));
//                                                                 break;
//                                                             }
//                                                         }
//                                                         if let Some(tip) = tip_text {
//                                                             canvas_resp.on_hover_text(
//                                                                 egui::RichText::new(tip).monospace().size(10.0)
//                                                             );
//                                                         }
//                                                     }

//                                                     // Legend
//                                                     egui::Frame::none().show(ui, |ui| {
//                                                         ui.horizontal_wrapped(|ui| {
//                                                             for p in board.placements.iter().take(count) {
//                                                                 let c = gc(p.color_idx);
//                                                                 ui.horizontal(|ui| {
//                                                                     let (dot, _) = ui.allocate_exact_size(Vec2::new(7.0, 7.0), egui::Sense::hover());
//                                                                     ui.painter().rect_filled(dot, Rounding::same(1.0), c);
//                                                                     ui.label(RichText::new(&p.name).size(7.0).color(TX2).monospace());
//                                                                 });
//                                                             }
//                                                         });
//                                                     });
//                                                 });
//                                         });
//                                 });

//                             ui.add_space(10.0);
//                         }

//                         ui.add_space(20.0);
//                     });
//             });
//     }
// }

// // ─────────────────────────────────────────────
// //  UI helpers
// // ─────────────────────────────────────────────
// fn section_header(ui: &mut egui::Ui, text: &str) {
//     ui.label(RichText::new(text).size(8.0).color(ACC).monospace().strong());
//     ui.add(egui::Separator::default().spacing(5.0));
//     ui.add_space(3.0);
// }

// fn field_row(ui: &mut egui::Ui, label: &str, value: &mut String) {
//     ui.label(RichText::new(label).size(9.0).color(TX2).monospace());
//     ui.add(egui::TextEdit::singleline(value).desired_width(80.0).font(FontId::monospace(11.0)));
// }

// fn accent_btn(ui: &mut egui::Ui, label: &str) -> egui::Response {
//     ui.add(egui::Button::new(RichText::new(label).monospace().size(9.0).color(Color32::BLACK)).fill(ACC))
// }

// fn secondary_btn(ui: &mut egui::Ui, label: &str) -> egui::Response {
//     ui.add(egui::Button::new(RichText::new(label).monospace().size(9.0).color(TX2)).fill(Color32::TRANSPARENT).stroke(Stroke::new(1.0, BD)))
// }

// fn danger_btn(ui: &mut egui::Ui, label: &str) -> egui::Response {
//     ui.add(egui::Button::new(RichText::new(label).monospace().size(9.0).color(ERR)).fill(Color32::TRANSPARENT).stroke(Stroke::new(1.0, Color32::from_rgb(122, 37, 32))))
// }

// // ─────────────────────────────────────────────
// //  Entry Point
// // ─────────────────────────────────────────────
// fn main() -> eframe::Result<()> {
//     let native_options = eframe::NativeOptions {
//         viewport: egui::ViewportBuilder::default()
//             .with_title("木板分割优化 Pro")
//             .with_inner_size([1200.0, 800.0])
//             .with_min_inner_size([900.0, 600.0])
//             .with_icon(load_icon()),
//         ..Default::default()
//     };

//     eframe::run_native(
//         "木板分割优化",
//         native_options,
//         Box::new(|cc| {
//             // Load CJK-capable font
//             setup_fonts(&cc.egui_ctx);
//             Ok(Box::new(WoodCutterApp::default()))
//         }),
//     )
// }

// fn load_icon() -> egui::IconData {
//     // Simple 32×32 wood-colored icon
//     let size = 32usize;
//     let mut pixels = Vec::with_capacity(size * size * 4);
//     for y in 0..size {
//         for x in 0..size {
//             let in_circle = {
//                 let cx = x as f32 - 16.0;
//                 let cy = y as f32 - 16.0;
//                 cx * cx + cy * cy <= 14.0 * 14.0
//             };
//             if in_circle {
//                 pixels.extend_from_slice(&[212, 168, 83, 255]);
//             } else {
//                 pixels.extend_from_slice(&[0, 0, 0, 0]);
//             }
//         }
//     }
//     egui::IconData {
//         rgba: pixels,
//         width: size as u32,
//         height: size as u32,
//     }
// }

// fn setup_fonts(ctx: &egui::Context) {
//     let mut fonts = egui::FontDefinitions::default();

//     // Add Noto Sans SC or fallback to built-in — we embed a compact CJK font
//     // For production, embed actual font bytes; here we rely on system fallback
//     // If you have a font file: include_bytes!("../assets/SourceHanSerifCN-Medium-6.otf")
//     fonts.font_data.insert("noto_cjk".into(), egui::FontData::from_static(include_bytes!("../assets/SourceHanSerifCN-Regular-1.otf")));
//     fonts.families
//         .entry(egui::FontFamily::Proportional)
//         .or_insert_with(Vec::new)
//         .insert(0, "noto_cjk".into());

//     fonts.families
//         .entry(egui::FontFamily::Monospace)
//         .or_insert_with(Vec::new)
//         .insert(0, "noto_cjk".into());

//     ctx.set_fonts(fonts);
// }


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
