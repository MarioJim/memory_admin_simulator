use super::System;

impl System {
    pub fn fifo_find_page_to_replace(&self) -> usize {
        let (index, _) = self
            .real_mem
            .iter()
            .enumerate()
            .min_by_key(|(_, maybe_frame)| maybe_frame.unwrap().created)
            .unwrap();
        index
    }

    pub fn lru_find_page_to_replace(&self) -> usize {
        let (index, _) = self
            .real_mem
            .iter()
            .enumerate()
            .min_by_key(|(_, maybe_frame)| maybe_frame.unwrap().accessed)
            .unwrap();
        index
    }
}
