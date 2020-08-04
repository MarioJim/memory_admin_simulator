use std::collections::BTreeSet;

use rand::seq::SliceRandom;
use rand::{random, thread_rng};

use super::System;
use crate::algorithm::PageReplacementAlgorithm;
use crate::time::Time;

/// En este archivo implementamos los algoritmos
impl System {
    /// Usamos ésta función para esconder la elección de qué algoritmo usar
    pub(super) fn find_page_to_replace(&self) -> usize {
        // Dependiendo del algoritmo del sistema se llama a la función seleccionada
        match self.algorithm {
            PageReplacementAlgorithm::FIFO => self.fifo_find_page_to_replace(),
            PageReplacementAlgorithm::LRU => self.lru_find_page_to_replace(),
            PageReplacementAlgorithm::Random => self.rand_find_page_to_replace(),
        }
    }

    /// Regresa el índice del marco al que se debería reemplazar dependiendo del tiempo de creación
    /// de la página
    fn fifo_find_page_to_replace(&self) -> usize {
        self.real_memory
            .iter() // Iteramos por la memoria real
            .enumerate() // Envolvemos el marco en un tuple (índice, marco)
            .min_by_key(|(_, frame)| frame.as_ref().unwrap().get_created_time()) // Seleccionamos el mínimo por el tiempo de creación
            .unwrap()
            .0 // Regresamos sólo el índice
    }

    /// Regresa el índice del marco al que se debería reemplazar dependiendo del tiempo de acceso
    /// de la página
    fn lru_find_page_to_replace(&self) -> usize {
        // Misma implementación que fifo_find_page_to_replace sólo que seleccionamos el mínimo
        // por el tiempo de acceso
        self.real_memory
            .iter()
            .enumerate()
            .min_by_key(|(_, frame)| frame.as_ref().unwrap().get_accessed_time())
            .unwrap()
            .0
    }

    /// Regresa el índice del marco al que se debería reemplazar al azar
    fn rand_find_page_to_replace(&self) -> usize {
        // Generamos un número random y aplicamos el módulo de éste entre el tamaño de la memoria real
        random::<usize>() % self.real_memory.len()
    }

    /// Usamos ésta función para esconder la elección de qué algoritmo usar
    pub(super) fn find_n_pages_to_replace(&self, n: usize) -> BTreeSet<usize> {
        match self.algorithm {
            PageReplacementAlgorithm::FIFO => self.fifo_find_n_pages_to_replace(n),
            PageReplacementAlgorithm::LRU => self.lru_find_n_pages_to_replace(n),
            PageReplacementAlgorithm::Random => self.rand_find_n_pages_to_replace(n),
        }
    }

    /// Regresa un set de índices de marcos que se deberían reemplazar dependiendo del tiempo
    /// de creación de cada página. Recibe el tamaño del set que regresará
    fn fifo_find_n_pages_to_replace(&self, n: usize) -> BTreeSet<usize> {
        let mut page_indexes: Vec<(usize, &Time)> = self
            .real_memory
            .iter() // Iteramos por la memoria real
            .enumerate() // Envolvemos el marco en un tuple (índice, marco)
            .filter_map(|(index, frame)| {
                // Ésta función actúa como un filtro y un mapeo al mismo tiempo
                // Recibe un (índice, marco) y si el marco tiene una página dentro regresa
                // un tuple (índice, tiempo de creación de la página), y si no tiene un marco adentro
                // regresa None, por lo que este valor se filtra
                frame.as_ref().map(|page| (index, page.get_created_time()))
            })
            .collect(); // Se juntan los valores en una lista

        // Se ordenan los índices por el tiempo creados
        page_indexes.sort_unstable_by_key(|(_, &time_created)| time_created);
        // Se corta hasta el tamaño pedido
        page_indexes.truncate(n);
        // Convertimos la lista en iterador, mapeamos cada tuple a sólo el índice y convertimos el
        // iterador en un set (declarado en la firma de la función)
        page_indexes.into_iter().map(|(index, _)| index).collect()
    }

    /// Regresa un set de índices de marcos que se deberían reemplazar dependiendo del tiempo
    /// de acceso de cada página. Recibe el tamaño del set que regresará
    fn lru_find_n_pages_to_replace(&self, n: usize) -> BTreeSet<usize> {
        // Misma implementación que fifo_find_n_pages_to_replace sólo que los tuples ahora son
        // (índice, tiempo de accesp de la página)
        let mut page_indexes: Vec<(usize, &Time)> = self
            .real_memory
            .iter()
            .enumerate()
            .filter_map(|(index, frame)| {
                frame.as_ref().map(|page| (index, page.get_accessed_time()))
            })
            .collect();

        page_indexes.sort_unstable_by_key(|(_, &time_accessed)| time_accessed);
        page_indexes.truncate(n);
        page_indexes.into_iter().map(|(index, _)| index).collect()
    }

    /// Regresa un set de índices de marcos que se deberían reemplazar al azar. Recibe el tamaño
    /// del set que regresará
    fn rand_find_n_pages_to_replace(&self, n: usize) -> BTreeSet<usize> {
        let mut page_indexes: Vec<usize> = self
            .real_memory
            .iter() // Iteramos por la memoria real
            .enumerate() // Envolvemos cada marco en (índice, marco)
            // Filtramos los marcos vacíos, y los que sí tienen página los convertimos en su índice
            .filter_map(|(index, frame)| frame.as_ref().map(|_| index))
            .collect();
        // Ordenamos al azar la lista de índices
        page_indexes.shuffle(&mut thread_rng());
        // Cortamos la lista al tamaño indicado
        page_indexes.truncate(n);
        page_indexes.into_iter().collect()
    }
}
