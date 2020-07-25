pub mod fifo;
pub mod lru;

use crate::process::{Process, ProcessFrame};
use crate::Instruction;

#[derive(Debug)]
pub struct System {
    pub processes: Vec<Process>,
    pub page_size: u16,
    pub m: Vec<Option<ProcessFrame>>,
    pub s: Vec<Option<ProcessFrame>>,
}

enum Memory {
    Real,
    Swap,
}

impl System {
    fn new(page_size: u16, m_size: usize, s_size: usize) -> Self {
        let m_frames = ((m_size as f64) / (page_size as f64)).ceil() as usize;
        let s_frames = ((s_size as f64) / (page_size as f64)).ceil() as usize;
        System {
            processes: Vec::new(),
            page_size,
            m: vec![None; m_frames],
            s: vec![None; s_frames],
        }
    }

    fn find_page(&self, pid_to_find: u16, page_index: usize) -> Option<(Memory, usize)> {
        for m_index in 0..self.m.len() {
            if let Some(ProcessFrame {
                pid,
                index,
                size: _,
            }) = self.m[m_index]
            {
                if pid == pid_to_find && index == page_index {
                    return Some((Memory::Real, m_index));
                }
            }
        }

        for s_index in 0..self.s.len() {
            if let Some(ProcessFrame {
                pid,
                index,
                size: _,
            }) = self.m[s_index]
            {
                if pid == pid_to_find && index == page_index {
                    return Some((Memory::Swap, s_index));
                }
            }
        }

        None
    }
}

pub trait MemoryAdministrationSystem {
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
