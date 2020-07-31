use std::cmp;
use std::fmt;
use std::ops;

#[derive(Clone, Copy)]
pub struct Time(u32);

impl Time {
    pub fn new() -> Self {
        Time(0)
    }

    pub const fn from_miliseconds(ms: u32) -> Self {
        Time(ms)
    }

    pub fn max() -> Self {
        Time(u32::MAX)
    }
}

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}ms", self.0)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}s", self.0 as f64 / 1000.0)
    }
}

impl ops::Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self::Output {
        Time(self.0 + rhs.0)
    }
}

impl ops::AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl ops::Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Self) -> Self::Output {
        Time(self.0 - rhs.0)
    }
}

impl cmp::PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl cmp::Eq for Time {}

impl cmp::PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl cmp::Ord for Time {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl From<Time> for f64 {
    fn from(time: Time) -> Self {
        time.0 as f64
    }
}
