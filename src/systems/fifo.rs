use std::collections::VecDeque;
use std::ops::Range;

use super::MemoryAdministrationAlgorithm;
use super::Process;
use crate::memory::Memory;
use crate::memory::ProcessFrame;

#[derive(Debug)]
pub struct FIFOSystem<'a> {
    memory: Memory,
    time: f64,
    processes: Vec<Process>,
    queue: VecDeque<&'a ProcessFrame>,
}

impl<'a> MemoryAdministrationAlgorithm for FIFOSystem<'a> {
    fn new(page_size: u16, m_size: usize, s_size: usize) -> FIFOSystem<'a> {
        FIFOSystem {
            memory: Memory::new(page_size, m_size, s_size),
            time: 0.0,
            queue: VecDeque::new(),
            processes: Vec::new(),
        }
    }

    fn process(&mut self, pid: u16, size: u16) {
        let frames_needed = (size as f64 / self.memory.page_size as f64).ceil() as usize;
    }

    fn access(&mut self, address: u16, pid: u16, modifies: bool) {}

    fn free(&mut self, pid_to_free: u16) {
        let mut m_freed_ranges = Vec::<Range<usize>>::new();
        for index in 0..self.memory.m_size {
            match self.memory.m[index] {
                Some(ProcessFrame { pid, size: _ }) if pid == pid_to_free => {
                    self.memory.m[index] = None;
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
                .map(|r| format!("{}-{}", r.start, r.end))
                .collect::<Vec<String>>()
                .join(", ")
        );

        let mut s_freed_ranges = Vec::<Range<usize>>::new();
        for index in 0..self.memory.s_size {
            match self.memory.s[index] {
                Some(ProcessFrame { pid, size: _ }) if pid == pid_to_free => {
                    self.memory.s[index] = None;
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
            "Se liberan del área de swapping: {}",
            s_freed_ranges
                .iter()
                .map(|r| format!("{}-{}", r.start, r.end))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    fn end(&mut self) {
        println!("Reporte de salida:");
        // Calcular:
        //   Turnaround de cada proceso (desde que empezó P hasta que se terminó L)
        //   Turnaround promedio
        //   Núm de page faults por proceso (sólo ocasionados por A)
        //   Núm de swaps (out e in)
    }
}
