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

/// Encapsula el estado de un sistema, compuesto por:
/// - algorithm: una variedad de PageReplacementAlgorithm usada para definir qué algoritmo se usa para reemplazar las páginas
/// - time: el tiempo desde el inicio del sistema, medido en segundos
/// - alive_processes: tabla de hash que mapea pid - instancias de Process
/// - dead_processes: lista de instancias de Process ya liberados de la memoria
/// - page_size: tamaño en bytes de una página
/// - real_memory: lista de Option<ProcessPage> que corresponde a la memoria real
/// - swap_space: lista de Option<ProcessPage> que corresponde al espacio de paginación
#[derive(Debug)]
pub struct System {
    algorithm: PageReplacementAlgorithm,
    time: Time,
    alive_processes: HashMap<PID, Process>,
    dead_processes: Vec<Process>,
    frame_size: usize,
    real_memory: Vec<Option<ProcessPage>>,
    swap_space: Vec<Option<ProcessPage>>,
}

impl System {
    /// Crea una instancia del sistema tomando como argumentos:
    /// - el algoritmo a usar
    /// - el tamaño de página en bytes
    /// - el tamaño de la memoria real en bytes
    /// - el tamaño del espacio swap en bytes
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
            frame_size: page_size,
            real_memory: (0..num_real_frames).map(|_| None).collect(),
            swap_space: (0..num_swap_frames).map(|_| None).collect(),
        }
    }

    /// Punto de entrada de las instrucciones
    /// Procesa una variante de Instruction pasada como referencia
    /// Dependiendo de si la instrucción es válida imprime un error o llama a la función correspondiente en el sistema
    pub fn process_instruction(&mut self, instruction: &Instruction) {
        // Cada brazo del comando match devuelve una variedad de Result:
        // - Ok(Time) con el tiempo que llevó ejecutar la instrucción
        // - Err(String) con un mensaje de error si no se pudo ejecutar la función
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
                } else if *size > self.real_memory.len() * self.frame_size {
                    Err(format!(
                        "El tamaño del proceso ({} bytes) es mayor al de la memoria real ({} bytes)",
                        *size, self.real_memory.len() * self.frame_size,
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
        // Si el Result fue:
        match maybe_time_offset {
            Ok(time_offset) => {
                // Ok -> se le suma el tiempo al tiempo del sistema
                println!("La instrucción tomó {}", time_offset);
                self.time += time_offset;
            }
            // Error -> se imprime el error en la consola
            Err(error_message) => println!("Error: {}", error_message),
        };
    }

    /// Responde a las instrucciones P válidas
    /// Recibe el pid nuevo y el tamaño en bytes del proceso
    fn process(&mut self, pid: PID, total_size: usize) -> Time {
        // Se instancia el proceso
        let mut new_process = Process::new(pid, total_size);
        // Se calcula en número de páginas necesarias
        let pages_needed = new_process.num_pages(self.frame_size);
        println!(
            "Se asignaron {} bytes ({} páginas) al proceso {}",
            total_size, pages_needed, pid,
        );
        let mut time_offset = Time::new();
        // Implementamos System::allocate_n_frames en system/helpers.rs, que devuelve un
        // HashSet de índices en los que podemos colocar las páginas.
        // La llamada a .enumerate() convierte el iterador de índices en la memoria real
        // en un iterador de (índice de página, índice en la memoria real)
        for (page_index, empty_frame_index) in self
            .allocate_n_frames(pages_needed, &mut time_offset)
            .into_iter()
            .enumerate()
        {
            // Instanciamos la página del proceso en el espacio de memoria que le corresponde
            self.real_memory[empty_frame_index] =
                Some(ProcessPage::new(pid, page_index, self.time + time_offset));
            // Añadimos al tiempo de la función el tiempo que toma cargar una página
            time_offset += LOAD_PAGE_TIME;
        }
        // Asignamos el tiempo de "nacimiento" de nuestro proceso
        new_process.set_birth(self.time + time_offset);
        // Lo agregamos a la tabla de procesos vivos del sistema
        self.alive_processes.insert(pid, new_process);
        // En Rust, si la última línea no tiene ; se trata de un return implícito
        // Estamos regresando cuánto tiempo tomó ejecutar la función
        time_offset
    }

    /// Responde a las instrucciones A válidas
    /// Recibe el pid del proceso, la dirección virtual, y si modifica la página
    fn access(&mut self, pid: PID, process_address: usize, modifies: bool) -> Time {
        let mut time_offset = Time::new();
        // Calculamos el índice de la página del proceso en la que se encuentra la dirección
        let process_page_index = process_address / self.frame_size;
        // Obtenemos el índice en memoria real de la página que buscamos
        let frame_index = match self.find_page(pid, process_page_index) {
            // Si la página ya estaba en memoria real, devolvemos el index
            Frame(Memory::Real, index) => index,
            // Pero si la página se encuentra en el espacio swap, es necesario moverla
            Frame(Memory::Swap, index) => {
                // Añadimos a la cuenta de swap-ins del proceso un swap
                self.alive_processes.get_mut(&pid).unwrap().add_swap_in();
                // Buscamos con qué marco de la memoria real deberíamos swapear la página del espacio swap
                let frame_index_to_swap = self.get_frame_index_to_swap_into(&mut time_offset);
                swap(
                    &mut self.real_memory[frame_index_to_swap],
                    &mut self.swap_space[index],
                );
                println!(
                    "Swap in de la página {} del proceso {}",
                    process_page_index, pid,
                );
                // Regresamos el índice del marco en la memoria real
                frame_index_to_swap
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
            frame_index * self.frame_size + (process_address % self.frame_size),
            frame_index,
        );
        // Añadimos al tiempo de la función dependiendo si se modificó la página
        time_offset += if modifies {
            MODIFY_PAGE_TIME
        } else {
            ACCESS_PAGE_TIME
        };
        // Actualizamos el tiempo del último acceso a la página
        self.real_memory[frame_index]
            .as_mut()
            .unwrap()
            .update_accessed_time(self.time + time_offset);
        // Regresamos el tiempo de la función
        time_offset
    }

    /// Responde a las instrucciones L válidas
    /// Recibe el pid del proceso
    fn free(&mut self, pid: PID) -> Time {
        // Declaramos una función que recibe un índice y una referencia (&) mutable (mut) a una posible
        // página del proceso, que si cumple con la condición de que la posible página es página
        // y tiene el mismo pid que el que pasamos a la función borra la página y regresa el índice
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
        // Saca el proceso de la lista de procesos vivos
        let mut now_dead_process = self.alive_processes.remove(&pid).unwrap();
        // Para facilitar la presentación de los marcos de memoria que se limpiaron usamos una lista
        // de rangos de índices en vez de imprimir cada vez que se liberaba un marco
        let mut r_freed_ranges = Vec::<Range<usize>>::new();
        // Iteramos por la memoria real, metemos cada opción de página en un tuple
        // (índice, opción de página) y
        self.real_memory
            .iter_mut()
            .enumerate()
            .for_each(|(index, maybe_frame)| {
                // Checamos si se borró la página
                if let Some(index) = frame_is_freed(index, maybe_frame) {
                    // En ese caso sumamos el tiempo de liberación de página
                    time_offset += FREE_PAGE_TIME;
                    // Usamos una función auxiliar (declarada en util.rs) para añadir al índice
                    // al rango de índices
                    util::add_index_to_vec_of_ranges(index, &mut r_freed_ranges);
                }
            });
        // Usamos otra función auxiliar para imprimir los rangos de memoria real que se limpiaron
        if let Some(ranges_str) = util::display_ranges_vec(&r_freed_ranges) {
            println!("Se liberan de la memoria real: {}", ranges_str);
        }
        // Hacemos lo mismo para el espacio swap
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
        // Asignamos el tiempo de "muerte" al proceso
        now_dead_process.set_death(self.time + time_offset);
        // Añadimos el proceso a la lista de procesos muertos
        self.dead_processes.push(now_dead_process);
        // Regresamos el tiempo de la función
        time_offset
    }

    /// Responde a las instrucciones F
    fn end(&mut self) {
        println!("Turnaround de cada proceso:");
        // Por cada proceso muerto imprimimos su vida y su turnaround
        self.dead_processes.iter().for_each(|process| {
            println!(
                "\tProceso {}:\t{:16}\t{} de turnaround",
                process.get_pid(),
                process.display_life(),
                process.calc_turnaround(),
            );
        });
        // Calculmos el tiempo de turnaround en milisegundos
        let average_turnaround_in_ms = self
            .dead_processes
            .iter() // Iteramos por los procesos muertos
            .map(|process| process.calc_turnaround()) // Mapeamos cada proceso a su turnaround
            .fold(0.0, |sum, turnaround| sum + f64::from(turnaround)) // Sumamos el turnaround de cada uno
            / self.dead_processes.len() as f64; // Lo dividimos entre el número de procesos muertos
        println!(
            "Turnaround promedio: {} segundos",
            average_turnaround_in_ms / 1000.0,
        );
        println!("Swaps por proceso:");
        // Por cada proceso muerto imprimimos su número de swap-ins y swap-outs
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

/// Usamos este enum para marcar en qué espacio de la memoria se encontraba un marco
pub enum Memory {
    Real,
    Swap,
}

/// Usamos este struct para referirnos a un punto específico de la memoria
/// Compuesto por: en qué tipo de memoria se encuentra y su índice
pub struct Frame(Memory, usize);
