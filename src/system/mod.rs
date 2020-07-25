use std::ops::Range;

pub mod fifo;
pub mod lru;

use crate::Instruction;

#[derive(Debug)]
pub struct System {
    pub page_size: u16,
    pub m: Vec<Option<ProcessFrame>>,
    pub s: Vec<Option<ProcessFrame>>,
    pub processes: Vec<Process>,
}

impl System {
    pub fn new(page_size: u16, m_size: usize, s_size: usize) -> Self {
        let m_frames = ((m_size as f64) / (page_size as f64)).ceil() as usize;
        let s_frames = ((s_size as f64) / (page_size as f64)).ceil() as usize;
        System {
            page_size,
            m: vec![None; m_frames],
            s: vec![None; s_frames],
            processes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessFrame {
    pub pid: u16,
    pub size: u8,
}

#[derive(Debug)]
pub struct Process {
    pub pid: u16,
    pub life: Range<f64>,
    pub page_faults: u16,
}

pub trait MemoryAdministrationAlgorithm {
    fn new(page_size: u16, m_size: usize, s_size: usize) -> Self;

    fn process_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Process { pid, size } => self.process(*pid, *size),
            Instruction::Access {
                pid,
                address,
                modifies,
            } => self.access(*pid, *address, *modifies),
            Instruction::Free { pid } => self.free(*pid),
            Instruction::End() => self.end(),
            Instruction::Comment(_) | Instruction::Exit() => (),
        }
    }

    fn process(&mut self, pid_to_process: u16, total_size: u16);

    fn access(&mut self, pid_to_access: u16, virtual_address: u16, modifies_process: bool);

    fn free(&mut self, pid_to_free: u16);

    fn end(&mut self);
}
