use std::ops::Range;

use crate::time::Time;
use crate::util::ceil_division;

#[derive(Debug)]
pub struct Process {
    pub pid: u16,
    pub size: usize,
    pub life: Range<Time>,
    pub page_faults: u16,
}

impl Process {
    pub fn num_pages(&self, page_size: usize) -> usize {
        ceil_division(self.size, page_size)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessPage {
    pub pid: u16,
    pub index: usize,
    pub created: Time,
    pub accessed: Time,
}
