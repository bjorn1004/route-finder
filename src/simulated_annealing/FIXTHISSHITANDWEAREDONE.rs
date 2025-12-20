use crate::get_orders;
use crate::simulated_annealing::day::{Day, TimeOfDay};
use crate::simulated_annealing::route::Route;
use crate::simulated_annealing::week::{DayEnum, Week};
use std::collections::HashMap;
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::solution::Solution;

pub fn fixplzplzplzpl(solution: &mut Solution) {
    let orders = get_orders();
    let mut order_count: HashMap<usize, usize> = HashMap::new();

    count_per_week(&mut solution.truck1, &mut order_count);
    count_per_week(&mut solution.truck2, &mut order_count);

    let a: Vec<(&usize, &usize)> = order_count
        .iter()
        .filter(|(order_i, freq)| orders[**order_i].frequency as usize != **freq)
        .collect();

    let dropoff_index = get_orders().len() - 1;
    let bad: Vec<usize> = a.iter().map(|(a, _)| **a).collect();
    let filtered_bad: Vec<&usize> = bad.iter().filter(|i| **i != dropoff_index).collect();
    let good_bad: Vec<usize> = filtered_bad.iter().map(|i| **i).collect();

    for bad_order in &good_bad{
        solution.order_flags.clear(*bad_order);
    }

    delete_bad_week(&mut solution.truck1, &good_bad);
    delete_bad_week(&mut solution.truck2, &good_bad);
}

fn count_per_week(truck: &Week, order_count: &mut HashMap<usize, usize>) {
    count_per_day(truck.get(DayEnum::Monday), order_count);
    count_per_day(truck.get(DayEnum::Tuesday), order_count);
    count_per_day(truck.get(DayEnum::Wednesday), order_count);
    count_per_day(truck.get(DayEnum::Thursday), order_count);
    count_per_day(truck.get(DayEnum::Friday), order_count);
}
fn count_per_day(day: &Day, order_count: &mut HashMap<usize, usize>) {
    count_per_route(day.get(TimeOfDay::Morning), order_count);
    count_per_route(day.get(TimeOfDay::Afternoon), order_count);
}
fn count_per_route(route: &Route, order_count: &mut HashMap<usize, usize>) {
    for (_, order_i) in route.linked_vector.iter() {
        *order_count.entry(*order_i).or_insert(0) += 1;
    }
}

fn delete_bad_week(truck: &mut Week, bad_list: &[usize]) {
    delete_bad_day(truck.get_mut(DayEnum::Monday), bad_list);
    delete_bad_day(truck.get_mut(DayEnum::Tuesday), bad_list);
    delete_bad_day(truck.get_mut(DayEnum::Wednesday), bad_list);
    delete_bad_day(truck.get_mut(DayEnum::Thursday), bad_list);
    delete_bad_day(truck.get_mut(DayEnum::Friday), bad_list);
}
fn delete_bad_day(day: &mut Day, bad_list: &[usize]) {
    delete_bad_route(day.get_mut(TimeOfDay::Morning), bad_list);
    delete_bad_route(day.get_mut(TimeOfDay::Afternoon), bad_list);
}
fn delete_bad_route(route: &mut Route, bad_list: &[usize]) {
    let mut bad_indexes = Vec::new();
    for (node_i, order_i) in route.linked_vector.iter() {
        if bad_list.contains(order_i) {
            bad_indexes.push(node_i);
        }
    }

    for bad_index in bad_indexes {
        route.apply_remove_node_without_compact(bad_index);
    }
    route.linked_vector.compact();
}
