use std::ops::Range;

use crate::time::Time;
use crate::util::ceil_div;

pub type PID = u16;

#[derive(Debug)]
pub struct Process {
    pub pid: PID,
    size: usize,
    life: Range<Time>,
    page_faults: u16,
}

impl Process {
    pub fn new(pid: PID, size: usize) -> Self {
        Process {
            pid,
            size,
            life: (Time::new()..Time::max()),
            page_faults: 0,
        }
    }

    pub fn num_pages(&self, page_size: usize) -> usize {
        ceil_div(self.size, page_size)
    }

    pub fn includes_address(&self, address: usize) -> bool {
        address < self.size
    }

    pub fn add_page_fault(&mut self) {
        self.page_faults += 1;
    }

    pub fn get_page_faults(&self) -> u16 {
        self.page_faults
    }

    pub fn has_died(&self) -> bool {
        self.life.end != Time::max()
    }

    pub fn set_birth(&mut self, birth: Time) {
        self.life.start = birth;
    }

    pub fn set_death(&mut self, death: Time) {
        self.life.end = death;
    }

    pub fn display_life(&self) -> String {
        format!("{} - {}", self.life.start, self.life.end)
    }

    pub fn calc_turnaround(&self) -> Time {
        self.life.end - self.life.start
    }
}

#[derive(Debug, Clone)]
pub struct ProcessPage {
    pub pid: PID,
    pub index: usize,
    pub created: Time,
    pub accessed: Time,
}
