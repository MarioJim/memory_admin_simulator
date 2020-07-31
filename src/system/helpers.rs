use super::{Frame, Memory, System, SWAP_PAGE_TIME};
use crate::algorithm::PageReplacementAlgorithm;
use crate::process::PID;
use crate::time::Time;

impl System {
    pub fn is_valid_pid(&self, checked_pid: PID, maybe_address: Option<usize>) -> bool {
        match (self.processes.get(&checked_pid), maybe_address) {
            (Some(process), Some(address)) => process.includes_address(address),
            (Some(_), None) => true,
            (None, _) => {
                println!("Instrucción ignorada: no existe un proceso con ese pid");
                false
            }
        }
    }

    pub fn find_page(&self, pid_to_find: PID, page_index: usize) -> Frame {
        if let Some(m_index) = self.real_mem.iter().position(|frame| match frame {
            Some(page) => page.pid == pid_to_find && page.index == page_index,
            None => false,
        }) {
            Frame(Memory::Real, m_index)
        } else if let Some(s_index) = self.virt_mem.iter().position(|frame| match frame {
            Some(page) => page.pid == pid_to_find && page.index == page_index,
            None => false,
        }) {
            Frame(Memory::Virtual, s_index)
        } else {
            panic!(
                "No se encontró la página {} del proceso {}",
                page_index, pid_to_find
            );
        }
    }

    pub fn swap_out_page(&mut self, time_offset: &mut Time) -> usize {
        *time_offset += SWAP_PAGE_TIME;
        let frame_index_to_be_replaced = match self.algorithm {
            PageReplacementAlgorithm::FIFO => self.fifo_find_page_to_replace(),
            PageReplacementAlgorithm::LRU => self.lru_find_page_to_replace(),
        };
        let empty_frame_index_in_virtual = self.find_empty_frame(Memory::Virtual).unwrap();
        println!(
            "Swap out de la página {} del proceso {}",
            self.real_mem[frame_index_to_be_replaced].unwrap().index,
            self.real_mem[frame_index_to_be_replaced].unwrap().pid
        );
        self.virt_mem[empty_frame_index_in_virtual] = self.real_mem[frame_index_to_be_replaced];
        frame_index_to_be_replaced
    }

    pub fn find_empty_frame(&self, memory: Memory) -> Result<usize, ()> {
        let maybe_empty_frame = match memory {
            Memory::Real => &self.real_mem,
            Memory::Virtual => &self.virt_mem,
        }
        .iter()
        .enumerate()
        .find(|(_, maybe_frame)| maybe_frame.is_none());

        match maybe_empty_frame {
            Some((index, _)) => Ok(index),
            None => Err(()),
        }
    }
}
