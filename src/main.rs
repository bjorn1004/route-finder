use std::{error::Error, sync::OnceLock};

use eframe::UserEvent;
use egui::{
    Align2, Color32, FontId, Pos2, Rect, Sense, Vec2,
    emath::{self, TSTransform},
    epaint::CircleShape,
};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::{
    gui::GuiApp,
    parser::{parse_distance_matrix, parse_orderfile},
    resource::{Company, DistanceMatrix},
};

mod datastructures;
mod gui;
mod parser;
mod resource;
mod simulated_annealing;

pub static ORDERS: OnceLock<Vec<Company>> = const { OnceLock::new() };

#[inline(always)]
/// If you call this function before orders are parsed I will call you silly and make you wear a dunce hat.
pub fn get_orders() -> &'static Vec<Company> {
    // this is naughty (and faster) but unless you're *really* silly and try
    // getting the orders before parsing them, this should be fine.
    unsafe { ORDERS.get().unwrap_unchecked() }
}

pub static DISTANCE_MATRIX: OnceLock<DistanceMatrix> = const { OnceLock::new() };

#[inline(always)]
/// If you call this function before the distance matrix is parsed I will call you silly and make you wear a dunce hat.
pub fn get_distance_matrix() -> &'static DistanceMatrix {
    unsafe { DISTANCE_MATRIX.get().unwrap_unchecked() }
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let instant = std::time::Instant::now();
    let order_vec = parse_orderfile()?;
    ORDERS.set(order_vec).ok();
    let distance_matrix = parse_distance_matrix()?;
    DISTANCE_MATRIX.set(distance_matrix).ok();

    // let mut dot_file = File::create("dotfile.dot")?;
    // Don't actually try to use dot on this file, it will break your PC
    // dot_file.write_all(
    // Dot::new(&DISTANCE_MATRIX.get().unwrap())
    // .to_string()
    // .as_bytes(),
    // )?;

    // GUI stuff
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    let eventloop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    eventloop.set_control_flow(ControlFlow::Poll);

    let gui_app = GuiApp::new();

    let mut gui_app = eframe::create_native(
        "Route finder",
        options,
        Box::new(|_cc| Ok(Box::new(gui_app))),
        &eventloop,
    );

    eventloop.run_app(&mut gui_app)?;

    println!(
        "Total program runtime: {}s",
        instant.elapsed().as_secs_f64()
    );
    Ok(())
}
