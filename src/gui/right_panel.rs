use super::GuiApp;
use crate::datastructures::linked_vectors::LinkedVector;
use crate::get_orders;
use crate::simulated_annealing::{day::TimeOfDay, simulated_annealing::TruckEnum, week::DayEnum};
use egui::{Color32, Ui};
use time::Time;

pub fn show_right_panel(ui: &mut Ui, app: &mut GuiApp) {
    ui.vertical_centered(|ui| ui.heading("Inspector"));
    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.collapsing("Routes", |ui| {
            if ui.button("Deselect all").clicked() {
                app.route_selection.clear();
            }
            let mut shift_ui = |ui: &mut Ui, shift: TimeOfDay, day: DayEnum, truck: TruckEnum| {
                let selection = super::RouteSelection { truck, day, shift };
                let selected = &mut app.route_selection.contains(&selection);
                ui.checkbox(selected, shift.to_string());
                if *selected {
                    app.route_selection.insert(selection);
                } else {
                    app.route_selection.remove(&selection);
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
        ui.collapsing("Selected routes", |ui| {
            if let Some(routes) = &app.cur_route.get(app.drawn_thread) {
                for selection in app.route_selection.iter() {
                    ui.collapsing(
                        format!(
                            "{:?}, {:?}, {:?}",
                            selection.truck, selection.day, selection.shift
                        ),
                        |ui| {
                            egui::Grid::new(format!(
                                "{:?}_{:?}_{:?}",
                                selection.truck, selection.day, selection.shift
                            ))
                            .num_columns(2)
                            .show(ui, |ui| {
                                let route = super::route_selection_to_route(routes, selection);
                                ui.label("Trash collected:");
                                if route.capacity > 100_000 {
                                    ui.colored_label(
                                        Color32::RED,
                                        format!("{}L, (OVERFLOW)", route.capacity),
                                    );
                                } else {
                                    ui.label(format!("{}L", route.capacity));
                                };
                                ui.end_row();
                                ui.label("Time (h:m:s):");
                                let total_seconds = route.time as u32 / 100;
                                let hours = total_seconds / 3600;
                                let minutes = (total_seconds % 3600) / 60;
                                let seconds = total_seconds % 60;
                                ui.label(format!("{:02}:{:02}:{:02}", hours, minutes, seconds));
                                ui.end_row();
                                ui.label("Orders fulfilled:");
                                ui.label(route.linked_vector.len().to_string());
                                ui.end_row();
                            });
                        },
                    );
                }
            }
        });
        ui.separator();
        ui.collapsing("Week", |ui| {
            if let Some(routes) = &app.cur_route.get(app.drawn_thread) {
                egui::Grid::new("week_overview")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Total orders:");
                        let total_orders: usize = [TruckEnum::Truck1, TruckEnum::Truck2]
                            .iter()
                            .map(|&truck| {
                                [
                                    DayEnum::Monday,
                                    DayEnum::Tuesday,
                                    DayEnum::Wednesday,
                                    DayEnum::Thursday,
                                    DayEnum::Friday,
                                ]
                                .iter()
                                .map(|&day| {
                                    [TimeOfDay::Morning, TimeOfDay::Afternoon]
                                        .iter()
                                        .map(|&shift| {
                                            let selection =
                                                super::RouteSelection { truck, day, shift };
                                            let route =
                                                super::route_selection_to_route(routes, &selection);
                                            route.linked_vector.len() - 2 // exclude depot nodes
                                        })
                                        .sum::<usize>()
                                })
                                .sum::<usize>()
                            })
                            .sum();
                        let order_sum =
                            get_orders().iter().fold(0, |a, o| o.frequency as usize + a);
                        ui.label(format!(
                            "{} / {} ({:.2}%)",
                            total_orders,
                            order_sum,
                            total_orders as f32 / order_sum as f32 * 100f32
                        ));
                        ui.end_row();
                    });
            }
        });
        ui.collapsing("Days", |ui| {
            if let Some(routes) = &app.cur_route.get(app.drawn_thread) {
                let day_overview = |ui: &mut Ui, day: DayEnum| {
                    ui.collapsing(format!("{:?}", day), |ui| {
                        for &truck in &[TruckEnum::Truck1, TruckEnum::Truck2] {
                            ui.collapsing(format!("{:?}", truck), |ui| {
                                let (summary_route, has_overflow) = {
                                    let selection_morning = super::RouteSelection {
                                        truck,
                                        day,
                                        shift: TimeOfDay::Morning,
                                    };
                                    let morning_route = crate::gui::route_selection_to_route(
                                        routes,
                                        &selection_morning,
                                    );
                                    let selection_afternoon = super::RouteSelection {
                                        truck,
                                        day,
                                        shift: TimeOfDay::Afternoon,
                                    };
                                    let afternoon_route = super::route_selection_to_route(
                                        routes,
                                        &selection_afternoon,
                                    );
                                    let mut combined_route = morning_route.clone();
                                    for (_, order_index) in
                                        afternoon_route.linked_vector.iter().skip(1)
                                    {
                                        combined_route.linked_vector.insert_before(
                                            combined_route.linked_vector.get_tail_index().unwrap(),
                                            *order_index,
                                        );
                                    }
                                    combined_route.capacity =
                                        morning_route.capacity + afternoon_route.capacity;
                                    combined_route.time = morning_route.time + afternoon_route.time;
                                    (
                                        combined_route,
                                        morning_route.capacity > 100_000
                                            || afternoon_route.capacity > 100_000,
                                    )
                                };
                                egui::Grid::new(format!("day_overview_{:?}_{:?}", truck, day))
                                    .num_columns(2)
                                    .show(ui, |ui| {
                                        ui.label("Trash collected:");
                                        if has_overflow {
                                            ui.colored_label(
                                                Color32::RED,
                                                format!("{}L, (OVERFLOW)", summary_route.capacity),
                                            );
                                        } else {
                                            ui.label(format!("{}L", summary_route.capacity));
                                        };
                                        ui.end_row();
                                        ui.label("Time (h:m:s):");
                                        let total_seconds = summary_route.time as u32 / 100;
                                        let hours = total_seconds / 3600;
                                        let minutes = (total_seconds % 3600) / 60;
                                        let seconds = total_seconds % 60;
                                        ui.label(format!(
                                            "{:02}:{:02}:{:02}",
                                            hours, minutes, seconds
                                        ));
                                        ui.end_row();
                                        ui.label("Finish time:");
                                        let finish_time = {
                                            let total_minutes = summary_route.time as u32 / 6000;
                                            let hours = 6 + (total_minutes / 60);
                                            let minutes = total_minutes % 60;
                                            Time::from_hms(
                                                hours.try_into().unwrap(),
                                                minutes.try_into().unwrap(),
                                                0,
                                            )
                                            .unwrap_or(Time::from_hms(23, 59, 59).unwrap())
                                        };
                                        if finish_time >= Time::from_hms(18, 0, 0).unwrap() {
                                            ui.colored_label(
                                                Color32::RED,
                                                format!(
                                                    "{:02}:{:02} (LATE)",
                                                    finish_time.hour(),
                                                    finish_time.minute()
                                                ),
                                            );
                                        } else {
                                            ui.label(format!(
                                                "{:02}:{:02}",
                                                finish_time.hour(),
                                                finish_time.minute()
                                            ));
                                        }
                                        ui.end_row();
                                        ui.label("Orders fulfilled:");
                                        ui.label(summary_route.linked_vector.len().to_string());
                                        ui.end_row();
                                    });
                            });
                        }
                    });
                };
                for day in [
                    DayEnum::Monday,
                    DayEnum::Tuesday,
                    DayEnum::Wednesday,
                    DayEnum::Thursday,
                    DayEnum::Friday,
                ] {
                    day_overview(ui, day);
                }
            }
        });
    });
}
