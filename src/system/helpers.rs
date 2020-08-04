use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::mem::swap;
use std::ops::Range;

use super::{Frame, Memory, System, SWAP_PAGE_TIME};
use crate::process::{ProcessPage, PID};
use crate::time::Time;
use crate::util;

/// En éste archivo implementamos funciones auxiliares
/// con el fin de aligerar el archivo principal (mod.rs)
impl System {
    /// Encuentra una página especificando el pid y la índice de ella
    /// Devuelve un Frame (como las coordenadas de un marco de página, declarado al final de mod.rs)
    pub(super) fn find_page(&self, pid: PID, page_index: usize) -> Frame {
        // Usamos una función que, pasándole una referencia a un marco de página, devuelve true si el
        // marco contiene una página, y la página tiene el mismo pid e índice que los proporcionados
        // a la función que la engloba
        let frame_meets_conditions = |frame: &Option<ProcessPage>| -> bool {
            frame.is_some() && frame.as_ref().unwrap().get_page_info() == (pid, page_index)
        };

        // Iteramos por la memoria real buscando si algún marco cumple con las condiciones
        if let Some(m_index) = self.real_memory.iter().position(frame_meets_conditions) {
            // En caso de encontrar uno regresamos un Frame con sus datos (una línea sin ; es un
            // return implícito en Rust)
            Frame(Memory::Real, m_index)
        } else if let Some(s_index) = self.swap_space.iter().position(frame_meets_conditions) {
            Frame(Memory::Swap, s_index)
        } else {
            // Si no encontramos la página significa que la memoria se corrompió por lo que provocamos un crash
            panic!(
                "No se encontró la página {} del proceso {}",
                page_index, pid,
            );
        }
    }

    /// Si la página encontrada en una instrucción A está en espacio swap es necesario
    /// moverla a memoria real. Esta función busca qué espacio de la memoria real "le toca"
    /// ser swappeado.
    /// Recibe una referencia (&) mutable (mut) a una instancia de tiempo para que en caso
    /// de ser necesario añada el tiempo por swappear una página
    pub(super) fn get_frame_index_to_swap_into(&mut self, time_offset: &mut Time) -> usize {
        // Iteramos por la memoria real buscando un marco de página vacío (que no tenga página)
        match self
            .real_memory
            .iter()
            .enumerate()
            .find(|(_, frame)| frame.is_none())
        {
            // Si encontramos un espacio vacío, regresamos el índice
            Some((index, _)) => return index,
            None => {
                // En otro caso añadimos a la referencia de tiempo, el tiempo de swappear una página
                *time_offset += SWAP_PAGE_TIME;
                // Obtenemos el índice de marco en la memoria real al que "le toca ser swappeado"
                let frame_index_to_be_replaced = self.find_page_to_replace();
                // Obtenemos información sobre la página actualmente en el marco
                let (pid, page_index) = self.real_memory[frame_index_to_be_replaced]
                    .as_ref()
                    .unwrap()
                    .get_page_info();
                // Añadimos un swap-out al proceso al que le pertenece esa página
                self.alive_processes.get_mut(&pid).unwrap().add_swap_out();
                println!("Swap out de la página {} del proceso {}", page_index, pid);
                // Regresamos el índice del marco
                frame_index_to_be_replaced
            }
        }
    }

    /// Usamos esta función para obtener una lista de índices de marcos en memoria real en los que
    /// asignaremos nuestro proceso (por una instrucción P)
    /// Recibe un número n (el número de marcos necesarios) y una referencia a una instancia de tiempo
    pub(super) fn allocate_n_frames(&mut self, n: usize, time_offset: &mut Time) -> Vec<usize> {
        // Generamos un set de índices iterando por la memoria real, filtrando los marcos que si
        // tienen página, y mapeando los marcos vacíos a sus índices
        let mut set_of_indexes = BTreeSet::from_iter(
            self.real_memory
                .iter()
                .enumerate()
                .filter_map(|(index, frame)| match frame {
                    Some(_) => None,
                    None => Some(index),
                })
                .into_iter(),
        );
        // Si tuvimos espacio suficiente en memoria real (el número de marcos vacíos es mayor a los
        // necesarios para el nuevo proceso) convertimos el set en lista, cortamos la lista al
        // tamaño requerido y regresamos la lista
        if set_of_indexes.len() >= n {
            let mut result = Vec::from_iter(set_of_indexes.into_iter());
            result.truncate(n);
            return result;
        }
        // En otro caso pedimos el número de índices restantes a la función find_n_pages_to_replace,
        // declarada en system/algorithms.rs que devuelve un set de índices
        let frame_indexes = self.find_n_pages_to_replace(n - set_of_indexes.len());
        // Usamos una tabla hash mapeando el pid del proceso con un vector de rangos de índices
        // para facilitar imprimir a qué páginas fue necesario hacerle swap-out
        let mut swapped_out_ranges = HashMap::<PID, Vec<Range<usize>>>::new();
        // Por cada índice de marco
        for frame_index_to_be_replaced in frame_indexes {
            // Añadimos el tiempo para hacerle swap-out
            *time_offset += SWAP_PAGE_TIME;
            // Obtenemos la información de la página que se encuentra en el marco
            let (pid, page_index) = self.real_memory[frame_index_to_be_replaced]
                .as_ref()
                .unwrap()
                .get_page_info();
            // Añadimos la información de la página a nuestra tabla hash
            match swapped_out_ranges.get_mut(&pid) {
                Some(vec_of_ranges) => util::add_index_to_vec_of_ranges(page_index, vec_of_ranges),
                None => {
                    let mut new_vec_of_ranges = Vec::<Range<usize>>::new();
                    util::add_index_to_vec_of_ranges(page_index, &mut new_vec_of_ranges);
                    swapped_out_ranges.insert(pid, new_vec_of_ranges);
                }
            }
            // Añadimos un swap out al proceso correspondiente
            self.alive_processes.get_mut(&pid).unwrap().add_swap_out();
            // Buscamos un marco en la memoria virtual vacío y obtenemos su índice
            let (empty_frame_index_in_swap, _) = self
                .swap_space
                .iter()
                .enumerate()
                .find(|(_, maybe_frame)| maybe_frame.is_none())
                .unwrap();
            // Swapeamos la página de memoria real al marco vacío del espacio swap
            swap(
                &mut self.swap_space[empty_frame_index_in_swap],
                &mut self.real_memory[frame_index_to_be_replaced],
            );
            // Añadimos el índice al set de índices que declaramos al inicio de la función
            set_of_indexes.insert(frame_index_to_be_replaced);
        }
        // Por cada pid en la tabla hash imprimimos un string de qué rangos de páginas se swapearon
        swapped_out_ranges.iter().for_each(|(pid, ranges)| {
            if let Some(ranges_str) = util::display_ranges_vec(&ranges) {
                println!("Swap out de páginas del proceso {}: {}", pid, ranges_str);
            }
        });
        // Convertimos el set en un iterador y después en un vector (por la firma de la función)
        set_of_indexes.into_iter().collect()
    }

    /// Calcula el espacio libre en el sistema en bytes
    pub(super) fn calc_free_space(&self) -> usize {
        // Declaramos una función que recibe un número y un marco, y si el marco está vacío
        // regresa el número más uno, y si no el número
        let free_frames_accumulator =
            |acc: usize, frame: &Option<_>| if frame.is_none() { acc + 1 } else { acc };

        // Calculamos el número de marcos vacíos en memoria real y en espacio swap
        let free_frames = self.real_memory.iter().fold(0, free_frames_accumulator)
            + self.swap_space.iter().fold(0, free_frames_accumulator);

        // Regresamos el número de marcos vacíos por el tamaño de cada marco
        self.frame_size * free_frames
    }
}
