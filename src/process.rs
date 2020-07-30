use std::ops::Range;

use crate::time::Time;

#[derive(Debug)]
pub struct Process {
    pub pid: u16,
    pub size: usize,
    pub life: Range<Time>,
    pub page_faults: u16,
}

impl Process {
    pub fn num_pages(&self, page_size: usize) -> usize {
        (self.size as f64 / page_size as f64).ceil() as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessPage {
    pub pid: u16,
    pub index: usize,
    pub created: Time,
    pub accessed: Time,
}
