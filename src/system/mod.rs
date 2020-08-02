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
        let num_real_frames = util::ceil_div(real_mem_size, page_size);
        let num_virt_frames = util::ceil_div(virtual_mem_size, page_size);
        System {
            algorithm,
            time: Time::new(),
            processes: HashMap::new(),
            page_size,
            real_mem: (0..num_real_frames).map(|_| None).collect(),
            virt_mem: (0..num_virt_frames).map(|_| None).collect(),
        }
    }

    pub fn process_instruction(&mut self, instruction: &Instruction) {
        let maybe_time_offset = match instruction {
            Instruction::Process { pid, size } => {
                if self.processes.get(pid).is_some() {
                    Err(format!("Ya existe un proceso con el pid {}", *pid))
                } else if *size > self.calc_free_space() {
                    Err(format!(
                        "El tamaño del proceso ({} bytes) es mayor a la memoria disponible en el sistema ({} bytes)",
                        *size, self.calc_free_space()
                    ))
                } else if *size > self.real_mem.len() * self.page_size {
                    Err(format!(
                        "El tamaño del proceso ({} bytes) es mayor al de la memoria real ({} bytes)",
                        *size, self.real_mem.len() * self.page_size
                    ))
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
        };
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
                Err(_) => self.free_real_mem_frame(&mut time_offset),
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

    fn access(&mut self, pid: PID, process_address: usize, modifies: bool) -> Time {
        let mut time_offset = Time::new();
        let process_page_index = process_address / self.page_size;
        let real_mem_index = match self.find_page(pid, process_page_index) {
            Frame(Memory::Real, index) => index,
            Frame(Memory::Virtual, index) => {
                self.processes.get_mut(&pid).unwrap().add_swap_in();
                let empty_frame_index = self.free_real_mem_frame(&mut time_offset);
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
            process_address,
            pid,
            process_page_index,
        );
        println!(
            "Esta dirección corresponde a la dirección {} en la memoria real (marco de página {})",
            real_mem_index * self.page_size + (process_address % self.page_size),
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

    fn free(&mut self, pid: PID) -> Time {
        let frame_is_freed =
            |index: usize, maybe_frame: &mut Option<ProcessPage>| -> Option<usize> {
                if maybe_frame.is_some() && maybe_frame.as_ref().unwrap().pid == pid {
                    *maybe_frame = None;
                    Some(index)
                } else {
                    None
                }
            };

        let mut time_offset = Time::new();
        let mut r_freed_ranges = Vec::<Range<usize>>::new();
        self.real_mem
            .iter_mut()
            .enumerate()
            .for_each(|(index, maybe_frame)| {
                if let Some(index) = frame_is_freed(index, maybe_frame) {
                    time_offset += FREE_PAGE_TIME;
                    util::add_index_to_vec_of_ranges(index, &mut r_freed_ranges);
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
                if let Some(index) = frame_is_freed(index, maybe_frame) {
                    time_offset += FREE_PAGE_TIME;
                    util::add_index_to_vec_of_ranges(index, &mut v_freed_ranges);
                }
            });
        println!(
            "Se liberan de la memoria virtual: {}",
            util::display_ranges_vec(&v_freed_ranges)
        );
        self.processes
            .get_mut(&pid)
            .unwrap()
            .set_death(self.time + time_offset);
        time_offset
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
        let average_turnaround_in_ms = finished_processes
            .iter()
            .map(|process| process.calc_turnaround())
            .fold(0.0, |sum, turnaround| sum + f64::from(turnaround))
            / finished_processes.len() as f64;
        println!(
            "Turnaround promedio: {} segundos",
            average_turnaround_in_ms / 1000.0
        );
        println!("Swaps por proceso:");
        finished_processes.iter().for_each(|process| {
            let (swap_ins, swap_outs) = process.get_swaps();
            println!(
                "\tProceso {}:\t{} swap-ins,\t{} swap-outs",
                process.pid, swap_ins, swap_outs,
            );
        });
    }
}

pub enum Memory {
    Real,
    Virtual,
}

pub struct Frame(Memory, usize);
