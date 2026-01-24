use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::day::TimeOfDay::{Afternoon, Morning};
use crate::simulated_annealing::order_location::ExactLocation;
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::simulated_annealing::TruckEnum::{Truck1, Truck2};
use crate::simulated_annealing::solution::Solution;
use crate::simulated_annealing::week::DayEnum;
use crate::simulated_annealing::week::DayEnum::{Friday, Monday, Thursday, Tuesday, Wednesday};

pub fn move_route_between_trucks(solution: Solution) -> Solution {
    let solution =  move_routes_between_trucks_(solution, Monday);
    let solution =  move_routes_between_trucks_(solution, Tuesday);
    let solution =  move_routes_between_trucks_(solution, Wednesday);
    let solution =  move_routes_between_trucks_(solution, Thursday);
    move_routes_between_trucks_(solution, Friday)
}

fn move_routes_between_trucks_(mut solution: Solution, day_enum: DayEnum) -> Solution{
    solution = yay(solution, Truck1, day_enum);
    solution = yay(solution, Truck2, day_enum);

    solution
}

fn yay(mut solution: Solution, truck_enum: TruckEnum, day_enum: DayEnum) -> Solution{
    let (t1, t2) = if truck_enum == Truck1 {
        (solution.truck1.get_mut(day_enum), solution.truck2.get_mut(day_enum))
    } else {
        (solution.truck2.get_mut(day_enum), solution.truck1.get_mut(day_enum))
    };

    if t1.morning.linked_vector.len() == 2 && t1.afternoon.linked_vector.len() == 2{
        if t2.morning.linked_vector.len() == 2 {
            (t1.morning, t2.afternoon) = (t2.morning.clone(), t1.afternoon.clone());
            solution = fix_order_locations(solution, truck_enum, day_enum, Morning);
        } else {
            (t1.morning, t2.morning) = (t2.morning.clone(), t1.morning.clone());
            solution = fix_order_locations(solution, truck_enum, day_enum, Morning);
        }
    }
    solution
}

fn fix_order_locations(mut solution: Solution, truck_enum: TruckEnum, day_enum: DayEnum, time_of_day: TimeOfDay) -> Solution {

    let route = if truck_enum == Truck1 {&mut solution.truck1} else {&mut solution.truck2}.get_mut(day_enum).get_mut(time_of_day);

    for (node_index, order) in route.linked_vector.iter(){
        if node_index == 0 || node_index == 1 {continue;}

        solution.order_location.update_in_day(*order, ExactLocation{
            truck_enum,
            day_enum,
            time_of_day,
            node_index,
        });
    }

    solution
}