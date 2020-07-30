use std::ops::Range;

use crate::algorithm::PageReplacementAlgorithm;
use crate::process::{Process, ProcessFrame};
use crate::Instruction;

const FREE_PAGE_TIME: f64 = 0.1;

#[derive(Debug)]
pub struct System {
    algorithm: PageReplacementAlgorithm,
    time: f64,
    processes: Vec<Process>,
    page_size: usize,
    real_mem: Vec<Option<ProcessFrame>>,
    virt_mem: Vec<Option<ProcessFrame>>,
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
            time: 0.0,
            processes: Vec::new(),
            page_size,
            real_mem: vec![None; num_real_frames],
            virt_mem: vec![None; num_virtual_frames],
        }
    }

    pub fn process_instruction(&mut self, instruction: &Instruction) {
        let time = match instruction {
            Instruction::Process { pid, size } => {
                if *size as usize <= self.real_mem.len() * self.page_size {
                    self.process(*pid, *size)
                } else {
                    println!(
                        "La instrucción no se ha ejecutado ya que el proceso no cabe en memoria"
                    );
                    println!("Disminuya el tamaño del proceso o aumente el tamaño de la memoria con la opción -m");
                    0.0
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
                    0.0
                }
            }
            Instruction::Free { pid } => {
                if self.is_valid_pid(*pid) {
                    self.free(*pid)
                } else {
                    0.0
                }
            }
            Instruction::End() => {
                self.end();
                0.0
            }
            Instruction::Comment(_) | Instruction::Exit() => 0.0,
        };
        println!("La instrucción tomó {} segundos", time);
        self.time += time;
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

    fn process(&mut self, pid: u16, total_size: u16) -> f64 {
        let pages_needed = (total_size as f64 / self.page_size as f64).ceil() as usize;
        println!(
            "Se asignaron {} bytes ({} páginas) al proceso {}",
            total_size, pages_needed, pid,
        );
        let process = Process {
            pid,
            page_faults: 0,
            life: (self.time..-1.0),
        };
        let mut size_left = total_size as usize;
        loop {
            // TODO: Allocate frame for process
            // TODO: If needed, handle page fault
            let page_to_be_replaced = match self.algorithm {
                PageReplacementAlgorithm::FIFO => self.fifo_replace_page(),
                PageReplacementAlgorithm::LRU => self.lru_replace_page(),
            };
            if size_left > self.page_size {
                size_left -= self.page_size;
            } else {
                break;
            }
        }
        self.processes.push(process);
        0.0
    }

    fn access(&mut self, pid_to_access: u16, virtual_address: u16, modifies: bool) -> f64 {
        let mut time = 0.0;
        let page_index = (virtual_address as f64 / self.page_size as f64).floor() as usize;
        match self.find_page(pid_to_access, page_index) {
            Some(Frame(Memory::Real, index)) => {
                time += 0.1;
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

    fn free(&mut self, pid_to_free: u16) -> f64 {
        let frame_is_freed = |index: usize,
                              maybe_frame: &mut Option<ProcessFrame>,
                              ranges: &mut Vec<Range<usize>>| {
            match maybe_frame {
                Some(ProcessFrame {
                    pid,
                    index: _,
                    size: _,
                    last_accessed: _,
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

        let mut time = 0.0;
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
        let mut finished_processes = Vec::<&Process>::new();
        for process in &self.processes {
            if process.life.end != -1.0 {
                finished_processes.push(process);
            }
        }
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

    fn fifo_replace_page(&self) -> Frame {
        unimplemented!()
    }

    fn lru_replace_page(&self) -> Frame {
        unimplemented!()
    }
}

enum Memory {
    Real,
    Virtual,
}

struct Frame(Memory, usize);
