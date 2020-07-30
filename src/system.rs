use std::collections::HashMap;
use std::ops::Range;

use crate::algorithm::PageReplacementAlgorithm;
use crate::process::{Process, ProcessPage};
use crate::time::Time;
use crate::Instruction;

const ACCESS_PAGE_TIME: Time = Time::from_miliseconds(0100);
const FREE_PAGE_TIME: Time = Time::from_miliseconds(0100);
const LOAD_PAGE_TIME: Time = Time::from_miliseconds(1000);
const MODIFY_PAGE_TIME: Time = Time::from_miliseconds(0100);
const SWAP_PAGE_TIME: Time = Time::from_miliseconds(1000);

#[derive(Debug)]
pub struct System {
    algorithm: PageReplacementAlgorithm,
    time: Time,
    processes: HashMap<u16, Process>,
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
        let time_offset = match instruction {
            Instruction::Process { pid, size } => {
                if *size as usize <= self.real_mem.len() * self.page_size {
                    self.process(*pid, *size)
                } else {
                    println!(
                        "La instrucción no se ha ejecutado ya que el proceso no cabe en memoria"
                    );
                    println!("Disminuya el tamaño del proceso o aumente el tamaño de la memoria con la opción -m");
                    Time::new()
                }
            }
            Instruction::Access {
                pid,
                address,
                modifies,
            } => {
                if self.is_valid_pid(*pid, Some(*address)) {
                    self.access(*pid, *address, *modifies)
                } else {
                    Time::new()
                }
            }
            Instruction::Free { pid } => {
                if self.is_valid_pid(*pid, None) {
                    self.free(*pid)
                } else {
                    Time::new()
                }
            }
            Instruction::End() => {
                self.end();
                Time::new()
            }
            Instruction::Comment(_) | Instruction::Exit() => Time::new(),
        };
        println!("La instrucción tomó {} segundos", time_offset);
        self.time += time_offset;
    }

    fn is_valid_pid(&self, checked_pid: u16, maybe_address: Option<usize>) -> bool {
        match self.processes.get(&checked_pid) {
            Some(process) => match maybe_address {
                Some(address) => process.size > address,
                None => true,
            },
            None => {
                println!(
                    "La instrucción no se ha ejecutado ya que no existe un proceso con ese pid"
                );
                false
            }
        }
    }

    fn process(&mut self, pid: u16, total_size: usize) -> Time {
        let mut new_process = Process {
            pid,
            page_faults: 0,
            size: total_size,
            life: (Time::new()..Time::max()),
        };
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
        new_process.life.start = self.time + time_offset;
        self.processes.insert(pid, new_process);
        time_offset
    }

    fn access(&mut self, pid_to_access: u16, virtual_address: usize, modifies: bool) -> Time {
        let page_index = (virtual_address as f64 / self.page_size as f64).floor() as usize;
        match self.find_page(pid_to_access, page_index) {
            Some(Frame(Memory::Real, index)) => {
                self.access_page_at_index(index, page_index, modifies)
            }
            Some(Frame(Memory::Virtual, index)) => {
                let mut time_offset = Time::new();
                let empty_frame_index = self.swap_out_page(&mut time_offset);
                self.real_mem[empty_frame_index] = self.virt_mem[index];
                time_offset += self.access_page_at_index(empty_frame_index, page_index, modifies);
                time_offset
            }
            None => {
                println!(
                    "Error en instrucción A {{ dirección: {}, proceso: {} }}",
                    virtual_address, pid_to_access
                );
                println!("No se encontró la página {}", page_index);
                Time::new()
            }
        }
    }

    fn access_page_at_index(
        &self,
        real_mem_index: usize,
        page_index: usize,
        modifies: bool,
    ) -> Time {
        println!(
            "Se {} la página {} del proceso",
            if modifies { "modificó" } else { "accedió a" },
            page_index,
        );
        println!(
            "Esta corresponde a la página {} de la memoria real",
            real_mem_index
        );
        if modifies {
            MODIFY_PAGE_TIME
        } else {
            ACCESS_PAGE_TIME
        }
    }

    fn free(&mut self, pid_to_free: u16) -> Time {
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
            r_freed_ranges
                .iter()
                .map(|range| format!("{} a {}", range.start, range.end))
                .collect::<Vec<String>>()
                .join(", ")
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
            v_freed_ranges
                .iter()
                .map(|range| format!("{} a {}", range.start, range.end))
                .collect::<Vec<String>>()
                .join(", ")
        );

        self.processes.get_mut(&pid_to_free).unwrap().life.end = self.time + time;

        time
    }

    fn end(&mut self) {
        println!("Reporte de salida:");
        let finished_processes: Vec<&Process> = self
            .processes
            .iter()
            .filter_map(|(_, process)| {
                if process.life.end != Time::max() {
                    Some(process)
                } else {
                    None
                }
            })
            .collect();
        // TODO: Calcular turnaround de cada proceso (desde que empezó P hasta que se terminó L)
        // TODO: Calcular turnaround promedio
        // TODO: Calcular núm de page faults por proceso (sólo ocasionados por A)
        // TODO: Calcular núm de swaps (out e in)
    }

    fn find_page(&self, pid_to_find: u16, page_index: usize) -> Option<Frame> {
        if let Some(m_index) = self.real_mem.iter().position(|frame| match frame {
            Some(page) => page.pid == pid_to_find && page.index == page_index,
            None => false,
        }) {
            Some(Frame(Memory::Real, m_index))
        } else if let Some(s_index) = self.virt_mem.iter().position(|frame| match frame {
            Some(page) => page.pid == pid_to_find && page.index == page_index,
            None => false,
        }) {
            Some(Frame(Memory::Virtual, s_index))
        } else {
            None
        }
    }

    fn swap_out_page(&mut self, time_offset: &mut Time) -> usize {
        *time_offset += SWAP_PAGE_TIME;
        let frame_index_to_be_replaced = match self.algorithm {
            PageReplacementAlgorithm::FIFO => self.fifo_find_page_to_replace(),
            PageReplacementAlgorithm::LRU => self.lru_find_page_to_replace(),
        };
        let empty_frame_index_in_virtual = self.find_empty_frame(Memory::Virtual).unwrap();
        self.virt_mem[empty_frame_index_in_virtual] = self.real_mem[frame_index_to_be_replaced];
        frame_index_to_be_replaced
    }

    fn fifo_find_page_to_replace(&self) -> usize {
        let (index, _) = self
            .real_mem
            .iter()
            .enumerate()
            .min_by_key(|(_, maybe_frame)| maybe_frame.unwrap().created)
            .unwrap();
        index
    }

    fn lru_find_page_to_replace(&self) -> usize {
        let (index, _) = self
            .real_mem
            .iter()
            .enumerate()
            .min_by_key(|(_, maybe_frame)| maybe_frame.unwrap().accessed)
            .unwrap();
        index
    }

    fn find_empty_frame(&self, memory: Memory) -> Result<usize, ()> {
        let maybe_empty_frame = match memory {
            Memory::Real => &self.real_mem,
            Memory::Virtual => &self.virt_mem,
        }
        .iter()
        .enumerate()
        .find(|(_, maybe_frame)| maybe_frame.is_none());

        match maybe_empty_frame {
            Some((index, _)) => Ok(index),
            None => Err(()),
        }
    }
}

enum Memory {
    Real,
    Virtual,
}

struct Frame(Memory, usize);
