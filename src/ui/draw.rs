use eframe::egui::{self, Color32, FontId, Painter, Pos2, Rect, Rounding, Stroke, Vec2};
use crate::core::Board;
use crate::ui::theme::gc;

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


pub fn draw_convergence_chart(painter: &Painter, rect: Rect, history: &[f64]) {
    if history.len() < 2 {
        return;
    }
    painter.rect_filled(rect, Rounding::same(2.0), Color32::from_rgb(28, 28, 28));

    let min = history.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = history.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max - min).max(1e-9);
    let pad = 12.0_f32;
    let w = rect.width() - 2.0 * pad;
    let h = rect.height() - 2.0 * pad;

    let to_pos = |i: usize, v: f64| -> Pos2 {
        Pos2::new(
            rect.min.x + pad + (i as f32 / (history.len() - 1) as f32) * w,
            rect.min.y + pad + (1.0 - ((v - min) / range) as f32) * h,
        )
    };

    // Fill area
    let mut fill_pts: Vec<Pos2> = history.iter().enumerate().map(|(i, &v)| to_pos(i, v)).collect();
    fill_pts.push(to_pos(history.len() - 1, min));
    fill_pts.push(to_pos(0, min));
    painter.add(egui::Shape::convex_polygon(
        fill_pts,
        Color32::from_rgba_unmultiplied(212, 168, 83, 30),
        Stroke::NONE,
    ));

    // Line
    let pts: Vec<Pos2> = history.iter().enumerate().map(|(i, &v)| to_pos(i, v)).collect();
    painter.add(egui::Shape::line(pts, Stroke::new(1.5, Color32::from_rgb(212, 168, 83))));

    // Labels
    painter.text(
        Pos2::new(rect.min.x + pad + 2.0, rect.min.y + pad + 3.0),
        egui::Align2::LEFT_TOP,
        &format!("最高:{:.3}", max),
        FontId::monospace(9.0),
        Color32::from_rgba_unmultiplied(122, 106, 88, 200),
    );
    painter.text(
        Pos2::new(rect.min.x + pad + 2.0, rect.max.y - pad - 3.0),
        egui::Align2::LEFT_BOTTOM,
        &format!("最低:{:.3}", min),
        FontId::monospace(9.0),
        Color32::from_rgba_unmultiplied(122, 106, 88, 200),
    );
}
