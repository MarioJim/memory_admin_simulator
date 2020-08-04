use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::mem::swap;
use std::ops::Range;

use super::{Frame, Memory, System, SWAP_PAGE_TIME};
use crate::process::{ProcessPage, PID};
use crate::time::Time;
use crate::util;

impl System {
    pub(super) fn find_page(&self, pid: PID, page_index: usize) -> Frame {
        let frame_meets_conditions = |frame: &Option<ProcessPage>| -> bool {
            frame.is_some() && frame.as_ref().unwrap().get_page_info() == (pid, page_index)
        };

        if let Some(m_index) = self.real_memory.iter().position(frame_meets_conditions) {
            Frame(Memory::Real, m_index)
        } else if let Some(s_index) = self.swap_space.iter().position(frame_meets_conditions) {
            Frame(Memory::Swap, s_index)
        } else {
            panic!(
                "No se encontró la página {} del proceso {}",
                page_index, pid,
            );
        }
    }

    pub(super) fn get_frame_index_to_swap_into(&mut self, time_offset: &mut Time) -> usize {
        match self
            .real_memory
            .iter()
            .enumerate()
            .find(|(_, frame)| frame.is_none())
        {
            Some((index, _)) => return index,
            None => {
                *time_offset += SWAP_PAGE_TIME;
                let frame_index_to_be_replaced = self.find_page_to_replace();
                let (pid, page_index) = self.real_memory[frame_index_to_be_replaced]
                    .as_ref()
                    .unwrap()
                    .get_page_info();
                self.alive_processes.get_mut(&pid).unwrap().add_swap_out();
                println!("Swap out de la página {} del proceso {}", page_index, pid);
                frame_index_to_be_replaced
            }
        }
    }

    pub(super) fn allocate_n_frames(&mut self, n: usize, time_offset: &mut Time) -> Vec<usize> {
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
        if set_of_indexes.len() >= n {
            let mut result = Vec::from_iter(set_of_indexes.into_iter());
            result.truncate(n);
            return result;
        }
        let frame_indexes = self.find_n_pages_to_replace(n - set_of_indexes.len());
        let mut swapped_out_ranges = HashMap::<PID, Vec<Range<usize>>>::new();
        for frame_index_to_be_replaced in frame_indexes {
            *time_offset += SWAP_PAGE_TIME;
            let (pid, page_index) = self.real_memory[frame_index_to_be_replaced]
                .as_ref()
                .unwrap()
                .get_page_info();
            match swapped_out_ranges.get_mut(&pid) {
                Some(vec_of_ranges) => util::add_index_to_vec_of_ranges(page_index, vec_of_ranges),
                None => {
                    let mut new_vec_of_ranges = Vec::<Range<usize>>::new();
                    util::add_index_to_vec_of_ranges(page_index, &mut new_vec_of_ranges);
                    swapped_out_ranges.insert(pid, new_vec_of_ranges);
                }
            }
            self.alive_processes.get_mut(&pid).unwrap().add_swap_out();
            let (empty_frame_index_in_swap, _) = self
                .swap_space
                .iter()
                .enumerate()
                .find(|(_, maybe_frame)| maybe_frame.is_none())
                .unwrap();
            swap(
                &mut self.swap_space[empty_frame_index_in_swap],
                &mut self.real_memory[frame_index_to_be_replaced],
            );
            set_of_indexes.insert(frame_index_to_be_replaced);
        }
        swapped_out_ranges.iter().for_each(|(pid, ranges)| {
            if let Some(ranges_str) = util::display_ranges_vec(&ranges) {
                println!("Swap out de páginas del proceso {}: {}", pid, ranges_str);
            }
        });
        let mut vec_of_indexes = Vec::from_iter(set_of_indexes.into_iter());
        vec_of_indexes.sort_unstable();
        vec_of_indexes
    }

    pub(super) fn calc_free_space(&self) -> usize {
        let free_frames_accumulator =
            |acc: usize, frame: &Option<_>| if frame.is_none() { acc + 1 } else { acc };

        let free_frames = self.real_memory.iter().fold(0, free_frames_accumulator)
            + self.swap_space.iter().fold(0, free_frames_accumulator);

        self.page_size * free_frames
    }
}
