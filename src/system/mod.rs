use std::collections::HashMap;
use std::mem::swap;
use std::ops::Range;

use crate::algorithm::PageReplacementAlgorithm;
use crate::process::{Process, ProcessPage, PID};
use crate::time::Time;
use crate::util;
use crate::Instruction;

mod algorithms;
mod helpers;

const ACCESS_PAGE_TIME: Time = Time::from_miliseconds(100);
const FREE_PAGE_TIME: Time = Time::from_miliseconds(100);
const LOAD_PAGE_TIME: Time = Time::from_miliseconds(1000);
const MODIFY_PAGE_TIME: Time = Time::from_miliseconds(100);
const SWAP_PAGE_TIME: Time = Time::from_miliseconds(1000);

#[derive(Debug)]
pub struct System {
    algorithm: PageReplacementAlgorithm,
    time: Time,
    processes: HashMap<PID, Process>,
    page_size: usize,
    real_mem: Vec<Option<ProcessPage>>,
    virt_mem: Vec<Option<ProcessPage>>,
}

impl System {
    pub fn new(
        algorithm: PageReplacementAlgorithm,
        page_size: usize,
        real_mem_size: usize,
        virtual_mem_size: usize,
    ) -> Self {
        let num_real_frames = ((real_mem_size as f64) / (page_size as f64)).ceil() as usize;
        let num_virtual_frames = ((virtual_mem_size as f64) / (page_size as f64)).ceil() as usize;
        System {
            algorithm,
            time: Time::new(),
            processes: HashMap::new(),
            page_size,
            real_mem: vec![None; num_real_frames],
            virt_mem: vec![None; num_virtual_frames],
        }
    }

    pub fn process_instruction(&mut self, instruction: &Instruction) {
        let maybe_time_offset = match instruction {
            Instruction::Process { pid, size } => {
                if self.processes.get(pid).is_some() {
                    Err(format!("Ya existe un proceso con el pid {}", *pid))
                } else if *size > self.real_mem.len() * self.page_size {
                    Err(format!("El tamaño del proceso ({} bytes) es mayor al de la memoria real ({} bytes)", *size, self.real_mem.len() * self.page_size))
                } else {
                    Ok(self.process(*pid, *size))
                }
            }
            Instruction::Access {
                pid,
                address,
                modifies,
            } => {
                if self.processes.get(pid).is_none() {
                    Err(format!("No existe un proceso con el pid {}", *pid))
                } else if !self.processes.get(pid).unwrap().includes_address(*address) {
                    Err(format!(
                        "El proceso {} no contiene la dirección virtual {}",
                        *pid, *address
                    ))
                } else {
                    Ok(self.access(*pid, *address, *modifies))
                }
            }
            Instruction::Free { pid } => {
                if self.processes.get(pid).is_none() {
                    Err(format!("Ya existe un proceso con el pid {}", *pid))
                } else {
                    Ok(self.free(*pid))
                }
            }
            Instruction::End() => {
                self.end();
                Ok(Time::new())
            }
            Instruction::Comment(_) | Instruction::Exit() => Ok(Time::new()),
        };
        match maybe_time_offset {
            Ok(time_offset) => {
                println!("La instrucción tomó {}", time_offset);
                self.time += time_offset;
            }
            Err(error_message) => println!("Error: {}", error_message),
        }
        println!();
    }

    fn process(&mut self, pid: PID, total_size: usize) -> Time {
        let mut new_process = Process::new(pid, total_size);
        let pages_needed = new_process.num_pages(self.page_size);
        println!(
            "Se asignaron {} bytes ({} páginas) al proceso {}",
            total_size, pages_needed, pid,
        );
        let mut time_offset = Time::new();
        for page_index in 0..pages_needed {
            let empty_frame_index = match self.find_empty_frame(Memory::Real) {
                Ok(index) => index,
                Err(_) => self.swap_out_page(&mut time_offset),
            };
            self.real_mem[empty_frame_index] = Some(ProcessPage {
                pid,
                index: page_index,
                created: self.time + time_offset,
                accessed: self.time + time_offset,
            });
            time_offset += LOAD_PAGE_TIME;
        }
        new_process.set_birth(self.time + time_offset);
        self.processes.insert(pid, new_process);
        time_offset
    }

    fn access(&mut self, pid_to_access: PID, virtual_address: usize, modifies: bool) -> Time {
        let mut time_offset = Time::new();
        let process_page_index = virtual_address / self.page_size;
        let real_mem_index = match self.find_page(pid_to_access, process_page_index) {
            Frame(Memory::Real, index) => index,
            Frame(Memory::Virtual, index) => {
                self.processes
                    .get_mut(&pid_to_access)
                    .unwrap()
                    .add_page_fault();
                let empty_frame_index = self.swap_out_page(&mut time_offset);
                swap(
                    &mut self.real_mem[empty_frame_index],
                    &mut self.virt_mem[index],
                );
                println!(
                    "Swap in de la página {} del proceso {}",
                    self.real_mem[empty_frame_index].as_ref().unwrap().index,
                    self.real_mem[empty_frame_index].as_ref().unwrap().pid
                );
                empty_frame_index
            }
        };
        println!(
            "Se {} la dirección {} del proceso {} (página {})",
            if modifies { "modificó" } else { "accedió a" },
            virtual_address,
            process_page_index,
            pid_to_access
        );
        println!(
            "Esta dirección corresponde a la dirección {} en la memoria real (marco {})",
            real_mem_index * self.page_size + (virtual_address % self.page_size),
            real_mem_index
        );
        time_offset += if modifies {
            MODIFY_PAGE_TIME
        } else {
            ACCESS_PAGE_TIME
        };
        self.real_mem[real_mem_index].as_mut().unwrap().accessed = self.time + time_offset;
        time_offset
    }

    fn free(&mut self, pid_to_free: PID) -> Time {
        let frame_is_freed = |index: usize,
                              maybe_frame: &mut Option<ProcessPage>,
                              ranges: &mut Vec<Range<usize>>| {
            match maybe_frame {
                Some(ProcessPage {
                    pid,
                    index: _,
                    created: _,
                    accessed: _,
                }) if *pid == pid_to_free => {
                    *maybe_frame = None;
                    match ranges.last_mut() {
                        Some(Range { start: _, end }) if *end == index - 1 => *end = index,
                        Some(_) | None => ranges.push(Range {
                            start: index,
                            end: index,
                        }),
                    };
                    true
                }
                Some(_) | None => false,
            }
        };

        let mut time = Time::new();
        let mut r_freed_ranges = Vec::<Range<usize>>::new();
        self.real_mem
            .iter_mut()
            .enumerate()
            .for_each(|(index, maybe_frame)| {
                if frame_is_freed(index, maybe_frame, &mut r_freed_ranges) {
                    time += FREE_PAGE_TIME;
                }
            });
        println!(
            "Se liberan de la memoria real: {}",
            util::display_ranges_vec(&r_freed_ranges)
        );

        let mut v_freed_ranges = Vec::<Range<usize>>::new();
        self.virt_mem
            .iter_mut()
            .enumerate()
            .for_each(|(index, maybe_frame)| {
                if frame_is_freed(index, maybe_frame, &mut v_freed_ranges) {
                    time += FREE_PAGE_TIME;
                }
            });
        println!(
            "Se liberan de la memoria virtual: {}",
            util::display_ranges_vec(&v_freed_ranges)
        );

        self.processes
            .get_mut(&pid_to_free)
            .unwrap()
            .set_death(self.time + time);

        time
    }

    fn end(&mut self) {
        let finished_processes: Vec<&Process> = self
            .processes
            .iter()
            .filter_map(|(_, process)| {
                if process.has_died() {
                    Some(process)
                } else {
                    None
                }
            })
            .collect();

        println!("Turnaround de cada proceso:");
        finished_processes.iter().for_each(|process| {
            println!(
                "\tProceso {}: {}, {} de turnaround",
                process.pid,
                process.display_life(),
                process.calc_turnaround()
            );
        });
        let average_turnaround = finished_processes
            .iter()
            .map(|process| process.calc_turnaround())
            .fold(0.0, |sum, turnaround| sum + f64::from(turnaround))
            / finished_processes.len() as f64
            / 1000.0;
        println!("Turnaround promedio: {} segundos", average_turnaround);

        println!("Page faults por proceso:");
        finished_processes.iter().for_each(|process| {
            println!(
                "\tProceso {}: {} page faults",
                process.pid,
                process.get_page_faults(),
            );
        });
        // TODO: Calcular núm de swaps (out e in)
    }
}

pub enum Memory {
    Real,
    Virtual,
}

pub struct Frame(Memory, usize);
