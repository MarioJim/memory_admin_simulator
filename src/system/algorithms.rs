use std::collections::BTreeSet;

use rand::seq::SliceRandom;
use rand::thread_rng;

use super::System;
use crate::time::Time;

impl System {
    pub fn fifo_find_n_pages_to_replace(&self, n: usize) -> BTreeSet<usize> {
        let mut page_indexes: Vec<(usize, &Time)> = self
            .real_memory
            .iter()
            .enumerate()
            .filter_map(|(index, frame)| {
                frame.as_ref().map(|page| (index, page.get_created_time()))
            })
            .collect();

        page_indexes.sort_unstable_by_key(|(_, &time_created)| time_created);
        assert!(page_indexes.len() >= n, "More pages needed than available");
        page_indexes.truncate(n);

        page_indexes.into_iter().map(|(index, _)| index).collect()
    }

    pub fn lru_find_n_pages_to_replace(&self, n: usize) -> BTreeSet<usize> {
        let mut page_indexes: Vec<(usize, &Time)> = self
            .real_memory
            .iter()
            .enumerate()
            .filter_map(|(index, frame)| {
                frame.as_ref().map(|page| (index, page.get_accessed_time()))
            })
            .collect();

        page_indexes.sort_unstable_by_key(|(_, &time_accessed)| time_accessed);
        assert!(page_indexes.len() >= n, "More pages needed than available");
        page_indexes.truncate(n);

        page_indexes.into_iter().map(|(index, _)| index).collect()
    }

    pub fn rand_find_n_pages_to_replace(&self, n: usize) -> BTreeSet<usize> {
        let mut page_indexes: Vec<usize> = self
            .real_memory
            .iter()
            .enumerate()
            .filter_map(|(index, frame)| frame.as_ref().map(|_| index))
            .collect();
        page_indexes.shuffle(&mut thread_rng());
        assert!(page_indexes.len() >= n, "More pages needed than available");
        page_indexes.truncate(n);

        page_indexes.into_iter().collect()
    }
}
