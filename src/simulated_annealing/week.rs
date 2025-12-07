use rand::{Rng, distr::{Distribution, StandardUniform}};
use super::day::{Day};

#[derive(Clone)]
pub struct Week {
    monday: Day,
    tuesday: Day,
    wednesday: Day,
    thursday: Day,
    friday: Day,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum DayEnum {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday
}
// This makes it easier to get a random day
impl Distribution<DayEnum> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DayEnum {
        match rng.random_range(0..5) {
            0 => DayEnum::Monday,
            1 => DayEnum::Tuesday,
            2 => DayEnum::Wednesday,
            3 => DayEnum::Thursday,
            _ => DayEnum::Friday
        }
    }
}
impl Week{
    pub fn new() -> Self{
        Week{
            monday: Day::new(),
            tuesday: Day::new(),
            wednesday: Day::new(),
            thursday: Day::new(),
            friday: Day::new(),
        }
    }
    pub fn get_random<R>(&self, rng: &mut R) -> (&Day, DayEnum) where R: Rng + ?Sized, {
        let random_day:DayEnum = rng.random();
        (self.get(random_day), random_day)
    }

    pub fn get_mut(&mut self, day: DayEnum) -> &mut Day {
        match day {
            DayEnum::Monday     => {&mut self.monday}
            DayEnum::Tuesday    => {&mut self.tuesday}
            DayEnum::Wednesday  => {&mut self.wednesday}
            DayEnum::Thursday   => {&mut self.thursday}
            DayEnum::Friday     => {&mut self.friday}
        }
    }
    pub fn get(&self, day: DayEnum) -> &Day {
        match day {
            DayEnum::Monday     => {&self.monday}
            DayEnum::Tuesday    => {&self.tuesday}
            DayEnum::Wednesday  => {&self.wednesday}
            DayEnum::Thursday   => {&self.thursday}
            DayEnum::Friday     => {&self.friday}
        }
    }
}