use crate::datastructures::linked_vectors::LVNodeIndex;
use crate::get_orders;
use crate::resource::Frequency;
use crate::simulated_annealing::day::TimeOfDay;
use crate::simulated_annealing::order_location::OrderInfo::{Empty, Filled, Frice, Thrice, Twice};
use crate::simulated_annealing::route::OrderIndex;
use crate::simulated_annealing::simulated_annealing::TruckEnum;
use crate::simulated_annealing::week::DayEnum;

#[derive(Clone)]
pub struct OrderLocation {
    list: Vec<OrderInfo>
}

#[derive(Clone, Copy)]
pub enum OrderInfo {
    Empty,
    Filled,
    Twice([ExactLocation;2]),
    Thrice([ExactLocation;3]),
    Frice([ExactLocation;4]),
}
#[derive(Clone, Copy)]
pub struct ExactLocation{
    pub truck_enum: TruckEnum,
    pub day_enum: DayEnum,
    pub time_of_day: TimeOfDay,
    pub node_index: LVNodeIndex,
}

impl OrderLocation{
    pub fn new() -> Self{
        let orders = get_orders();
        OrderLocation {
            list: orders.iter().map(|o| match o.frequency {
                _ => Empty,

        }).collect()
        }
    }
    
    pub fn remove_order(&mut self, order_index: OrderIndex){
        self.list[order_index] = Empty;
    }
    pub fn get_order_location(&self, order_index: OrderIndex) -> OrderInfo{
        self.list[order_index]
    }
    pub fn add_order(&mut self, order_index: OrderIndex, loc: ExactLocation, frequency: Frequency){
        match frequency {
            Frequency::None => return,
            Frequency::Once => self.list[order_index] = Filled,
            Frequency::Twice => todo!(),
            Frequency::Thrice => todo!(),
            Frequency::FourTimes => todo!(),
        }
    }

    pub fn is_filled(&self, order_index: OrderIndex) -> bool {
        self.is_filled_helper(self.list[order_index])
    }
    pub fn is_filled_helper(&self, order_info: OrderInfo) -> bool {
        match order_info {
            Empty => false,
            Filled => true,
            Twice(_) => true,
            Thrice(_) => true,
            Frice(_) => true,
        }
    }

    pub fn update_shifts_in_route(&mut self, compact_changes: Vec<(LVNodeIndex, OrderIndex, DayEnum)>) {
        for (node_i, order_i, day) in compact_changes {
            match self.list[order_i]{
                Empty => unreachable!(),
                Filled => {}
                Twice([o1, o2]) => self.list[order_i] = if o1.day_enum == day {Twice([ExactLocation{
                    node_index: node_i,
                    ..o1
                }, o2])} else {
                    assert_eq!(o2.day_enum, day);
                    Twice([o1, ExactLocation{
                        node_index: node_i,
                        ..o2
                    }])
                },
                Thrice(mut arr) => {
                    if let Some(i) = arr.iter().position(|o| o.day_enum == day) {
                        arr[i] = ExactLocation {
                            node_index: node_i,
                            ..arr[i]
                        };
                        self.list[order_i] = Thrice(arr);
                    } else {
                        panic!("day not found in Thrice");
                    }
                }
               Frice(mut arr) => {
                    if let Some(i) = arr.iter().position(|o| o.day_enum == day) {
                        arr[i] = ExactLocation {
                            node_index: node_i,
                            ..arr[i]
                        };
                        self.list[order_i] = Frice(arr);
                    } else {
                        panic!("day not found in Thrice");
                    }
                }
            }
        }
    }
    pub fn update_shift_in_route(&mut self, compact_change: (LVNodeIndex, OrderIndex), day: DayEnum) {
        let (node_i, order_i) = compact_change;
        self.update_shifts_in_route(vec![(node_i, order_i, day)]);
    }

    pub fn unfilled_count(&self) -> usize {
        self.list.iter().map(|x| match self.is_filled_helper(*x) {
            true => 0,
            false => 1,
        }).sum()
    }
    pub fn unfilled_orders(&self) -> Vec<(OrderIndex, Frequency)> {
        self.list.iter().enumerate().filter_map(|(order_index, order_info)| match self.is_filled_helper(*order_info) {
            true => None,
            false => Some((order_index as OrderIndex, get_orders()[order_index].frequency))
        }).collect()
    }

}
