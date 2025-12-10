use crate::get_orders;
use crate::resource::Time;
use crate::simulated_annealing::route::Route;
use crate::simulated_annealing::week::Week;

pub fn calculate_score(truck1: &Week, truck2: &Week) -> f32{
    let orders = get_orders();
    let mut order_count: Vec<usize> = vec![0;orders.len()];

    for route in truck1.iter(){
        add_orders(route, &mut order_count);
    }
    for route in truck2.iter(){
        add_orders(route, &mut order_count);
    }

    order_count.pop(); // remove dropoff

    let unfinished_orders:Vec<usize> =
        order_count
            .iter()
            .enumerate()
            .filter(|(order_i, count)| orders[*order_i].frequency as usize != **count)
            .map(|(order_i, _)| order_i)
            .collect();

    let penalty:Time =
        unfinished_orders
            .iter()
            .map(|order_i| orders[*order_i].frequency as usize as Time * orders[*order_i].emptying_time * 3 as Time)
            .sum();

    let total_time = truck1.get_total_time() + truck2.get_total_time();
    
    println!("time: {}", total_time);
    println!("penalty: {}", penalty);
    total_time as f32 + penalty as f32
}

pub fn add_orders(route: &Route, order_count: &mut Vec<usize>){
    for (_, order_i) in route.linked_vector.iter(){
        order_count[*order_i] += 1;
    }
}

pub fn calcualte_starting_score() -> f32{
    calculate_score(&Week::new(), &Week::new())
}

