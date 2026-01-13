use crate::get_orders;
use crate::resource::Frequency;
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::order_location::OrderInfo::{EmptyFrice, EmptyTwice, Nothing};
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::week::DayEnum;

pub struct OrderLocation {
    list: Vec<OrderInfo>
}

enum OrderInfo {
    Nothing,
    EmptyTwice,
    Twice(ExactLocation, ExactLocation),
    EmptyFrice,
    Frice(u8),
}
pub struct ExactLocation{
    truck_enum: TruckEnum,
    day_enum: DayEnum,
    time_of_day: TimeOfDay,
    order_index: usize,
}

impl OrderLocation{
    pub fn new() -> Self{
        let orders = get_orders();
        OrderLocation {
            list: orders.iter().map(|o| match o.frequency {
                Frequency::Twice => EmptyTwice,
                Frequency::FourTimes => EmptyFrice,
                _ => Nothing,

        }).collect()
        }
    }

    pub fn replace_twice(&mut self, order_index: OrderIndex, one: ExactLocation, two: ExactLocation) {
        self.list[order_index as usize] = OrderInfo::Twice(one, two);
    }

}
