use super::GuiApp;
use crate::simulated_annealing::simulated_annealing::SimulatedAnnealing;
use egui::Ui;
use rand::SeedableRng;
use rand::rngs::SmallRng;

pub fn show_left_panel(ui: &mut Ui, app: &mut GuiApp, ctx: &egui::Context) {
    ui.vertical_centered(|ui| ui.heading("Controls"));
    ui.separator();
    ui.horizontal(|ui| {
        if let Some(search_handle) = app.search_handle.as_ref() {
            if search_handle.is_finished() {
                println!(
                    "Search thread result: {:?}",
                    app.search_handle.take().unwrap().join()
                );
            }
            if ui.button("Stop search").clicked() {
                let _ = app.stop_channel.0.send(());
                println!(
                    "Search thread result: {:?}",
                    app.search_handle.take().unwrap().join()
                );
            }
        } else if ui.button("Start search").clicked() {
            let mut rng = SmallRng::seed_from_u64(0);
            let mut the_thing = SimulatedAnnealing::new(
                &mut rng,
                app.temp,
                app.end_temp,
                app.q,
                app.alpha,
                ctx.clone(),
                app.pause_channel.1.clone(),
                app.stop_channel.1.clone(),
            );
            let (q, temp, route) = the_thing.get_channels();
            app.q_rec = Some(q);
            app.temp_rec = Some(temp);
            app.route_rec = Some(route);
            app.search_handle = Some(std::thread::spawn(move || the_thing.biiiiiig_loop()));
        }
        if ui.button("Pause search").clicked() {
            let _ = app.pause_channel.0.try_send(());
        }
    });
    ui.label("Searching overview");
    if let Some(temp_rec) = &app.temp_rec
        && let Ok(cur_temp) = temp_rec.try_recv()
    {
        app.cur_temp = cur_temp;
    }
    if let Some(q_rec) = &app.q_rec
        && let Ok(cur_q) = q_rec.try_recv()
    {
        app.cur_q = cur_q;
    }

    egui::Grid::new("sim_anneal_overview")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Current score:");
            ui.label("todo");
            ui.end_row();
            ui.label("Temperature:");
            ui.label(app.cur_temp.to_string());
            ui.end_row();
            ui.label("Q:");
            ui.label(app.cur_q.to_string());
            ui.end_row();
        });
    ui.separator();
    ui.label("Searching parameters");
    ui.collapsing("Simulated annealing", |ui| {
        egui::Grid::new("sim_anneal_params")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Start temp.:");
                ui.add(egui::DragValue::new(&mut app.temp).range(0.0..=f32::INFINITY));
                ui.end_row();
                ui.label("End temp.:");
                ui.add(egui::DragValue::new(&mut app.end_temp).range(0.0..=f32::INFINITY));
                ui.end_row();
                ui.label("Q:");
                ui.add(egui::DragValue::new(&mut app.q).range(0..=u32::MAX));
                ui.end_row();
                ui.label("α:");
                ui.add(
                    egui::DragValue::new(&mut app.alpha)
                        .range(0.0..=(1.0 - f32::EPSILON))
                        .speed(0.01),
                );
                ui.end_row();
            });
    });
}
