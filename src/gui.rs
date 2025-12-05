use std::collections::{BTreeSet, HashSet};

use egui::{Color32, Pos2, Sense, Stroke, Ui, Vec2, emath::TSTransform};

use crate::{
    datastructures::linked_vectors::LinkedVector,
    get_orders,
    simulated_annealing::{
        day::TimeOfDay, route::Route, simulated_annealing::TruckEnum, week::DayEnum,
    },
};

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
struct RouteSelection {
    truck: TruckEnum,
    day: DayEnum,
    shift: TimeOfDay,
}

pub struct GuiApp {
    pub camera: TSTransform,
    // A BTree lets us keep the order of routes consistent in GUI
    route_selection: BTreeSet<RouteSelection>,
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
            route_selection: BTreeSet::new(),
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading("Controls"));
            ui.separator();
            ui.horizontal(|ui|{
                 if ui.button("Start search").clicked() {
                    // TODO
                 }
                 if ui.button("Pause search").clicked() {
                    // TODO
                 }
            });
            ui.label("Searching overview");
            egui::Grid::new("sim_anneal_overview")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Temperature:");
                    ui.label("todo");
                    ui.end_row();
                    ui.label("Q:");
                    ui.label("todo");
                    ui.end_row();
                });
            ui.separator();
            ui.label("Searching parameters");
            ui.collapsing("Simulated annealing", |ui| {
                egui::Grid::new("sim_anneal_params")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Temperature:");
                        ui.add(egui::DragValue::new(&mut 0.0).range(0.0..=f32::INFINITY));
                        ui.end_row();
                        ui.label("Q:");
                        ui.add(egui::DragValue::new(&mut 0).range(0..=u16::MAX));
                        ui.end_row();
                        ui.label("α:");
                        ui.add(egui::DragValue::new(&mut 0.0).range(0.0..=1.0));
                        ui.end_row();
                    });
            });
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
            for (i, _) in get_orders().iter().enumerate() {
                temp_route.linked_vector.push_front(i);
            }
            temp_route.linked_vector.compact();

            let orders = get_orders();
            let route_line = egui::Shape::line(
                temp_route
                    .linked_vector
                    .iter()
                    .map(|(_, order_index)| {
                        let order = &orders[*order_index];
                        self.camera
                            * Pos2::new(order.x_coordinate as f32, order.y_coordinate as f32)
                    })
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
            ui.vertical_centered(|ui| ui.heading("Inspector"));
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.collapsing("Routes", |ui| {
                    // Future: select all, more difficult than it might seem...
                    if ui.button("Deselect all").clicked() {
                        self.route_selection.clear();
                    }
                    let mut shift_ui =
                        |ui: &mut Ui, shift: TimeOfDay, day: DayEnum, truck: TruckEnum| {
                            let selection = RouteSelection { truck, day, shift };
                            let selected = &mut self.route_selection.contains(&selection);
                            ui.checkbox(selected, shift.to_string());
                            if *selected {
                                self.route_selection.insert(selection);
                            } else {
                                self.route_selection.remove(&selection);
                            }
                        };
                    let mut weekday_ui = |ui: &mut Ui, day: DayEnum, truck: TruckEnum| {
                        shift_ui(ui, TimeOfDay::Morning, day, truck);
                        shift_ui(ui, TimeOfDay::Afternoon, day, truck);
                    };
                    let mut truck_ui = |ui: &mut Ui, truck: TruckEnum| {
                        ui.collapsing("Monday", |ui| {
                            weekday_ui(ui, DayEnum::Monday, truck);
                        });
                        ui.collapsing("Tuesday", |ui| {
                            weekday_ui(ui, DayEnum::Tuesday, truck);
                        });
                        ui.collapsing("Wednesday", |ui| {
                            weekday_ui(ui, DayEnum::Wednesday, truck);
                        });
                        ui.collapsing("Thursday", |ui| {
                            weekday_ui(ui, DayEnum::Thursday, truck);
                        });
                        ui.collapsing("Friday", |ui| {
                            weekday_ui(ui, DayEnum::Friday, truck);
                        });
                    };
                    ui.collapsing("Truck 1", |ui| {
                        truck_ui(ui, TruckEnum::Truck1);
                    });
                    ui.collapsing("Truck 2", |ui| {
                        truck_ui(ui, TruckEnum::Truck2);
                    });
                });
                ui.separator();
                ui.collapsing("Selected routes", |ui| {
                    for route in self.route_selection.iter() {
                        ui.label(format!(
                            "{:?}, {:?}, {:?}",
                            route.truck, route.day, route.shift
                        ));
                        // Future: summary of route statistics
                    }
                });
                // ui.separator();
                // ui.collapsing("Selected orders", |ui| {});
            });
        });
    }
}
