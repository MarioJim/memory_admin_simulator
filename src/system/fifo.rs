use std::collections::VecDeque;
use std::ops::Range;

use super::MemoryAdministrationAlgorithm;
use super::ProcessFrame;
use super::System;

#[derive(Debug)]
pub struct FIFOSystem<'a> {
    system: System,
    time: f64,
    queue: VecDeque<&'a ProcessFrame>,
}

impl<'a> MemoryAdministrationAlgorithm for FIFOSystem<'a> {
    fn new(page_size: u16, m_size: usize, s_size: usize) -> FIFOSystem<'a> {
        FIFOSystem {
            system: System::new(page_size, m_size, s_size),
            time: 0.0,
            queue: VecDeque::new(),
        }
    }

    fn process(&mut self, pid: u16, size: u16) {
        let frames_needed = (size as f64 / self.system.page_size as f64).ceil() as usize;
    }

    fn access(&mut self, address: u16, pid: u16, modifies: bool) {}

    fn free(&mut self, pid_to_free: u16) {
        let mut m_freed_ranges = Vec::<Range<usize>>::new();
        for index in 0..self.system.m.len() {
            match self.system.m[index] {
                Some(ProcessFrame { pid, size: _ }) if pid == pid_to_free => {
                    self.system.m[index] = None;
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
        for index in 0..self.system.s.len() {
            match self.system.s[index] {
                Some(ProcessFrame { pid, size: _ }) if pid == pid_to_free => {
                    self.system.s[index] = None;
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
