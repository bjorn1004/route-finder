use std::collections::{HashMap};
use std::fs::{read_to_string};
use crate::{get_orders};
use crate::datastructures::compact_linked_vector::CompactLinkedVector;
use crate::datastructures::linked_vectors::LinkedVector;
use crate::simulated_annealing::day::{TimeOfDay};
use crate::simulated_annealing::order_location::{ExactLocation, OrderLocation};
use crate::simulated_annealing::route::{OrderIndex};
use crate::simulated_annealing::score_calculator::{calculate_score, calculate_starting_score};
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::simulated_annealing::TruckEnum::{Truck1, Truck2};
use crate::simulated_annealing::week::{DayEnum, Week};

#[derive(Clone)]
pub struct Solution{
    pub truck1: Week,
    pub truck2: Week,
    pub score: i32,
    pub unfilled_orders: CompactLinkedVector<OrderIndex>,
    pub order_location: OrderLocation,
}


impl Solution {
    pub fn new() -> Self {
        Solution {
            truck1: Default::default(),
            truck2: Default::default(),
            score: calculate_starting_score(),
            unfilled_orders: Self::fill_unfilled_orders_list(),
            order_location: OrderLocation::new(),
        }
    }

    fn fill_unfilled_orders_list() -> CompactLinkedVector<OrderIndex> {
        let mut deliveries = CompactLinkedVector::new();
        let orders = get_orders();
        for i in 0..orders.len() - 1 {
            deliveries.push_back(i);
        }
        deliveries
    }


    pub fn fulfilled_order_count(&self) -> usize {
        Self::_fulfilled_order_count(&self.truck1) +
            Self::_fulfilled_order_count(&self.truck2)
    }
    fn _fulfilled_order_count(truck: &Week) -> usize {
        truck.iter().map(|route| route.linked_vector.len() - 2)
            .sum()
    }

    pub fn from_file(path: &str) -> Solution{
         let solution_file = read_to_string(path)
             .expect("Could not read the solution file");
        let lines: Vec<Vec<&str>> = solution_file.lines().map(|line|line.split(";").collect()).collect();
        let mut solution = Self::new();
        let orders = get_orders();

        let mut current_day = DayEnum::Monday;
        let mut current_time = TimeOfDay::Morning;
        let id_to_index = Self::order_id_to_index_hash_map();

        let start_index = 2;
        let mut node_index = start_index;
        let mut order_locations:Vec<Vec<ExactLocation>> = get_orders().iter().map(|_| vec![]).collect();

        for line in lines{
            let truck_enum = match line[0].trim() {
                "1" => Truck1,
                "2" => Truck2,
                _ => panic!("Invalid truck number")
            };
            let truck = match truck_enum {
                Truck1 => &mut solution.truck1,
                Truck2 => &mut solution.truck2,
            };

            let day_enum = match line[1].trim() {
                "1" => DayEnum::Monday,
                "2" => DayEnum::Tuesday,
                "3" => DayEnum::Wednesday,
                "4" => DayEnum::Thursday,
                "5" => DayEnum::Friday,
                _ => panic!("Invalid day number")
            };

            if day_enum != current_day {
                current_day = day_enum;
                current_time = TimeOfDay::Morning;
                node_index = start_index;
            }

            let day = truck.get_mut(day_enum);

            if line[3].trim() == "0" {
                current_time = TimeOfDay::Afternoon;
                node_index = start_index;
                continue;
            }

            let order_index = id_to_index[&line[3].trim().parse::<u16>().unwrap()];
            let route = match current_time {
                TimeOfDay::Morning   => &mut day.morning,
                TimeOfDay::Afternoon => &mut day.afternoon,
            };

            let end = route.linked_vector.get_tail_index().unwrap();
            let before_end = route.linked_vector.get_prev_index(end).unwrap();
            solution.score += route.apply_add_order(before_end, order_index);

            order_locations[order_index].push(ExactLocation{
                truck_enum,
                day_enum,
                time_of_day: current_time,
                node_index,
            });

            node_index += 1;
        }

        for (order_i, order_loc) in order_locations.iter().enumerate(){
            if order_i == 1177 {
                continue;
            }
            // clone because too lazy to do it gud
            solution.order_location.add_multiple(order_i, order_loc.clone(), orders[order_i].frequency)
        }

        let unfilled_orders = solution.order_location.unfilled_orders();

        let mut new_unfilled_orders = Vec::new();
        for (order_index, count) in unfilled_orders.iter() {
            for _ in 0..*count as usize {
                new_unfilled_orders.push(*order_index);
            }
        }

        let orders = get_orders();
        solution.score -= unfilled_orders
            .iter()
            .map(|(order_index, count)| if *count as usize > 0 {orders[*order_index].total_container_volume} else {0})
            .sum::<u32>() as i32;

        solution.truck1.recalculate_total_time();
        solution.truck2.recalculate_total_time();
        solution.score = calculate_score(&solution, &solution.order_location);

        solution.unfilled_orders = CompactLinkedVector::new();
        for a in new_unfilled_orders.iter() {
            solution.unfilled_orders.push_back(*a);
        }
        solution
    }

    fn order_id_to_index_hash_map() -> HashMap<u16, OrderIndex> {
        let mut map: HashMap<u16, OrderIndex> = HashMap::new();
        let orders = get_orders();
        for (order_index, order) in orders.iter().enumerate().rev().skip(1).rev() {
            map.insert(order.order, order_index as OrderIndex);
            assert_eq!(orders[order_index].order, order.order);
        }
        map
    }

    pub fn get_truck(&self, truck_enum: TruckEnum) -> &Week {
        match truck_enum {
            Truck1 => {&self.truck1}
            Truck2 => {&self.truck2}
        }
    }
    pub fn get_truck_mut(&mut self, truck_enum: TruckEnum) -> &mut Week {
        match truck_enum {
            Truck1 => {&mut self.truck1}
            Truck2 => {&mut self.truck2}
        }
    }
}

impl Default for Solution {
    fn default() -> Self {
        Self::new()
    }
}