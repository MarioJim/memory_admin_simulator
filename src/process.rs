use std::ops::Range;

#[derive(Debug)]
pub struct Process {
    pub pid: u16,
    pub life: Range<f64>,
    pub page_faults: u16,
}

#[derive(Debug, Clone)]
pub struct ProcessFrame {
    pub pid: u16,
    pub index: usize,
    pub size: u8,
    pub last_accessed: f64,
}
