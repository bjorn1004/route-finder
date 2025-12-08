use super::GuiApp;
use crate::get_orders;
use egui::{Color32, Pos2, Sense, Stroke, Ui};

pub fn show_center_panel(ctx: &egui::Context, ui: &mut Ui, app: &mut GuiApp) {
    let (response, painter) = ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

    app.camera.translation += response.drag_delta();

    // Zoom handling: pinch (touch) and scroll wheel.
    {
        let mut scale_factor: f32 = 1.0;

        let pinch = ctx.input(|i| i.zoom_delta());
        if (pinch - 1.0).abs() > f32::EPSILON {
            scale_factor *= pinch;
        }

        let scroll = ctx.input(|i| i.smooth_scroll_delta.y);
        if scroll.abs() > f32::EPSILON {
            let scroll_sensitivity = 0.0025;
            scale_factor *= (1.0 - scroll * scroll_sensitivity).clamp(0.001, 10.0);
        }

        if (scale_factor - 1.0).abs() > f32::EPSILON {
            let screen_pos = response.hover_pos().unwrap_or(response.rect.center());
            let old_scale = app.camera.scaling;
            let world_x = (screen_pos.x - app.camera.translation.x) / old_scale;
            let world_y = (screen_pos.y - app.camera.translation.y) / old_scale;

            let new_scale = (old_scale * scale_factor).clamp(1e-8, 1e8);
            app.camera.scaling = new_scale;

            app.camera.translation = egui::Vec2::new(
                screen_pos.x - world_x * new_scale,
                screen_pos.y - world_y * new_scale,
            );
        }
    }

    if let Some(route_rec) = &app.route_rec
        && let Ok(cur_route) = route_rec.try_recv()
    {
        app.cur_route = Some(cur_route);
    }

    let mut routes = vec![];
    if let Some(route) = &app.cur_route {
        for selection in app.route_selection.iter() {
            let sel_route = super::route_selection_to_route(route, selection);
            routes.push(sel_route);
        }
    }

    let orders = get_orders();
    let route_lines = routes.iter().map(|route| {
        egui::Shape::line(
            route
                .linked_vector
                .iter()
                .map(|(_, order_index)| {
                    let order = &orders[*order_index];
                    app.camera * Pos2::new(order.x_coordinate as f32, order.y_coordinate as f32)
                })
                .collect(),
            Stroke::new(1.0, Color32::LIGHT_BLUE),
        )
    });

    painter.extend(route_lines);
    let shapes = get_orders().iter().map(|o| {
        let screen_pos = app.camera * Pos2::new(o.x_coordinate as f32, o.y_coordinate as f32);
        let (colour, radius) = match o.matrix_id {
            287 => (Color32::GREEN, 3.5), // Maarheeze, the dump site
            _ => (Color32::BLUE, 2.0),
        };
        egui::Shape::circle_filled(screen_pos, radius, colour)
    });

    painter.extend(shapes);
}
