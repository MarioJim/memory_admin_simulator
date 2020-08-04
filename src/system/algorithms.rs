use std::collections::BTreeSet;

use rand::seq::SliceRandom;
use rand::{random, thread_rng};

use super::System;
use crate::algorithm::PageReplacementAlgorithm;
use crate::time::Time;

impl System {
    pub(super) fn find_page_to_replace(&self) -> usize {
        match self.algorithm {
            PageReplacementAlgorithm::FIFO => self.fifo_find_page_to_replace(),
            PageReplacementAlgorithm::LRU => self.lru_find_page_to_replace(),
            PageReplacementAlgorithm::Random => self.rand_find_page_to_replace(),
        }
    }

    fn fifo_find_page_to_replace(&self) -> usize {
        self.real_memory
            .iter()
            .enumerate()
            .min_by_key(|(_, frame)| frame.as_ref().unwrap().get_created_time())
            .unwrap()
            .0
    }

    fn lru_find_page_to_replace(&self) -> usize {
        self.real_memory
            .iter()
            .enumerate()
            .min_by_key(|(_, frame)| frame.as_ref().unwrap().get_accessed_time())
            .unwrap()
            .0
    }

    fn rand_find_page_to_replace(&self) -> usize {
        random::<usize>() % self.real_memory.len()
    }

    pub(super) fn find_n_pages_to_replace(&self, n: usize) -> BTreeSet<usize> {
        match self.algorithm {
            PageReplacementAlgorithm::FIFO => self.fifo_find_n_pages_to_replace(n),
            PageReplacementAlgorithm::LRU => self.lru_find_n_pages_to_replace(n),
            PageReplacementAlgorithm::Random => self.rand_find_n_pages_to_replace(n),
        }
    }

    fn fifo_find_n_pages_to_replace(&self, n: usize) -> BTreeSet<usize> {
        let mut page_indexes: Vec<(usize, &Time)> = self
            .real_memory
            .iter()
            .enumerate()
            .filter_map(|(index, frame)| {
                frame.as_ref().map(|page| (index, page.get_created_time()))
            })
            .collect();

        page_indexes.sort_unstable_by_key(|(_, &time_created)| time_created);
        page_indexes.truncate(n);

        page_indexes.into_iter().map(|(index, _)| index).collect()
    }

    fn lru_find_n_pages_to_replace(&self, n: usize) -> BTreeSet<usize> {
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

    fn rand_find_n_pages_to_replace(&self, n: usize) -> BTreeSet<usize> {
        let mut page_indexes: Vec<usize> = self
            .real_memory
            .iter()
            .enumerate()
            .filter_map(|(index, frame)| frame.as_ref().map(|_| index))
            .collect();
        page_indexes.shuffle(&mut thread_rng());
        page_indexes.truncate(n);

        page_indexes.into_iter().collect()
    }
}
