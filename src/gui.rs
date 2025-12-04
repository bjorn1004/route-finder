use std::collections::HashMap;

use egui::{Color32, Pos2, Sense, Stroke, Vec2, emath::TSTransform};

use crate::{
    datastructures::linked_vectors::LinkedVector, get_orders, resource::MatrixID,
    simulated_annealing::route::Route,
};

pub struct GuiApp {
    pub camera: TSTransform,
    pub matrix_coordinate_map: HashMap<MatrixID, Pos2>,
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
            // pre-fill a matrix to coordinate system so we don't have to search the list for every point
            matrix_coordinate_map: get_orders().iter().fold(HashMap::new(), |mut acc, o| {
                acc.insert(
                    o.matrix_id,
                    Pos2::new(o.x_coordinate as f32, o.y_coordinate as f32),
                );
                acc
            }),
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.label("Here will be controls for starting searches, tweaking params, etc...")
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
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

                let pinch = ctx.input(|i| i.zoom_delta());
                if (pinch - 1.0).abs() > f32::EPSILON {
                    scale_factor *= pinch;
                }

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

            // TODO: Get the routes from the simulated annealing (with minimum overhead).
            // Furthermore, if we draw all routes, from both trucks, for all days, at once,
            // you would get a massive unintelligable spider web
            // so we need a way to select specific routes
            let mut temp_route = Route::new();
            for o in get_orders().iter() {
                temp_route.linked_vector.push_front(o.matrix_id);
            }
            temp_route.linked_vector.compact();

            let route_line = egui::Shape::line(
                temp_route
                    .linked_vector
                    .iter()
                    .map(|(_, m_id)| self.camera * *self.matrix_coordinate_map.get(&m_id).unwrap())
                    .collect(),
                // FUTURE: we could colour code days, trucks, morning/afternoon, etc.
                Stroke::new(1.0, Color32::LIGHT_BLUE),
            );

            painter.add(route_line);
            let shapes = get_orders().iter().map(|o| {
                let screen_pos =
                    self.camera * Pos2::new(o.x_coordinate as f32, o.y_coordinate as f32);
                let (colour, radius) = match o.matrix_id {
                    287 => (Color32::GREEN, 3.5), // Maarheeze, the dump site
                    // Future: we can also highlight selected companies here, colour code things, whatever else
                    _ => (Color32::BLUE, 2.0),
                };
                egui::Shape::circle_filled(screen_pos, radius, colour)
            });

            painter.extend(shapes);
        });
        egui::SidePanel::right("inspector").show(ctx, |ui| {
            ui.label("Here will be details on the routes, nodes, distances, etc...")
        });
    }
}
