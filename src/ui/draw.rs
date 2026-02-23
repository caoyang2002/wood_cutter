use eframe::egui::{self, Color32, FontId, Painter, Pos2, Rect, Rounding, Stroke, Vec2};
use crate::core::Board;
use crate::ui::theme::gc;
use egui::Align2;
use egui::Shape;

// 绘制木板
pub fn draw_board_canvas(
    painter: &Painter,
    rect: Rect,
    board: &Board,
    count: usize,
    bw: f64,
    bh: f64,
    highlight: Option<(f64, f64, f64, f64)>,
) {
    let scale_x = rect.width() as f64 / bw;
    let scale_y = rect.height() as f64 / bh;
    let scale = scale_x.min(scale_y);
    let ox = rect.min.x;
    let oy = rect.min.y;

    // Background: dark wood color
    painter.rect_filled(rect, Rounding::same(2.0), Color32::from_rgb(26, 15, 4));

    // Subtle wood grain lines
    let grain_color = Color32::from_rgba_unmultiplied(130, 70, 15, 8);
    let mut gy = rect.min.y;
    while gy < rect.max.y {
        painter.line_segment(
            [Pos2::new(rect.min.x, gy), Pos2::new(rect.max.x, gy)],
            Stroke::new(0.5, grain_color),
        );
        gy += 5.0;
    }

    // Grid lines at 100mm intervals
    let grid_color = Color32::from_rgba_unmultiplied(255, 255, 255, 8);
    let step_px = (scale * 100.0) as f32;
    let mut gx = rect.min.x;
    while gx < rect.max.x {
        painter.line_segment(
            [Pos2::new(gx, rect.min.y), Pos2::new(gx, rect.max.y)],
            Stroke::new(0.3, grid_color),
        );
        gx += step_px;
    }
    let mut gy = rect.min.y;
    while gy < rect.max.y {
        painter.line_segment(
            [Pos2::new(rect.min.x, gy), Pos2::new(rect.max.x, gy)],
            Stroke::new(0.3, grid_color),
        );
        gy += step_px;
    }

    // Draw placements
    for p in board.placements.iter().take(count) {
        let px = ox + (p.x * scale) as f32;
        let py = oy + (p.y * scale) as f32;
        let pw = (p.w * scale) as f32;
        let ph = (p.h * scale) as f32;

        let color = gc(p.color_idx);
        let fill = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 220);
        let piece_rect = Rect::from_min_size(Pos2::new(px, py), Vec2::new(pw, ph));

        let is_hl = if let Some((hx, hy, _hw, _hh)) = highlight {
            (p.x - hx).abs() < 0.5 && (p.y - hy).abs() < 0.5
        } else {
            false
        };

        painter.rect_filled(piece_rect, Rounding::same(1.0), fill);

        // Inner texture lines
        if ph > 6.0 {
            let texture_color = Color32::from_rgba_unmultiplied(255, 255, 255, 12);
            let mut ty = py + 7.0;
            while ty < py + ph {
                painter.line_segment(
                    [Pos2::new(px, ty), Pos2::new(px + pw, ty)],
                    Stroke::new(0.4, texture_color),
                );
                ty += 7.0;
            }
        }

        // Border
        if is_hl {
            painter.rect_stroke(
                piece_rect,
                Rounding::same(1.0),
                Stroke::new(2.0, Color32::WHITE),
            );
        } else {
            painter.rect_stroke(
                piece_rect,
                Rounding::same(1.0),
                Stroke::new(0.6, Color32::from_rgba_unmultiplied(255, 255, 255, 55)),
            );
        }

        // Label
        if pw > 14.0 && ph > 9.0 {
            let font_size = (pw / 5.0).min(ph / 2.2).min(11.0).max(6.0);
            let cx = px + pw / 2.0;
            let cy = py + ph / 2.0;
            painter.text(
                Pos2::new(cx, cy),
                egui::Align2::CENTER_CENTER,
                &p.name,
                FontId::monospace(font_size),
                Color32::from_rgba_unmultiplied(255, 255, 255, 230),
            );
            if ph > font_size * 2.8 && font_size >= 7.0 {
                painter.text(
                    Pos2::new(cx, cy + font_size + 2.0),
                    egui::Align2::CENTER_CENTER,
                    &format!("{}×{}", p.w, p.h),
                    FontId::monospace((font_size - 2.0).max(5.0)),
                    Color32::from_rgba_unmultiplied(255, 255, 255, 90),
                );
            }
            if p.rotated && pw > 12.0 {
                painter.text(
                    Pos2::new(px + 3.0, py + 2.0),
                    egui::Align2::LEFT_TOP,
                    "↻",
                    FontId::proportional(9.0),
                    Color32::from_rgb(255, 220, 60),
                );
            }
        }

        // Highlight accent border
        if is_hl {
            let acc = Color32::from_rgb(212, 168, 83);
            painter.rect_stroke(
                piece_rect.expand(1.5),
                Rounding::same(2.0),
                Stroke::new(2.0, acc),
            );
        }
    }

    // Outer board border
    painter.rect_stroke(
        rect,
        Rounding::same(2.0),
        Stroke::new(1.5, Color32::from_rgb(72, 72, 72)),
    );
}


// 绘制收敛曲线
// 绘制收敛曲线 - 带背景、间距和关键转折标记
pub fn draw_convergence_chart(painter: &Painter, rect: Rect, history: &[f64]) {
    if history.len() < 2 {
        return;
    }

    // === 1. 绘制背景 ===
    painter.rect_filled(rect, Rounding::same(4.0), Color32::from_rgb(28, 28, 28));

    // 添加内阴影效果（浅色边框）
    painter.rect_stroke(
        rect,
        Rounding::same(4.0),
        Stroke::new(0.5, Color32::from_rgba_unmultiplied(80, 80, 80, 100)),
    );

    let min_val = history.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = history.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max_val - min_val).max(1e-9);

    // 内边距 - 为数值标签留出空间
    let pad = 16.0_f32;
    let label_pad = 24.0_f32; // 左侧为数值标签留出的额外空间

    let w = rect.width() - label_pad - pad;
    let h = rect.height() - 2.0 * pad;

    // 图表实际绘制区域
    let chart_rect = Rect::from_min_size(
        Pos2::new(rect.min.x + label_pad, rect.min.y + pad),
        Vec2::new(w, h),
    );

    // === 2. 绘制背景网格 ===
    let grid_color = Color32::from_rgba_unmultiplied(80, 80, 80, 60);
    let text_color = Color32::from_rgba_unmultiplied(150, 150, 150, 180);

    // 水平网格线（5条）
    for i in 0..=5 {
        let y = chart_rect.min.y + (i as f32 / 5.0) * chart_rect.height();

        // 网格线
        painter.line_segment(
            [
                Pos2::new(chart_rect.min.x, y),
                Pos2::new(chart_rect.max.x, y),
            ],
            Stroke::new(0.3, grid_color),
        );

        // Y轴数值标签
        let val = max_val - (i as f64 / 5.0) * range;
        painter.text(
            Pos2::new(rect.min.x + 4.0, y - 6.0),
            Align2::LEFT_TOP,
            &format!("{:.2}", val),
            FontId::monospace(7.0),
            text_color,
        );
    }

    // 垂直网格线（根据数据点数量动态调整）
    let num_grid = (history.len() / 5).max(5).min(20);
    for i in 0..=num_grid {
        let x = chart_rect.min.x + (i as f32 / num_grid as f32) * chart_rect.width();

        // 网格线
        painter.line_segment(
            [
                Pos2::new(x, chart_rect.min.y),
                Pos2::new(x, chart_rect.max.y),
            ],
            Stroke::new(0.3, grid_color),
        );

        // X轴标签（迭代次数）
        if i % 2 == 0 {  // 隔一个显示一个，避免太密集
            let iter_idx = (i as f64 / num_grid as f64 * (history.len() - 1) as f64).round() as usize;
            painter.text(
                Pos2::new(x - 10.0, chart_rect.max.y + 8.0),
                Align2::LEFT_TOP,
                &format!("{}", iter_idx),
                FontId::monospace(7.0),
                text_color,
            );
        }
    }

    // 坐标轴标签
    painter.text(
        Pos2::new(chart_rect.max.x - 30.0, chart_rect.min.y - 12.0),
        Align2::LEFT_TOP,
        "适应度",
        FontId::monospace(8.0),
        Color32::from_rgba_unmultiplied(180, 180, 180, 220),
    );

    painter.text(
        Pos2::new(chart_rect.max.x - 30.0, chart_rect.max.y + 15.0),
        Align2::LEFT_TOP,
        "迭代次数",
        FontId::monospace(8.0),
        Color32::from_rgba_unmultiplied(180, 180, 180, 220),
    );

    // 坐标转换函数
    let to_pos = |i: usize, v: f64| -> Pos2 {
        Pos2::new(
            chart_rect.min.x + (i as f32 / (history.len() - 1) as f32) * chart_rect.width(),
            chart_rect.min.y + (1.0 - ((v - min_val) / range) as f32) * chart_rect.height(),
        )
    };

    // === 3. 绘制曲线 ===
    let curve_pts: Vec<Pos2> = history.iter().enumerate().map(|(i, &v)| to_pos(i, v)).collect();
    painter.add(Shape::line(curve_pts, Stroke::new(2.0, Color32::from_rgb(212, 168, 83))));

    // === 4. 标记关键转折点 ===
    let marker_color = Color32::from_rgb(255, 200, 100);
    let highlight_color = Color32::from_rgb(255, 100, 100);

    // 使用 String 而不是 &str 来避免生命周期问题
    #[derive(Clone)]
    struct TurningPoint {
        idx: usize,
        val: f64,
        label: String,
    }

    let mut turning_points = Vec::new();

    // 起点
    turning_points.push(TurningPoint {
        idx: 0,
        val: history[0],
        label: "起点".to_string(),
    });

    // 终点
    turning_points.push(TurningPoint {
        idx: history.len() - 1,
        val: history[history.len() - 1],
        label: "终点".to_string(),
    });

    // 最高点
    if let Some(max_idx) = history.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
    {
        turning_points.push(TurningPoint {
            idx: max_idx,
            val: history[max_idx],
            label: "最高".to_string(),
        });
    }

    // 最低点
    if let Some(min_idx) = history.iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
    {
        turning_points.push(TurningPoint {
            idx: min_idx,
            val: history[min_idx],
            label: "最低".to_string(),
        });
    }

    // 检测明显的转折点（变化率大的点）
    if history.len() > 10 {
        let window_size = 3;
        for i in window_size..history.len() - window_size {
            let left_avg = (history[i-3] + history[i-2] + history[i-1]) / 3.0;
            let right_avg = (history[i+1] + history[i+2] + history[i+3]) / 3.0;
            let change = (right_avg - left_avg).abs();

            // 如果变化率大于平均变化率的2倍，标记为转折点
            if change > (range * 0.1) {
                turning_points.push(TurningPoint {
                    idx: i,
                    val: history[i],
                    label: format!("Δ{:.2}", change),
                });
            }
        }
    }

    // 去重（保留最近的标记）
    turning_points.sort_by(|a, b| a.idx.cmp(&b.idx));
    turning_points.dedup_by(|a, b| a.idx == b.idx);

    // 绘制标记点
    for point in turning_points {
        let pos = to_pos(point.idx, point.val);

        // 绘制标记圆点
        let is_highlight = point.idx == 0
            || point.idx == history.len() - 1
            || point.label == "最高"
            || point.label == "最低";

        let color = if is_highlight { highlight_color } else { marker_color };
        let size = if is_highlight { 5.0 } else { 3.0 };

        painter.circle_filled(pos, size, color);
        painter.circle_stroke(pos, size + 1.0, Stroke::new(1.0, Color32::WHITE));

        // 绘制数值标签
        let label_pos = if point.idx < history.len() / 2 {
            Pos2::new(pos.x + 8.0, pos.y - 8.0)
        } else {
            Pos2::new(pos.x - 45.0, pos.y - 8.0)  // 调宽一点以容纳更长的标签
        };

        // 标签背景
        let label_bg_rect = Rect::from_min_size(
            label_pos,
            Vec2::new(42.0, 14.0),  // 调宽一点以容纳更长的标签
        );
        painter.rect_filled(
            label_bg_rect,
            Rounding::same(2.0),
            Color32::from_rgba_unmultiplied(20, 20, 20, 200),
        );
        painter.rect_stroke(
            label_bg_rect,
            Rounding::same(2.0),
            Stroke::new(0.5, color),
        );

        // 标签文字
        painter.text(
            Pos2::new(label_pos.x + 2.0, label_pos.y + 2.0),
            Align2::LEFT_TOP,
            &format!("{}:{:.2}", point.label, point.val),
            FontId::monospace(7.0),
            Color32::WHITE,
        );
    }

    // === 5. 添加统计信息 ===
    let stats_y = rect.min.y + 5.0;
    painter.text(
        Pos2::new(rect.max.x - 100.0, stats_y),
        Align2::LEFT_TOP,
        &format!("数据点: {}", history.len()),
        FontId::monospace(7.0),
        Color32::from_rgba_unmultiplied(150, 150, 150, 200),
    );

    // 计算收敛率（最后10%的平均变化）
    let converge_window = (history.len() / 10).max(5);
    if history.len() > converge_window * 2 {
        let early_avg = history[0..converge_window].iter().sum::<f64>() / converge_window as f64;
        let late_avg = history[history.len()-converge_window..].iter().sum::<f64>() / converge_window as f64;
        let converge_rate = (early_avg - late_avg) / early_avg * 100.0;

        painter.text(
            Pos2::new(rect.max.x - 100.0, stats_y + 12.0),
            Align2::LEFT_TOP,
            &format!("收敛率: {:.1}%", converge_rate),
            FontId::monospace(7.0),
            if converge_rate > 0.0 {
                Color32::from_rgb(100, 200, 100)
            } else {
                Color32::from_rgb(200, 100, 100)
            },
        );
    }
}
