use std::collections::BTreeSet;
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::get_orders;
use crate::simulated_annealing::route::Route;
use crate::simulated_annealing::week::Week;
use crate::simulated_annealing::{day::TimeOfDay, simulated_annealing::TruckEnum, week::DayEnum};
use egui::Vec2;
use egui::emath::TSTransform;
use flume::{Receiver, Sender};

mod bottom_panel;
mod center_panel;
mod left_panel;
mod right_panel;

pub use bottom_panel::show_bottom_panel;
pub use center_panel::show_center_panel;
pub use left_panel::show_left_panel;
pub use right_panel::show_right_panel;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct RouteSelection {
    truck: TruckEnum,
    day: DayEnum,
    shift: TimeOfDay,
}

pub struct GuiApp {
    pub camera: TSTransform,
    // A BTree lets us keep the order of routes consistent in GUI
    route_selection: BTreeSet<RouteSelection>,

    // Simulated annealing parameters
    pub temp: f32,
    pub end_temp: f32,
    pub q: u32,
    pub alpha: f32,

    // Multithreading parameters
    pub num_threads: usize,
    pub drawn_thread: usize,

    // Search thread communication
    pub search_handle: Vec<JoinHandle<()>>,
    pub pause_channel: Vec<(Sender<()>, Receiver<()>)>,
    pub stop_channel: Vec<(Sender<()>, Receiver<()>)>,
    pub score_rec: Vec<Receiver<i32>>,
    pub cur_score: Vec<f32>,
    pub q_rec: Vec<Receiver<u32>>,
    pub cur_q: Vec<u32>,
    pub temp_rec: Vec<Receiver<f32>>,
    pub cur_temp: Vec<f32>,
    pub route_rec: Vec<Receiver<(Arc<Week>, Arc<Week>)>>,
    // The currently displayed route
    pub cur_route: Vec<(Arc<Week>, Arc<Week>)>,
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
            temp: 10_000_000.0,
            end_temp: 10.0,
            q: 500_000,
            alpha: 0.99,
            num_threads: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
            drawn_thread: 0,
            search_handle: vec![],
            pause_channel: vec![],
            stop_channel: vec![],
            score_rec: vec![],
            cur_score: vec![],
            q_rec: vec![],
            cur_q: vec![],
            temp_rec: vec![],
            cur_temp: vec![],
            route_rec: vec![],
            cur_route: vec![],
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls").show(ctx, |ui| {
            show_left_panel(ui, self, ctx);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            show_center_panel(ctx, ui, self);
        });

        egui::SidePanel::right("inspector").show(ctx, |ui| {
            show_right_panel(ui, self);
        });

        egui::TopBottomPanel::bottom("multithreading_info").show(ctx, |ui| {
            show_bottom_panel(ui, self);
        });
    }
}

fn route_selection_to_route<'a>(
    cur_route: &'a (Arc<Week>, Arc<Week>),
    selection: &'a RouteSelection,
) -> &'a Route {
    let week = if selection.truck == TruckEnum::Truck1 {
        &cur_route.0
    } else {
        &cur_route.1
    };
    week.get(selection.day).get(selection.shift)
}
