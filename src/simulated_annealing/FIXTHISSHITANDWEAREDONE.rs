use std::collections::HashMap;
use crate::get_orders;
use crate::simulated_annealing::day::Day;
use crate::simulated_annealing::route::Route;
use crate::simulated_annealing::week::Week;

pub fn fixplzplzplzpl(truck1: &mut Week, truck2: &mut Week){
    let orders = get_orders();
    let mut order_count:HashMap<usize, usize> = HashMap::new();

}


pub fn count_per_week(truck: & Week){

}
pub fn count_per_day(day: &Day){

}
pub fn count_per_route(day: &Route, order_count: HashMap<usize, usize>){
    for node in day.linked_vector.iter(){

    }

}
