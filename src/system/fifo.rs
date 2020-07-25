use std::ops::Range;

use super::{Memory, MemoryAdministrationSystem, Process, ProcessFrame, System};

#[derive(Debug)]
pub struct FIFOSystem {
    system: System,
    time: f64,
}

impl MemoryAdministrationSystem for FIFOSystem {
    fn new(page_size: u16, m_size: usize, s_size: usize) -> FIFOSystem {
        FIFOSystem {
            system: System::new(page_size, m_size, s_size),
            time: 0.0,
        }
    }

    fn process(&mut self, pid: u16, size: u16) {
        let pages_needed = (size as f64 / self.system.page_size as f64).ceil() as usize;
        let process = Process {
            pid,
            page_faults: 0,
            life: Range {
                start: self.time,
                end: self.time,
            },
        };
        let mut size_left = size;
        loop {
            // TODO: Allocate frame for process
            if size_left > self.system.page_size {
                size_left -= self.system.page_size;
            } else {
                break;
            }
        }
        // TODO: Implementar
    }

    fn access(&mut self, address: u16, pid: u16, modifies: bool) {
        let page_index = (address as f64 / self.system.page_size as f64).floor() as usize;
        match self.system.find_page(pid, page_index) {
            Some((Memory::Real, index)) => {
                self.time += 0.1;
                println!(
                    "Se {} la página {} del proceso",
                    if modifies { "modificó" } else { "accedió a" },
                    page_index,
                );
                println!("Esta corresponde a la página {} de la memoria real", index);
            }
            Some((Memory::Swap, index)) => {
                // TODO: Implementar page fault
            }
            None => {
                println!(
                    "Error en instrucción A {{ dirección: {}, proceso: {} }}",
                    address, pid
                );
                println!("No se encontró la página {}", page_index);
            }
        }
    }

    fn free(&mut self, pid_to_free: u16) {
        let mut m_freed_ranges = Vec::<Range<usize>>::new();
        for index in 0..self.system.m.len() {
            match self.system.m[index] {
                Some(ProcessFrame {
                    pid,
                    index,
                    size: _,
                }) if pid == pid_to_free => {
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
                .map(|range| format!("{}-{}", range.start, range.end))
                .collect::<Vec<String>>()
                .join(", ")
        );

        let mut s_freed_ranges = Vec::<Range<usize>>::new();
        for index in 0..self.system.s.len() {
            match self.system.s[index] {
                Some(ProcessFrame {
                    pid,
                    index,
                    size: _,
                }) if pid == pid_to_free => {
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
            "Se liberan del espacio swap: {}",
            s_freed_ranges
                .iter()
                .map(|range| format!("{}-{}", range.start, range.end))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    fn end(&mut self) {
        println!("Reporte de salida:");
        // TODO: Calcular turnaround de cada proceso (desde que empezó P hasta que se terminó L)
        // TODO: Calcular turnaround promedio
        // TODO: Calcular núm de page faults por proceso (sólo ocasionados por A)
        // TODO: Calcular núm de swaps (out e in)
    }
}
