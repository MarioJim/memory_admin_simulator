use std::ops::Range;

use crate::time::Time;
use crate::util::ceil_div;

pub type PID = u16;

#[derive(Debug)]
pub struct Process {
    pub pid: PID,
    size: usize,
    life: Range<Time>,
    swap_ins: u16,
    swap_outs: u16,
}

impl Process {
    pub fn new(pid: PID, size: usize) -> Self {
        Process {
            pid,
            size,
            life: (Time::new()..Time::max()),
            swap_ins: 0,
            swap_outs: 0,
        }
    }

    pub fn num_pages(&self, page_size: usize) -> usize {
        ceil_div(self.size, page_size)
    }

    pub fn includes_address(&self, address: usize) -> bool {
        address < self.size
    }

    pub fn add_swap_in(&mut self) {
        self.swap_ins += 1;
    }

    pub fn add_swap_out(&mut self) {
        self.swap_outs += 1;
    }

    pub fn get_swaps(&self) -> (u16, u16) {
        (self.swap_ins, self.swap_outs)
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

#[derive(Debug)]
pub struct ProcessPage {
    pub pid: PID,
    pub index: usize,
    pub created: Time,
    pub accessed: Time,
}
