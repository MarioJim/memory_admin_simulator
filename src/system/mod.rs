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
    alive_processes: HashMap<PID, Process>,
    dead_processes: Vec<Process>,
    page_size: usize,
    real_memory: Vec<Option<ProcessPage>>,
    swap_space: Vec<Option<ProcessPage>>,
}

impl System {
    pub fn new(
        algorithm: PageReplacementAlgorithm,
        page_size: usize,
        real_memory_size: usize,
        swap_space_size: usize,
    ) -> Self {
        let num_real_frames = util::ceil_div(real_memory_size, page_size);
        let num_swap_frames = util::ceil_div(swap_space_size, page_size);
        System {
            algorithm,
            time: Time::new(),
            alive_processes: HashMap::new(),
            dead_processes: Vec::new(),
            page_size,
            real_memory: (0..num_real_frames).map(|_| None).collect(),
            swap_space: (0..num_swap_frames).map(|_| None).collect(),
        }
    }

    pub fn process_instruction(&mut self, instruction: &Instruction) {
        let maybe_time_offset = match instruction {
            Instruction::Process { pid, size } => {
                if self.alive_processes.get(pid).is_some() {
                    Err(format!(
                        "Ya existe un proceso ejecutándose con el pid {}",
                        *pid,
                    ))
                } else if *size > self.calc_free_space() {
                    Err(format!(
                        "El tamaño del proceso ({} bytes) es mayor a la memoria disponible en el sistema ({} bytes)",
                        *size, self.calc_free_space(),
                    ))
                } else if *size > self.real_memory.len() * self.page_size {
                    Err(format!(
                        "El tamaño del proceso ({} bytes) es mayor al de la memoria real ({} bytes)",
                        *size, self.real_memory.len() * self.page_size,
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
                if self.alive_processes.get(pid).is_none() {
                    Err(format!(
                        "No existe un proceso ejecutándose con el pid {}",
                        *pid,
                    ))
                } else if !self
                    .alive_processes
                    .get(pid)
                    .unwrap()
                    .includes_address(*address)
                {
                    Err(format!(
                        "El proceso {} no contiene la dirección virtual {}",
                        *pid, *address,
                    ))
                } else {
                    Ok(self.access(*pid, *address, *modifies))
                }
            }
            Instruction::Free { pid } => {
                if self.alive_processes.get(pid).is_none() {
                    Err(format!(
                        "No existe un proceso ejecutándose con el pid {}",
                        *pid,
                    ))
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
        for (page_index, empty_frame_index) in self
            .get_n_empty_frame_indexes(pages_needed, &mut time_offset)
            .into_iter()
            .enumerate()
        {
            self.real_memory[empty_frame_index] =
                Some(ProcessPage::new(pid, page_index, self.time + time_offset));
            time_offset += LOAD_PAGE_TIME;
        }
        new_process.set_birth(self.time + time_offset);
        self.alive_processes.insert(pid, new_process);
        time_offset
    }

    fn access(&mut self, pid: PID, process_address: usize, modifies: bool) -> Time {
        let mut time_offset = Time::new();
        let process_page_index = process_address / self.page_size;
        let empty_frame_index = match self.find_page(pid, process_page_index) {
            Frame(Memory::Real, index) => index,
            Frame(Memory::Swap, index) => {
                self.alive_processes.get_mut(&pid).unwrap().add_swap_in();
                let empty_frame_index = self.get_empty_frame_index(&mut time_offset);
                swap(
                    &mut self.real_memory[empty_frame_index],
                    &mut self.swap_space[index],
                );
                let (page_pid, page_index) = self.real_memory[empty_frame_index]
                    .as_ref()
                    .unwrap()
                    .get_page_info();
                println!(
                    "Swap in de la página {} del proceso {}",
                    page_index, page_pid,
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
            empty_frame_index * self.page_size + (process_address % self.page_size),
            empty_frame_index,
        );
        time_offset += if modifies {
            MODIFY_PAGE_TIME
        } else {
            ACCESS_PAGE_TIME
        };
        self.real_memory[empty_frame_index]
            .as_mut()
            .unwrap()
            .update_accessed_time(self.time + time_offset);
        time_offset
    }

    fn free(&mut self, pid: PID) -> Time {
        let frame_is_freed =
            |index: usize, maybe_frame: &mut Option<ProcessPage>| -> Option<usize> {
                if maybe_frame.is_some() && maybe_frame.as_ref().unwrap().get_pid() == pid {
                    *maybe_frame = None;
                    Some(index)
                } else {
                    None
                }
            };

        let mut time_offset = Time::new();
        let mut now_dead_process = self.alive_processes.remove(&pid).unwrap();
        let mut r_freed_ranges = Vec::<Range<usize>>::new();
        self.real_memory
            .iter_mut()
            .enumerate()
            .for_each(|(index, maybe_frame)| {
                if let Some(index) = frame_is_freed(index, maybe_frame) {
                    time_offset += FREE_PAGE_TIME;
                    util::add_index_to_vec_of_ranges(index, &mut r_freed_ranges);
                }
            });
        if let Some(ranges_str) = util::display_ranges_vec(&r_freed_ranges) {
            println!("Se liberan de la memoria real: {}", ranges_str);
        }
        let mut v_freed_ranges = Vec::<Range<usize>>::new();
        self.swap_space
            .iter_mut()
            .enumerate()
            .for_each(|(index, maybe_frame)| {
                if let Some(index) = frame_is_freed(index, maybe_frame) {
                    time_offset += FREE_PAGE_TIME;
                    util::add_index_to_vec_of_ranges(index, &mut v_freed_ranges);
                }
            });
        if let Some(ranges_str) = util::display_ranges_vec(&v_freed_ranges) {
            println!("Se liberan del espacio swap: {}", ranges_str);
        }
        now_dead_process.set_death(self.time + time_offset);
        self.dead_processes.push(now_dead_process);
        time_offset
    }

    fn end(&mut self) {
        println!("Turnaround de cada proceso:");
        self.dead_processes.iter().for_each(|process| {
            println!(
                "\tProceso {}: {}, {} de turnaround",
                process.get_pid(),
                process.display_life(),
                process.calc_turnaround(),
            );
        });
        let average_turnaround_in_ms = self
            .dead_processes
            .iter()
            .map(|process| process.calc_turnaround())
            .fold(0.0, |sum, turnaround| sum + f64::from(turnaround))
            / self.dead_processes.len() as f64;
        println!(
            "Turnaround promedio: {} segundos",
            average_turnaround_in_ms / 1000.0,
        );
        println!("Swaps por proceso:");
        self.dead_processes.iter().for_each(|process| {
            let (swap_ins, swap_outs) = process.get_swaps();
            println!(
                "\tProceso {}:\t{} swap-ins,\t{} swap-outs",
                process.get_pid(),
                swap_ins,
                swap_outs,
            );
        });
    }
}

pub enum Memory {
    Real,
    Swap,
}

pub struct Frame(Memory, usize);
