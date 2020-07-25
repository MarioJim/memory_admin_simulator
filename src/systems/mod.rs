use std::ops::Range;

pub mod fifo;
pub mod lru;

use crate::Instruction;

#[derive(Debug)]
pub struct Process {
    pid: u16,
    life: Range<f64>,
    page_faults: u16,
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
