use rand::random;

use super::System;

impl System {
    pub fn fifo_find_page_to_replace(&self) -> usize {
        let (index, _) = self
            .real_memory
            .iter()
            .enumerate()
            .min_by_key(|(_, maybe_frame)| maybe_frame.as_ref().unwrap().get_created_time())
            .unwrap();
        index
    }

    pub fn lru_find_page_to_replace(&self) -> usize {
        let (index, _) = self
            .real_memory
            .iter()
            .enumerate()
            .min_by_key(|(_, maybe_frame)| maybe_frame.as_ref().unwrap().get_accessed_time())
            .unwrap();
        index
    }

    pub fn rand_find_page_to_replace(&self) -> usize {
        random::<usize>() % self.real_memory.len()
    }
}
