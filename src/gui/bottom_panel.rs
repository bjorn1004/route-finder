use super::GuiApp;

pub fn show_bottom_panel(ui: &mut egui::Ui, app: &mut GuiApp) {
    ui.horizontal(|ui| {
        ui.take_available_space();
        egui::Grid::new("multithreading_info_grid")
            .num_columns(5)
            .striped(true)
            .show(ui, |ui| {
                for i in 0..app.num_threads {
                    ui.radio_value(&mut app.drawn_thread, i, format!("Thread {}", i));
                    // TODO: Replace with real data
                    let status = {
                        if let Some(handle) = app.search_handle.get(i) {
                            if handle.is_finished() {
                                "Idle"
                            } else {
                                "Running"
                            }
                        } else {
                            "Idle"
                        }
                    };
                    ui.label(status);
                    ui.label(format!("Score: {}", app.cur_score.get(i).unwrap_or(&0.0)));
                    ui.label(format!("Temp.: {}", app.cur_temp.get(i).unwrap_or(&0.0)));
                    ui.label(format!("Q: {}", app.cur_q.get(i).unwrap_or(&0)));
                    ui.end_row();
                }
            });
    });
}
