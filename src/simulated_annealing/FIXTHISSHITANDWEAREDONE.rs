use std::collections::HashMap;
use crate::get_orders;
use crate::simulated_annealing::day::{Day, TimeOfDay};
use crate::simulated_annealing::route::Route;
use crate::simulated_annealing::week::DayEnum::Monday;
use crate::simulated_annealing::week::{DayEnum, Week};

pub fn fixplzplzplzpl(truck1: &mut Week, truck2: &mut Week){
    let orders = get_orders();
    let mut order_count:HashMap<usize, usize> = HashMap::new();

    count_per_week(truck1, &mut order_count);
    count_per_week(truck2, &mut order_count);


}


pub fn count_per_week(truck: &Week, order_count: &mut HashMap<usize,usize>){
    count_per_day(truck.get(DayEnum::Monday), order_count);
    count_per_day(truck.get(DayEnum::Tuesday), order_count);
    count_per_day(truck.get(DayEnum::Wednesday), order_count);
    count_per_day(truck.get(DayEnum::Thursday), order_count);
    count_per_day(truck.get(DayEnum::Friday), order_count);
}
pub fn count_per_day(day: &Day, order_count: &mut HashMap<usize,usize>){
    count_per_route(day.get(TimeOfDay::Morning), order_count);
    count_per_route(day.get(TimeOfDay::Afternoon), order_count);

}
pub fn count_per_route(day: &Route, order_count: &mut HashMap<usize, usize>){
    for (_, order_i) in day.linked_vector.iter(){
        *order_count.entry(*order_i).or_insert(0) += 1;
    }
}
