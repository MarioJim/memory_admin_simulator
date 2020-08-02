use std::mem::swap;

use super::{Frame, Memory, System, SWAP_PAGE_TIME};
use crate::algorithm::PageReplacementAlgorithm;
use crate::process::PID;
use crate::time::Time;

impl System {
    pub fn find_page(&self, pid: PID, page_index: usize) -> Frame {
        if let Some(m_index) = self.real_memory.iter().position(|frame| match frame {
            Some(page) => page.pid == pid && page.index == page_index,
            None => false,
        }) {
            Frame(Memory::Real, m_index)
        } else if let Some(s_index) = self.swap_space.iter().position(|frame| match frame {
            Some(page) => page.pid == pid && page.index == page_index,
            None => false,
        }) {
            Frame(Memory::Swap, s_index)
        } else {
            panic!(
                "No se encontró la página {} del proceso {}",
                page_index, pid
            );
        }
    }

    pub fn get_empty_frame_index(&mut self, time_offset: &mut Time) -> usize {
        match self.find_empty_frame(Memory::Real) {
            Ok(index) => index,
            Err(_) => {
                *time_offset += SWAP_PAGE_TIME;
                let frame_index_to_be_replaced = match self.algorithm {
                    PageReplacementAlgorithm::FIFO => self.fifo_find_page_to_replace(),
                    PageReplacementAlgorithm::LRU => self.lru_find_page_to_replace(),
                    PageReplacementAlgorithm::Random => self.rand_find_page_to_replace(),
                };
                let pid = self.real_memory[frame_index_to_be_replaced]
                    .as_ref()
                    .unwrap()
                    .pid;
                self.alive_processes.get_mut(&pid).unwrap().add_swap_out();
                let empty_frame_index_in_swap = self.find_empty_frame(Memory::Swap).unwrap();
                println!(
                    "Swap out de la página {} del proceso {}",
                    self.real_memory[frame_index_to_be_replaced]
                        .as_ref()
                        .unwrap()
                        .index,
                    self.real_memory[frame_index_to_be_replaced]
                        .as_ref()
                        .unwrap()
                        .pid
                );
                swap(
                    &mut self.swap_space[empty_frame_index_in_swap],
                    &mut self.real_memory[frame_index_to_be_replaced],
                );
                frame_index_to_be_replaced
            }
        }
    }

    pub fn find_empty_frame(&self, memory: Memory) -> Result<usize, ()> {
        let maybe_empty_frame = match memory {
            Memory::Real => &self.real_memory,
            Memory::Swap => &self.swap_space,
        }
        .iter()
        .enumerate()
        .find(|(_, maybe_frame)| maybe_frame.is_none());

        match maybe_empty_frame {
            Some((index, _)) => Ok(index),
            None => Err(()),
        }
    }

    pub fn calc_free_space(&self) -> usize {
        let free_frames_accumulator =
            |acc: usize, frame: &Option<_>| if frame.is_none() { acc + 1 } else { acc };

        let free_frames = self.real_memory.iter().fold(0, free_frames_accumulator)
            + self.swap_space.iter().fold(0, free_frames_accumulator);

        self.page_size * free_frames
    }
}
