use std::ops::Range;

use crate::time::Time;

#[derive(Debug)]
pub struct Process {
    pub pid: u16,
    pub life: Range<Time>,
    pub page_faults: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessPage {
    pub pid: u16,
    pub index: usize,
    pub size: usize,
    pub created: Time,
    pub accessed: Time,
}
