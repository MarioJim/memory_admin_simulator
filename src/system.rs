use std::ops::Range;

use crate::algorithm::PageReplacementAlgorithm;
use crate::process::{Process, ProcessFrame};
use crate::Instruction;

#[derive(Debug)]
pub struct System {
    algorithm: PageReplacementAlgorithm,
    pub time: f64,
    pub processes: Vec<Process>,
    pub page_size: usize,
    pub m: Vec<Option<ProcessFrame>>,
    pub s: Vec<Option<ProcessFrame>>,
}

impl System {
    pub fn new(
        algorithm: PageReplacementAlgorithm,
        page_size: usize,
        mem_size: usize,
        swap_size: usize,
    ) -> Self {
        let m_frames = ((mem_size as f64) / (page_size as f64)).ceil() as usize;
        let s_frames = ((swap_size as f64) / (page_size as f64)).ceil() as usize;
        System {
            algorithm,
            time: 0.0,
            processes: Vec::new(),
            page_size,
            m: vec![None; m_frames],
            s: vec![None; s_frames],
        }
    }

    pub fn process_instruction(&mut self, instruction: &Instruction) {
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

    fn find_page(&self, pid_to_find: u16, page_index: usize) -> Option<(Mem, usize)> {
        for m_index in 0..self.m.len() {
            match self.m[m_index] {
                Some(ProcessFrame {
                    pid,
                    index,
                    size: _,
                }) if pid == pid_to_find && index == page_index => {
                    return Some((Mem::Real, m_index));
                }
                _ => (),
            }
        }

        for s_index in 0..self.s.len() {
            match self.s[s_index] {
                Some(ProcessFrame {
                    pid,
                    index,
                    size: _,
                }) if pid == pid_to_find && index == page_index => {
                    return Some((Mem::Swap, s_index));
                }
                _ => (),
            }
        }

        None
    }

    fn process(&mut self, pid: u16, total_size: u16) {
        let pages_needed = (total_size as f64 / self.page_size as f64).ceil() as usize;
        println!(
            "Se asignaron {} bytes ({} páginas) al proceso {}",
            total_size, pages_needed, pid,
        );
        let process = Process {
            pid,
            page_faults: 0,
            life: Range {
                start: self.time,
                end: self.time,
            },
        };
        let mut size_left = total_size as usize;
        loop {
            // TODO: Allocate frame for process
            if size_left > self.page_size {
                size_left -= self.page_size;
            } else {
                break;
            }
        }
        self.processes.push(process);
    }

    fn access(&mut self, pid_to_access: u16, virtual_address: u16, modifies: bool) {
        let page_index = (virtual_address as f64 / self.page_size as f64).floor() as usize;
        match self.find_page(pid_to_access, page_index) {
            Some((Mem::Real, index)) => {
                self.time += 0.1;
                println!(
                    "Se {} la página {} del proceso",
                    if modifies { "modificó" } else { "accedió a" },
                    page_index,
                );
                println!("Esta corresponde a la página {} de la memoria real", index);
            }
            Some((Mem::Swap, index)) => {
                // TODO: Implementar page fault
            }
            None => {
                println!(
                    "Error en instrucción A {{ dirección: {}, proceso: {} }}",
                    virtual_address, pid_to_access
                );
                println!("No se encontró la página {}", page_index);
            }
        }
    }

    fn free(&mut self, pid_to_free: u16) {
        let mut m_freed_ranges = Vec::<Range<usize>>::new();
        for index in 0..self.m.len() {
            match self.m[index] {
                Some(ProcessFrame {
                    pid,
                    index,
                    size: _,
                }) if pid == pid_to_free => {
                    self.m[index] = None;
                    self.time += 0.1;
                    match m_freed_ranges.last_mut() {
                        Some(Range { start: _, end }) if *end == index - 1 => *end = index,
                        _ => m_freed_ranges.push(Range {
                            start: index,
                            end: index,
                        }),
                    }
                }
                Some(_) | None => (),
            }
        }
        println!(
            "Se liberan los marcos de memoria real: {}",
            m_freed_ranges
                .iter()
                .map(|range| format!("{}-{}", range.start, range.end))
                .collect::<Vec<String>>()
                .join(", ")
        );

        let mut s_freed_ranges = Vec::<Range<usize>>::new();
        for index in 0..self.s.len() {
            match self.s[index] {
                Some(ProcessFrame {
                    pid,
                    index,
                    size: _,
                }) if pid == pid_to_free => {
                    self.s[index] = None;
                    self.time += 0.1;
                    match s_freed_ranges.last_mut() {
                        Some(Range { start: _, end }) if *end == index - 1 => *end = index,
                        _ => s_freed_ranges.push(Range {
                            start: index,
                            end: index,
                        }),
                    }
                }
                Some(_) | None => (),
            }
        }
        println!(
            "Se liberan del espacio swap: {}",
            s_freed_ranges
                .iter()
                .map(|range| format!("{}-{}", range.start, range.end))
                .collect::<Vec<String>>()
                .join(", ")
        );

        for process in self.processes.iter_mut() {
            if process.pid == pid_to_free {
                process.life.end = self.time;
                break;
            }
        }
    }

    fn end(&mut self) {
        println!("Reporte de salida:");
        // TODO: Calcular turnaround de cada proceso (desde que empezó P hasta que se terminó L)
        // TODO: Calcular turnaround promedio
        // TODO: Calcular núm de page faults por proceso (sólo ocasionados por A)
        // TODO: Calcular núm de swaps (out e in)
    }
}

enum Mem {
    Real,
    Swap,
}
