
use std::fmt::Display;

use super::route::Route;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};

#[derive(Clone)]
pub struct Day {
    morning: Route,
    afternoon: Route,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum TimeOfDay{
    Morning,
    Afternoon
}

impl Display for TimeOfDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let show = match self {
           Self::Morning => "Morning",
           Self::Afternoon => "Afternoon"
        };
        write!(f, "{show}")
    }
}

// This makes it easier to get a random day
impl Distribution<TimeOfDay> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TimeOfDay {
        match rng.random_range(0..2) {
            0 => TimeOfDay::Morning,
            _ => TimeOfDay::Afternoon,
        }
    }
}
impl Day {
    pub fn new() -> Self {
        Day {
            morning: Route::new(),
            afternoon: Route::new(),
        }
    }
    pub fn get_random<R: Rng + ?Sized>(&self, rng: &mut R) -> (&Route, TimeOfDay) {
        match rng.random() {
            TimeOfDay::Morning => (&self.morning, TimeOfDay::Morning),
            TimeOfDay::Afternoon => (&self.afternoon, TimeOfDay::Afternoon)
        }
    }
    pub fn get_mut(&mut self, time_of_day: TimeOfDay) -> &mut Route{
        match time_of_day {
            TimeOfDay::Morning => {&mut self.morning}
            TimeOfDay::Afternoon => {&mut self.afternoon}
        }
    }

    pub fn get(&self, time_of_day: TimeOfDay) -> &Route{
        match time_of_day {
            TimeOfDay::Morning => {&self.morning}
            TimeOfDay::Afternoon => {&self.afternoon}
        }
    }

    pub fn get_time(&self) -> f32 {
        self.morning.time + self.afternoon.time
    }
}
