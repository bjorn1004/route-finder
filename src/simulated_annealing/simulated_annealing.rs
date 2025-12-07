use super::neighbor_move::add_new_order::AddNewOrder;
use super::neighbor_move::shift_in_route::ShiftInRoute;
use super::order_day_flags::OrderFlags;
use super::week::{DayEnum, Week};
use crate::get_orders;
use crate::printer::print_solution;
use crate::simulated_annealing::neighbor_move::neighbor_move_trait::{CostChange, NeighborMove};
use crate::simulated_annealing::route::OrderIndex;
use flume::{Receiver, Sender, bounded};
use rand::prelude::{SliceRandom, SmallRng};
use rand::{Rng, SeedableRng};
use std::collections::VecDeque;
use std::f32::consts::E;
use std::sync::Arc;
use rand::distr::{Distribution, StandardUniform};
use crate::simulated_annealing::neighbor_move::shift_between_days::ShiftBetweenDays;

pub struct SimulatedAnnealing {
    temp: f32,
    end_temp: f32,
    q: u32,
    iterations_done: u32,
    a: f32,

    truck1: Week,
    truck2: Week,
    order_flags: OrderFlags,
    unfilled_orders: VecDeque<OrderIndex>,
    // We could store variables here which are needed for simulated annealing.
    paused: bool,

    // Channels for communicating with the draw thread
    egui_ctx: egui::Context,
    pause_rec: Receiver<()>,
    stop_rec: Receiver<()>,
    q_channel: (Sender<u32>, Receiver<u32>),
    temp_channel: (Sender<f32>, Receiver<f32>),
    route_channel: (
        Sender<(Arc<Week>, Arc<Week>)>,
        Receiver<(Arc<Week>, Arc<Week>)>,
    ),
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
    pub fn new<R: Rng + ?Sized>(
        rng: &mut R,
        temp: f32,
        end_temp: f32,
        q: u32,
        a: f32,
        egui_ctx: egui::Context,
        pause_rec: Receiver<()>,
        stop_rec: Receiver<()>,
    ) -> Self {
        // intializationthings
        let orders = get_orders();
        SimulatedAnnealing {
            temp,// initialized as starting temperature, decreases to end_temp
            end_temp,
            q,
            iterations_done: 0,
            a, // keep around 0.95 or 0.99. It's better to change Q or temp
            truck1: Week::new(),
            truck2: Week::new(),
            order_flags: OrderFlags::new(orders.len()),
            unfilled_orders: Self::fill_unfilled_orders_list(rng),
            paused: false,
            egui_ctx,
            pause_rec,
            stop_rec,
            q_channel: bounded(1),
            temp_channel: bounded(1),
            route_channel: bounded(1),
        }
    }

    pub fn get_channels(
        &self,
    ) -> (
        Receiver<u32>,
        Receiver<f32>,
        Receiver<(Arc<Week>, Arc<Week>)>,
    ) {
        (
            self.q_channel.1.clone(),
            self.temp_channel.1.clone(),
            self.route_channel.1.clone(),
        )
    }

    pub fn biiiiiig_loop(&mut self) {
        let mut rng = SmallRng::from_os_rng();
        // let mut rng = SmallRng::seed_from_u64(0);
        // this ic currently an infinite loop.
        // We will need some predicate to exit this loop
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
            if self.iterations_done % self.q == 0 {
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
        print_solution(&self.truck1, &self.truck2).expect("TODO: panic message");
    }

    fn do_step<R: Rng + ?Sized>(&mut self, mut rng: &mut R) {
        // not really sure if this is correct
        loop {
            let a = rng.random_range(1..2);
            // something to decide which thing to choose
            let transactionthingy: Box<dyn NeighborMove> = match a {
                1 => {
                    if let Some(random_order) = self.unfilled_orders.pop_front() {
                        let new_order = AddNewOrder::new(
                            &self.truck1,
                            &self.truck2,
                            &mut rng,
                            &self.order_flags,
                            random_order,
                        );
                        if new_order.is_none() {
                            self.unfilled_orders.push_back(random_order);
                            continue;
                        }
                        Box::new(new_order.unwrap())
                    } else {
                        continue; // queue is empty, try something else
                    }
                }
                2 => {
                    if self.unfilled_orders.len() < 1000 {
                        let shift = ShiftInRoute::new(&self.truck1, &self.truck2, &mut rng);
                        if shift.is_none() {
                            continue;
                        }
                        Box::new(shift.unwrap())
                    } else {
                        continue;
                    }
                }
                3 => {
                    let shift = ShiftBetweenDays::new(&self.truck1, &self.truck2, &mut rng, &self.order_flags);
                    if shift.is_none(){
                        continue;
                    }
                    Box::new(shift.unwrap())
                }
                // remove function, try to remove all days from a single order.
                // for example, if freq==2, remove the order on both the monday and thursday,
                // this will cost O(n) in the length of the routes with our current strurcture
                _ => unreachable!(),
            };

            // get the change in capacity/time

            let cost = transactionthingy.evaluate(&self.truck1, &self.truck2, &self.order_flags);

            // I'm going to use is_none for bad things for now, will later probably be replaced by penalty costs.
            if cost.is_none() {
                continue;
            }
            let cost = cost.unwrap();

            // if we want to go through with this thing
            if self.accept(cost, rng) {
                // change the route
                transactionthingy.apply(&mut self.truck1, &mut self.truck2, &mut self.order_flags);

                // Yes... it uses a clone, I really tried to avoid it, but there's simply no way to ensure no data races or heavy slowdown through locking
                // Future: It should only send a new route when it's faster, not just accepted
                if !self.route_channel.0.is_full() {
                    self.route_channel
                        .0
                        .try_send((Arc::new(self.truck1.clone()), Arc::new(self.truck2.clone())))
                        .ok();
                    self.egui_ctx.request_repaint();
                }
                break;
            }
        }
    }

    fn accept<R: Rng + ?Sized>(&self, cost_change: CostChange, rng: &mut R) -> bool {
        if cost_change <= 0f32 {
            return true;
        }
        let prob = E.powf(-cost_change / self.temp);
        let rand_float: f32 = rng.random();
        if rand_float < prob {
            return true;
        }
        return false;
    }

    fn fill_unfilled_orders_list<R: Rng + ?Sized>(rng: &mut R) -> VecDeque<OrderIndex> {
        let mut list = Vec::new();
        let orders = get_orders();
        for (index, order) in orders.iter().enumerate() {
            for _ in 0..order.frequency as u8 {
                list.push(index);
            }
        }
        list.shuffle(rng);

        VecDeque::from(list)
    }
}
