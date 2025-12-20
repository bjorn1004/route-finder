use std::cmp::{max};
use super::order_day_flags::OrderFlags;
use super::week::Week;
use crate::get_orders;
use crate::printer::print_solution;
use crate::resource::{Company, Time};
use crate::simulated_annealing::FIXTHISSHITANDWEAREDONE::fixplzplzplzpl;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove};
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::score_calculator::{add_orders, calculate_score, calculate_starting_score};
use flume::{Receiver, Sender, bounded};
use rand::distr::{Distribution, StandardUniform};
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use std::collections::VecDeque;
use std::f32::consts::E;
use std::fs::create_dir;
use std::sync::Arc;
use std::time::Instant;
use time::OffsetDateTime;
use crate::simulated_annealing::neighbor_move::add_new_order::AddNewOrder;
use crate::simulated_annealing::solution::Solution;

type RouteState = (Arc<Week>, Arc<Week>);

pub struct SimulatedAnnealingConfig {
    pub temp: f32,
    pub end_temp: f32,
    pub q: u32,
    pub a: f32,
    pub egui_ctx: egui::Context,
    pub pause_rec: Receiver<()>,
    pub stop_rec: Receiver<()>,
}

pub struct SimulatedAnnealing {
    temp: f32,
    end_temp: f32,
    reheating_temp: f32,
    max_iterations: u32,
    num_pertubations: u32,
    q: u32,
    step_count: u32,
    a: f32,

    pub solution: Solution,
    pub(crate) order_flags: OrderFlags,
    pub(crate) unfilled_orders: VecDeque<OrderIndex>,
    // We could store variables here which are needed for simulated annealing.
    paused: bool,

    // Channels for communicating with the draw thread
    egui_ctx: egui::Context,
    pause_rec: Receiver<()>,
    stop_rec: Receiver<()>,
    score_channel: (Sender<i32>, Receiver<i32>),
    q_channel: (Sender<u32>, Receiver<u32>),
    temp_channel: (Sender<f32>, Receiver<f32>),
    route_channel: (Sender<RouteState>, Receiver<RouteState>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum TruckEnum {
    Truck1,
    Truck2,
}
impl Distribution<TruckEnum> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TruckEnum {
        match rng.random_range(0..2) {
            0 => TruckEnum::Truck1,
            _ => TruckEnum::Truck2,
        }
    }
}

impl SimulatedAnnealing {
    pub fn new<R: Rng + ?Sized>(rng: &mut R, config: SimulatedAnnealingConfig) -> Self {
        // intializationthings
        let orders = get_orders();
        SimulatedAnnealing {
            temp: config.temp, // initialized as starting temperature, decreases to end_temp
            end_temp: config.end_temp,
            reheating_temp: 5000f32,
            max_iterations: 100,
            num_pertubations: 100,
            q: config.q,
            step_count: 0,
            a: config.a, // keep around 0.95 or 0.99. It's better to change Q or temp

            solution: Solution{
                truck1: Default::default(),
                truck2: Default::default(),
                score: calculate_starting_score(),
            },
            order_flags: OrderFlags::new(orders.len()),
            unfilled_orders: Self::fill_unfilled_orders_list(rng),
            paused: false,
            egui_ctx: config.egui_ctx,
            pause_rec: config.pause_rec,
            stop_rec: config.stop_rec,
            score_channel: bounded(1),
            q_channel: bounded(1),
            temp_channel: bounded(1),
            route_channel: bounded(1),
        }
    }

    /// Get the channels for communicating with the GUI
    pub fn get_channels(
        &self,
    ) -> (
        Receiver<i32>,
        Receiver<u32>,
        Receiver<f32>,
        Receiver<RouteState>,
    ) {
        (
            self.score_channel.1.clone(),
            self.q_channel.1.clone(),
            self.temp_channel.1.clone(),
            self.route_channel.1.clone(),
        )
    }

    // Iterated Local Search (ILS)
    pub fn insanely_large_stuffloop(&mut self) {
        let mut rng = SmallRng::from_os_rng();
        // let mut rng = SmallRng::seed_from_u64(0);

        let now = OffsetDateTime::now_local().unwrap();
        let output_dir = format!("output/{now}").replace(":","_");

        self.biiiiiig_loop(&mut rng);
        create_dir(&output_dir).expect("Could not create an output folder");
        print_solution(&self.solution, &output_dir, 0).expect("failed to print the solution");

        for i in 1..=self.max_iterations {
            self.temp = f32::MAX;
            for _ in 0..self.num_pertubations{
                self.do_step(&mut rng, [
                    1, // add new order
                    10, // shift inside of a route
                    10, // shift between days
                    1
                    // if self.solution.score <= 6000*MINUTE {1} else {0}, // remove
                ]);
            }

            self.temp = self.reheating_temp;
            self.biiiiiig_loop(&mut rng);
            print_solution(&self.solution, &output_dir, i).expect("failed to print the solution");
        }
    }

    pub fn biiiiiig_loop<R: Rng + ?Sized>(&mut self, rng: &mut R){
        let now = Instant::now();
        // this ic currently an infinite loop.

        // main loop: gui stuff and do_step and thermostat
        loop {
            if self.stop_rec.try_recv().is_ok() {
                break;
            }
            if self.pause_rec.try_recv().is_ok() {
                self.paused = !self.paused;
            }
            if self.paused {
                // if paused, just send the latest state untill unpaused
                self.route_channel
                    .0
                    .send((Arc::new(self.solution.truck1.clone()), Arc::new(self.solution.truck2.clone())))
                    .ok();
                self.egui_ctx.request_repaint();
                continue;
            }
            self.do_step(rng, [
                100, // add new order
                1000, // shift inside of a route
                1000, // shift between days
                1
                // if self.solution.score <= 6000*MINUTE {1} else {0}, // remove
            ]);

            self.step_count += 1;
            self.q_channel
                .0
                .try_send(self.step_count % self.q)
                .ok();
            self.score_channel.0.try_send(self.solution.score).ok();
            if self.step_count.is_multiple_of(self.q) {
                self.temp *= self.a;
                self.temp_channel.0.try_send(self.temp).ok();
            }
            if self.temp <= self.end_temp {
                break;
            }
        }

       // summarize run
        println!("seconds:      {}", now.elapsed().as_secs());
        println!("iterations:   {}", self.step_count);
        println!(
            "iter/sec:     {}",
            self.step_count as u64 / max(now.elapsed().as_secs(), 1)
        );

        // cleanup
        let after_recalc = self.cleanup();

        println!("score: {}", after_recalc as f32 / 6000f32);

        // send final state before closing
        let _ = self.route_channel.1.drain().map(drop);
        self.route_channel
            .0
            .send((Arc::new(self.solution.truck1.clone()), Arc::new(self.solution.truck2.clone())))
            .ok();
        self.egui_ctx.request_repaint();
    }

    fn do_step<R: Rng + ?Sized>(&mut self, rng: &mut R, weights: [i32;4]) {
        let (neighborhood, order_to_add_after_apply) = self.choose_neighbor(rng,weights);

        // get the change in capacity/time
        let cost = neighborhood.evaluate(&self.solution, &self.order_flags);

        // if we want to go through with this thing
        if self.accept(cost, rng) {
            // change the route

            self.solution.score += neighborhood.apply(
                &mut self.solution,
                &mut self.order_flags,
            );

            if let EndOfStepInfo::Removed(order_to_add_after_apply) = order_to_add_after_apply {
                self.unfilled_orders.push_back(order_to_add_after_apply)
            }
            // Yes... it uses a clone, I really tried to avoid it, but there's simply no way to ensure no data races or heavy slowdown through locking
            // Future: It should only send a new route when it's faster, not just accepted
            if !self.route_channel.0.is_full() {
                self.route_channel
                    .0
                    .try_send((Arc::new(self.solution.truck1.clone()), Arc::new(self.solution.truck2.clone())))
                    .ok();
                self.egui_ctx.request_repaint();
            }
            return;
        }
        if let EndOfStepInfo::Add(order_to_add_after_apply) = order_to_add_after_apply {
            self.unfilled_orders.push_back(order_to_add_after_apply)
        }
    }

    fn accept<R: Rng + ?Sized>(&self, cost_change: CostChange, rng: &mut R) -> bool {
        if cost_change <= 0 {
            return true;
        }
        let prob = E.powf(-(cost_change as f32) / self.temp);
        let rand_float: f32 = rng.random();
        if rand_float < prob {
            return true;
        }
        false
    }

    fn fill_unfilled_orders_list<R: Rng + ?Sized>(_rng: &mut R) -> VecDeque<OrderIndex> {
        let mut deliveries = Vec::new();
        let orders = get_orders();
        let mut list: Vec<(usize, &Company)> = orders.iter().enumerate().collect();
        list.sort_by_key(|(_, order)| order.frequency as u8);
        for (index, order) in list.iter() {
            for _ in 0..order.frequency as u8 {
                deliveries.push(*index);
            }
        }

        VecDeque::from(deliveries)
    }

    fn cleanup(&mut self) -> Time {
        // Cleanup: remove incomplete orders and recalculate scores
        let before_fixplzplzplzplzplz = calculate_score(&self.solution, &self.order_flags);

        fixplzplzplzpl(&mut self.solution, &mut self.order_flags);

        let before_recalc = calculate_score(&self.solution, &self.order_flags);
        if before_fixplzplzplzplzplz != before_recalc {
            println!("fixplzplzplzplz removed at least one order to get a correct answer")
        }

        self.solution.truck1.recalculate_total_time();
        self.solution.truck2.recalculate_total_time();
        let after_recalc = calculate_score(&self.solution, &self.order_flags);
        if after_recalc != before_recalc {
            println!("Incorrect score was stored");
            println!();
            println!(
                "difference in minutes: {}",
                (before_recalc - after_recalc) / 6000
            );
        }

        after_recalc
    }
}

pub enum EndOfStepInfo{
    Nothing,
    Removed(OrderIndex),
    Add(OrderIndex),
}
