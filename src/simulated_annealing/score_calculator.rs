use crate::datastructures::linked_vectors::LinkedVector;
use crate::get_orders;
use crate::resource::Time;
use crate::simulated_annealing::order_day_flags::OrderFlags;
use crate::simulated_annealing::route::Route;
use crate::simulated_annealing::week::DayEnum::Monday;
use crate::simulated_annealing::week::Week;

pub fn calculate_score(truck1: &Week, truck2: &Week, order_flags: &OrderFlags) -> Time {
    let truck = truck1.get(Monday);

    truck.morning.linked_vector.push_front(0);

    let orders = get_orders();
    let mut order_count: Vec<usize> = vec![0; orders.len()];

    for route in truck1.iter() {
        add_orders(route, &mut order_count);
    }
    for route in truck2.iter() {
        add_orders(route, &mut order_count);
    }

    order_count.pop(); // remove dropoff

    
    #[cfg(debug_assertions)]
    for (counted_orders, saved_order_count) in order_count.iter().zip(order_flags.get_counts()){
        assert_eq!(*counted_orders as u32, saved_order_count);
    }
    

    let unfinished_orders: Vec<usize> = order_count
        .iter()
        .enumerate()
        .filter(|(order_i, count)| orders[*order_i].frequency as usize != **count)
        .map(|(order_i, _)| order_i)
        .collect();

    let penalty: Time = unfinished_orders
        .iter()
        .map(|order_i| {
            orders[*order_i].frequency as usize as Time * orders[*order_i].emptying_time * 3 as Time
        })
        .sum();

    let total_time = truck1.get_total_time() + truck2.get_total_time();

    println!("time: {}", total_time);
    println!("penalty: {}", penalty);
    total_time + penalty
}

pub fn add_orders(route: &Route, order_count: &mut [usize]) {
    for (_, order_i) in route.linked_vector.iter() {
        order_count[*order_i] += 1;
    }
}

pub fn calcualte_starting_score() -> Time {
    calculate_score(&Week::new(), &Week::new(), &OrderFlags::new(get_orders().iter().count()))
}
