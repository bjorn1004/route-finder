use std::cmp::{max};
use super::order_day_flags::OrderFlags;
use super::week::Week;
use crate::get_orders;
use crate::printer::print_solution;
use crate::resource::Company;
use crate::simulated_annealing::FIXTHISSHITANDWEAREDONE::fixplzplzplzpl;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove};
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::score_calculator::{calcualte_starting_score, calculate_score};
use flume::{Receiver, Sender, bounded};
use rand::distr::{Distribution, StandardUniform};
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use std::collections::VecDeque;
use std::f32::consts::E;
use std::sync::Arc;
use std::time::Instant;

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
    q: u32,
    iterations_done: u32,
    a: f32,
    pub score: i32,

    pub(crate) truck1: Week,
    pub(crate) truck2: Week,
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
            q: config.q,
            iterations_done: 0,
            a: config.a, // keep around 0.95 or 0.99. It's better to change Q or temp
            score: calcualte_starting_score(),
            truck1: Week::new(),
            truck2: Week::new(),
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

    /// Set the trucks to be used in the simulated annealing process
    pub fn with_trucks(&mut self, truck1: Week, truck2: Week) {
        self.truck1 = truck1;
        self.truck2 = truck2;
        self.score = calculate_score(&self.truck1, &self.truck2, &self.order_flags);
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

    pub fn biiiiiig_loop(&mut self) {
        let mut rng = SmallRng::from_os_rng();
        let now = Instant::now();
        // let mut rng = SmallRng::seed_from_u64(0);
        // this ic currently an infinite loop.

        calculate_score(&self.truck1, &self.truck2, &self.order_flags);

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
                    .send((Arc::new(self.truck1.clone()), Arc::new(self.truck2.clone())))
                    .ok();
                self.egui_ctx.request_repaint();
                continue;
            }
            self.do_step(&mut rng);

            self.iterations_done += 1;
            self.q_channel
                .0
                .try_send(self.iterations_done % self.q)
                .ok();
            self.score_channel.0.try_send(self.score).ok();
            if self.iterations_done.is_multiple_of(self.q) {
                self.temp *= self.a;
                self.temp_channel.0.try_send(self.temp).ok();
            }
            if self.temp <= self.end_temp {
                break;
            }
        }
        // send final state before closing
        let _ = self.route_channel.1.drain().map(drop);
        self.route_channel
            .0
            .send((Arc::new(self.truck1.clone()), Arc::new(self.truck2.clone())))
            .ok();
        self.egui_ctx.request_repaint();
        println!("seconds:      {}", now.elapsed().as_secs());
        println!("iterations:   {}", self.iterations_done);
        println!(
            "iter/sec:     {}",
            self.iterations_done as u64 / max(now.elapsed().as_secs(), 1)
        );
        let before_fixplzplzplzplzplz = calculate_score(&self.truck1, &self.truck2, &self.order_flags);
        fixplzplzplzpl(&mut self.truck1, &mut self.truck2, &mut self.order_flags);

        let before_recalc = calculate_score(&self.truck1, &self.truck2, &self.order_flags);
        if before_fixplzplzplzplzplz != before_recalc {
            println!("fixplzplzplzplz removed at least one order to get a correct answer")
        }
        self.truck1.recalculate_total_time();
        self.truck2.recalculate_total_time();
        let after_recalc = calculate_score(&self.truck1, &self.truck2, &self.order_flags);
        if after_recalc != before_recalc {
            println!("Incorrect score was stored");
            println!();
            println!(
                "difference in minutes: {}",
                (before_recalc - after_recalc) / 6000
            );
        }

        println!("score: {}", after_recalc);
        print_solution(after_recalc, &self.truck1, &self.truck2).expect("failed to print the solution");
    }

    fn do_step<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let (transactionthingy, order_to_add_after_apply) = self.choose_neighbor(rng);
        // get the change in capacity/time

        let cost = transactionthingy.evaluate(&self.truck1, &self.truck2, &self.order_flags);

        // if we want to go through with this thing
        if self.accept(cost, rng) {
            // change the route

            self.score += transactionthingy.apply(
                &mut self.truck1,
                &mut self.truck2,
                &mut self.order_flags,
            );

            if let Some(order_to_add_after_apply) = order_to_add_after_apply {
                println!("put something back");
                self.unfilled_orders.push_back(order_to_add_after_apply)
            }
            // Yes... it uses a clone, I really tried to avoid it, but there's simply no way to ensure no data races or heavy slowdown through locking
            // Future: It should only send a new route when it's faster, not just accepted
            if !self.route_channel.0.is_full() {
                self.route_channel
                    .0
                    .try_send((Arc::new(self.truck1.clone()), Arc::new(self.truck2.clone())))
                    .ok();
                self.egui_ctx.request_repaint();
            }
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
}
