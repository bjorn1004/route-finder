use std::fs::File;
use std::io::Write;
use time::OffsetDateTime;
use crate::get_orders;
use crate::resource::Time;
use crate::simulated_annealing::day::{Day, TimeOfDay};
use crate::simulated_annealing::route::Route;
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::week::{DayEnum, Week};
use crate::simulated_annealing::solution::Solution;

pub fn print_solution(solution: &Solution, dir: &String, iteration: u32) -> std::io::Result<()>
{
    let file_name = format!("{}/{} {}.txt", dir, iteration, solution.score/6000);
    let mut buffer = File::create(file_name)?;

    print_truck_schedule(&mut buffer, &solution.truck1, TruckEnum::Truck1)?;
    print_truck_schedule(&mut buffer, &solution.truck2, TruckEnum::Truck2)?;

    Ok(())
}
fn print_truck_schedule(buffer: &mut File, truck: &Week, truck_enum: TruckEnum)
    -> std::io::Result<()> {
    let truck_id = match truck_enum {
        TruckEnum::Truck1 => "1",
        TruckEnum::Truck2 => "2",
    };

    print_day_schedule(buffer, truck.get(DayEnum::Monday),&DayEnum::Monday, truck_id)?;
    print_day_schedule(buffer, truck.get(DayEnum::Tuesday),&DayEnum::Tuesday, truck_id)?;
    print_day_schedule(buffer, truck.get(DayEnum::Wednesday),&DayEnum::Wednesday, truck_id)?;
    print_day_schedule(buffer, truck.get(DayEnum::Thursday),&DayEnum::Thursday, truck_id)?;
    print_day_schedule(buffer, truck.get(DayEnum::Friday),&DayEnum::Friday, truck_id)?;

    Ok(())
}

fn print_day_schedule(buffer: &mut File, day: &Day, day_enum: &DayEnum, truck_id: &str)
    -> std::io::Result<()> {
    let day_id = match day_enum {
        DayEnum::Monday  => "1",
        DayEnum::Tuesday => "2",
        DayEnum::Wednesday => "3",
        DayEnum::Thursday => "4",
        DayEnum::Friday => "5",
    };

    let end_index =
        print_route(buffer, day.get(TimeOfDay::Morning), truck_id, day_id, 0)?;
    print_route(buffer, day.get(TimeOfDay::Afternoon), truck_id, day_id, end_index)?;
    Ok(())
}

fn print_route(buffer: &mut File, route: &Route, truck_id:&str, day_id:&str, start_index:usize)
    -> std::io::Result<usize> {
    let orders = get_orders();
    let mut last_i=0;
    let lv = &route.linked_vector;
    if lv.len() < 3 {
        return Ok(last_i)
    }

    let iter = lv.iter().enumerate();
    for (i, (_, order_index)) in iter.skip(1) {
        writeln!(buffer,"{}; {}; {}; {}", truck_id, day_id, start_index+i,orders[*order_index].order)?;
        last_i = i;
    }
    Ok(last_i)
}