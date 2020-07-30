use std::cmp;
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
    processes: Vec<Process>,
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
            processes: Vec::new(),
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
                if self.is_valid_pid(*pid) {
                    self.access(*pid, *address, *modifies)
                } else {
                    Time::new()
                }
            }
            Instruction::Free { pid } => {
                if self.is_valid_pid(*pid) {
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

    fn is_valid_pid(&self, checked_pid: u16) -> bool {
        match self.processes.iter().find(|p| p.pid == checked_pid) {
            Some(_) => true,
            None => {
                println!(
                    "La instrucción no se ha ejecutado ya que no existe un proceso con ese pid"
                );
                false
            }
        }
    }

    fn process(&mut self, pid: u16, total_size: u16) -> Time {
        let pages_needed = (total_size as f64 / self.page_size as f64).ceil() as usize;
        println!(
            "Se asignaron {} bytes ({} páginas) al proceso {}",
            total_size, pages_needed, pid,
        );
        let mut time_offset = Time::new();
        let mut size_left = total_size as usize;
        for page_index in 0..pages_needed {
            let empty_frame_index = match self.find_empty_frame(Memory::Real) {
                Ok(index) => index,
                Err(_) => {
                    let frame_index_to_be_replaced = match self.algorithm {
                        PageReplacementAlgorithm::FIFO => self.fifo_find_page_to_replace(),
                        PageReplacementAlgorithm::LRU => self.lru_find_page_to_replace(),
                    };
                    let empty_frame_index_in_virtual =
                        self.find_empty_frame(Memory::Virtual).unwrap();
                    self.virt_mem[empty_frame_index_in_virtual] =
                        self.real_mem[frame_index_to_be_replaced];
                    frame_index_to_be_replaced
                }
            };
            self.real_mem[empty_frame_index] = Some(ProcessPage {
                pid,
                index: page_index,
                size: cmp::min(size_left, self.page_size),
                created: self.time + time_offset,
                accessed: self.time + time_offset,
            });
            size_left -= self.page_size;
            time_offset += LOAD_PAGE_TIME;
        }
        self.processes.push(Process {
            pid,
            page_faults: 0,
            life: (self.time + time_offset..Time::max()),
        });
        time_offset
    }

    fn access(&mut self, pid_to_access: u16, virtual_address: u16, modifies: bool) -> Time {
        let mut time = Time::new();
        let page_index = (virtual_address as f64 / self.page_size as f64).floor() as usize;
        match self.find_page(pid_to_access, page_index) {
            Some(Frame(Memory::Real, index)) => {
                time += if modifies {
                    MODIFY_PAGE_TIME
                } else {
                    ACCESS_PAGE_TIME
                };
                println!(
                    "Se {} la página {} del proceso",
                    if modifies { "modificó" } else { "accedió a" },
                    page_index,
                );
                println!("Esta corresponde a la página {} de la memoria real", index);
            }
            Some(Frame(Memory::Virtual, index)) => {
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
        time
    }

    fn free(&mut self, pid_to_free: u16) -> Time {
        let frame_is_freed = |index: usize,
                              maybe_frame: &mut Option<ProcessPage>,
                              ranges: &mut Vec<Range<usize>>| {
            match maybe_frame {
                Some(ProcessPage {
                    pid,
                    index: _,
                    size: _,
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

        self.processes
            .iter_mut()
            .find(|p| p.pid == pid_to_free)
            .unwrap()
            .life
            .end = self.time + time;

        time
    }

    fn end(&mut self) {
        println!("Reporte de salida:");
        let finished_processes: Vec<&Process> = self
            .processes
            .iter()
            .filter(|process| process.life.end != Time::max())
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
