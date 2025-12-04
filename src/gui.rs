use egui::{Color32, Pos2, Sense, Vec2, emath::TSTransform, epaint::CircleShape};

use crate::get_orders;

pub struct GuiApp {
    pub camera: TSTransform,
}

impl GuiApp {
    pub fn new() -> Self {
        let min_x = get_orders()
            .iter()
            .fold(u32::MAX, |a, o| o.x_coordinate.min(a));
        let min_y = get_orders()
            .iter()
            .fold(u32::MAX, |a, o| o.y_coordinate.min(a));

        Self {
            camera: TSTransform {
                scaling: 0.0001,
                translation: -Vec2::new(min_x as f32, min_y as f32) * 0.0001,
            },
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("overview").show(ctx, |ui| {});
        egui::CentralPanel::default().show(ctx, |ui| {
            let (mut response, painter) =
                ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

            // Doesn't work for now, so the dots spawn in a little off screen
            // let to_screen = emath::RectTransform::from_to(
            //     Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            //     response.rect,
            // );

            self.camera.translation += response.drag_delta();

            // Zoom handling: pinch (touch) and scroll wheel.
            // We compute a scale factor from pinch/scroll and zoom the camera
            // around the current pointer (or center if none).
            {
                // Start with neutral scale factor
                let mut scale_factor: f32 = 1.0;

                // Pinch/zoom gesture (multitouch). Many egui versions expose
                // `ctx.input().zoom_delta` as a multiplicative factor (1.0 == no change).
                // If your egui version provides this as a method instead, adjust accordingly.
                // Using field access is broadly compatible with common egui releases.
                let pinch = ctx.input(|i| i.zoom_delta());
                if (pinch - 1.0).abs() > f32::EPSILON {
                    scale_factor *= pinch;
                }

                // Scroll wheel: typically ctx.input().scroll_delta.y is the vertical scroll.
                // We convert this into a multiplicative zoom factor. Tweak sensitivity as needed.
                let scroll = ctx.input(|i| i.smooth_scroll_delta.y);
                if scroll.abs() > f32::EPSILON {
                    // smaller multiplier = less sensitive. Negative scroll (wheel up) usually zooms in.
                    // Use an exponential mapping for smooth zooming.
                    let scroll_sensitivity = 0.0025;
                    scale_factor *= (1.0 - scroll * scroll_sensitivity).clamp(0.001, 10.0);
                }

                if (scale_factor - 1.0).abs() > f32::EPSILON {
                    // Determine the screen position to zoom around: pointer hover or center of the response rect.
                    let screen_pos = response.hover_pos().unwrap_or(response.rect.center());

                    // Compute the world position under the cursor before scaling changes.
                    let old_scale = self.camera.scaling;
                    let world_x = (screen_pos.x - self.camera.translation.x) / old_scale;
                    let world_y = (screen_pos.y - self.camera.translation.y) / old_scale;

                    // Apply scale factor, clamped to reasonable bounds.
                    let new_scale = (old_scale * scale_factor).clamp(1e-8, 1e8);
                    self.camera.scaling = new_scale;

                    // Recompute translation so the world point remains under the same screen pixel.
                    self.camera.translation = Vec2::new(
                        screen_pos.x - world_x * new_scale,
                        screen_pos.y - world_y * new_scale,
                    );
                }
            }

            let shapes = get_orders().iter().map(|o| {
                let screen_pos =
                    self.camera * Pos2::new(o.x_coordinate as f32, o.y_coordinate as f32);
                egui::Shape::Circle(CircleShape::filled(screen_pos, 2.0, Color32::BLUE))
            });

            painter.extend(shapes);
        });
    }
}
