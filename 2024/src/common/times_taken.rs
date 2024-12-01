use std::{time::Duration, ops::{Add, AddAssign}};

#[derive(Clone, Copy, Debug)]
pub struct TimesTaken {
    pub total: Duration,
    pub parsing: Duration,
    pub part_one: Duration,
    pub part_two: Duration,
}

impl TimesTaken {
    pub const ZERO: Self = Self {
        total: Duration::ZERO,
        parsing: Duration::ZERO,
        part_one: Duration::ZERO,
        part_two: Duration::ZERO,
    };

    pub fn divide(self: &Self, divisor: u32) -> TimesTaken {
        TimesTaken {
            total: self.total / divisor,
            parsing: self.parsing / divisor,
            part_one: self.part_one / divisor,
            part_two: self.part_two / divisor,
        }
    }
}

impl Add for TimesTaken {
    type Output = TimesTaken;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            total: self.total + rhs.total,
            parsing: self.parsing + rhs.parsing,
            part_one: self.part_one + rhs.part_one,
            part_two: self.part_two + rhs.part_two,
        }
    }
}

impl AddAssign for TimesTaken {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
