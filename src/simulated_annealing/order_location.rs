use std::collections::HashSet;
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
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
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
    pub fn add_multiple(&mut self, order_index: OrderIndex, locs: Vec<ExactLocation>, frequency: Frequency) {
         self.list[order_index] = match frequency {
             Frequency::None => unreachable!(),
             Frequency::Once => Filled,
             Frequency::Twice => Twice([locs[0], locs[1]]),
             Frequency::Thrice => Thrice([locs[0], locs[1], locs[2]]),
             Frequency::FourTimes => {

                 #[cfg(debug_assertions)]
                 {
                     let mut set: HashSet<DayEnum> = HashSet::new();
                     for i in &locs {
                         set.insert(i.day_enum);
                     }
                     assert_eq!(set.len(), locs.len());
                 }
                 Frice([locs[0], locs[1], locs[2], locs[3]])
             }
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
        // println!("update {}", compact_changes[0].1);
        for (node_i, order_i, day) in compact_changes {
            match self.list[order_i]{
                Empty => return, // if it is empty, we don't need to do anything probably????
                Filled => {}
                Twice([o1, o2]) => self.list[order_i] = if o1.day_enum == day {
                    #[cfg(debug_assertions)]
                    assert_eq!(o1.day_enum, day);
                    Twice([ExactLocation{
                    node_index: node_i,
                    ..o1
                }, o2])} else {
                    #[cfg(debug_assertions)]
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
                        #[cfg(debug_assertions)]
                        {
                            let mut set:HashSet<DayEnum> = HashSet::new();
                            for i in arr{
                                set.insert(i.day_enum);
                            }
                            assert_eq!(set.len(), arr.len());
                        }

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
    pub fn update_in_day(&mut self, order_index: OrderIndex, exact_location: ExactLocation) {
        let day = exact_location.day_enum;

        match self.list[order_index] {
            Empty => return,
            Filled => return,
            Twice(mut arr) => {
                if let Some(i) = arr.iter().position(|o| o.day_enum == day) {
                    arr[i] = exact_location;
                    self.list[order_index] = Twice(arr);
                } else {
                    unreachable!()
                }
            },
            Thrice(mut arr) => {
                if let Some(i) = arr.iter().position(|o| o.day_enum == day) {
                    arr[i] = exact_location;
                    self.list[order_index] = Thrice(arr);
                } else {
                    unreachable!()
                }
            },
            Frice(mut arr) => {
                if let Some(i) = arr.iter().position(|o| o.day_enum == day) {
                    arr[i] = exact_location;
                    #[cfg(debug_assertions)]
                    {
                        let mut set: HashSet<DayEnum> = HashSet::new();
                        for i in arr {
                            set.insert(i.day_enum);
                            let mut set: HashSet<DayEnum> = HashSet::new();
                            for i in arr {
                                set.insert(i.day_enum);
                            }
                        }
                        assert_eq!(set.len(), arr.len());
                    }

                    self.list[order_index] = Frice(arr);
                } else {
                    unreachable!()
                }
            },
        }
    }

    fn _other_time_of_day(time_of_day: TimeOfDay) -> TimeOfDay {
        match time_of_day{
            TimeOfDay::Morning => TimeOfDay::Afternoon,
            TimeOfDay::Afternoon => TimeOfDay::Morning,
        }
    }

    pub fn unfilled_count(&self) -> usize {
        self.list.iter().map(|x| match self.is_filled_helper(*x) {
            true => 0,
            false => 1,
        }).sum()
    }
    pub fn unfilled_orders(&self) -> Vec<(OrderIndex)> {
        let mut orders:Vec<(OrderIndex)> = self.list.iter().enumerate().filter_map(|(order_index, order_info)| match self.is_filled_helper(*order_info) {
            true => None,
            false => Some(order_index as OrderIndex)
        }).collect();
        // remove the last thingy, this is the droppoff
        orders.pop();
        orders
    }

}
