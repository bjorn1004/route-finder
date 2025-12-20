use std::sync::Arc;

use super::GuiApp;
use crate::simulated_annealing::simulated_annealing::{
    SimulatedAnnealing, SimulatedAnnealingConfig,
};
use egui::Ui;
use flume::bounded;
use rand::SeedableRng;
use rand::rngs::SmallRng;

pub fn show_left_panel(ui: &mut Ui, app: &mut GuiApp, ctx: &egui::Context) {
    ui.vertical_centered(|ui| ui.heading("Controls"));
    ui.separator();
    ui.horizontal(|ui| {
        if !app.search_handle.is_empty() && app.search_handle.iter().all(|t| !t.is_finished()) {
            {
                // println!(
                //     "Search thread result: {:?}",
                //     app.search_handle
                //         .iter()
                //         .map(|t| t.join())
                //         .collect::<Vec<_>>()
                // );
            }
            if ui.button("Stop search").clicked() {
                app.stop_channel.iter().for_each(|s| {
                    s.0.send(()).ok();
                });
                println!(
                    "Search thread result: {:?}",
                    app.search_handle
                        .drain(0..app.search_handle.len())
                        .map(|t| t.join())
                        .collect::<Vec<_>>()
                );
            }
        } else if ui.button("Start search").clicked() {
            app.score_rec.clear();
            app.q_rec.clear();
            app.temp_rec.clear();
            app.route_rec.clear();
            app.pause_channel.clear();
            app.stop_channel.clear();
            app.search_handle.clear();
            for idx in 0..app.num_threads {
                let (pause_snd, pause_rec) = bounded(1);
                let (stop_snd, stop_rec) = bounded(1);
                let (score_sender, score_rec) = bounded(app.num_threads);
                let (q_sender, q_rec) = bounded(app.num_threads);
                let (temp_sender, temp_rec) = bounded(app.num_threads);
                let (route_sender, route_rec) = bounded(app.num_threads);
                app.score_rec.push(score_rec);
                app.q_rec.push(q_rec);
                app.temp_rec.push(temp_rec);
                app.route_rec.push(route_rec);
                app.pause_channel.push((pause_snd, pause_rec));
                app.stop_channel.push((stop_snd, stop_rec));
                app.cur_score = vec![0.0; app.num_threads];
                app.cur_q = vec![0; app.num_threads];
                app.cur_temp = vec![0.0; app.num_threads];
                app.cur_route = vec![
                    (Arc::new(Default::default()), Arc::new(Default::default()));
                    app.num_threads
                ];
                let mut rng = SmallRng::seed_from_u64(0);
                let mut the_thing = SimulatedAnnealing::new(
                    &mut rng,
                    SimulatedAnnealingConfig {
                        idx,
                        temp: app.temp,
                        end_temp: app.end_temp,
                        q: app.q,
                        a: app.alpha,
                        egui_ctx: ctx.clone(),
                        pause_rec: app.pause_channel[idx].1.clone(),
                        stop_rec: app.stop_channel[idx].1.clone(),
                        score_sender: score_sender.clone(),
                        q_sender: q_sender.clone(),
                        temp_sender: temp_sender.clone(),
                        route_sender: route_sender.clone(),
                    },
                );
                app.search_handle.push(std::thread::spawn(move || {
                    the_thing.biiiiiig_loop();
                }));
            }
        }
        if ui.button("Pause search").clicked() {
            app.pause_channel.iter().for_each(|s| {
                s.0.try_send(()).ok();
            });
        }
    });
    ui.label("Searching overview");
    for (idx, score_rec) in app.score_rec.iter().enumerate() {
        if let Ok(cur_score) = score_rec.try_recv() {
            let score = cur_score as f32 / 6000.0;
            app.cur_score[idx] = score;
        }
    }
    for (idx, temp_rec) in app.temp_rec.iter().enumerate() {
        if let Ok(cur_temp) = temp_rec.try_recv() {
            app.cur_temp[idx] = cur_temp;
        }
    }
    for (idx, q_rec) in app.q_rec.iter().enumerate() {
        if let Ok(cur_q) = q_rec.try_recv() {
            app.cur_q[idx] = cur_q;
        }
    }

    egui::Grid::new("sim_anneal_overview")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Current score:");
            ui.label(
                app.cur_score
                    .get(app.drawn_thread)
                    .unwrap_or(&0.0)
                    .to_string(),
            );
            ui.end_row();
            ui.label("Temperature:");
            ui.label(
                app.cur_temp
                    .get(app.drawn_thread)
                    .unwrap_or(&0.0)
                    .to_string(),
            );
            ui.end_row();
            ui.label("Q:");
            ui.label(app.cur_q.get(app.drawn_thread).unwrap_or(&0).to_string());
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
    ui.collapsing("Multithreading", |ui| {
        egui::Grid::new("multithreading_params")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Number of threads:");
                ui.add(
                    egui::DragValue::new(&mut app.num_threads).range(
                        1..=std::thread::available_parallelism()
                            .map(|n| n.get())
                            .unwrap_or(32),
                    ),
                );
                ui.end_row();
            });
    });
}
