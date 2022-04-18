use crate::process::{generate_process_list, Process};
use colors_transform::{Color as c, Rgb};
use fltk::{button, enums::Color, prelude::*};

#[derive(Debug)]
pub enum AlgorithmType {
    First,
    Best,
    Worst,
}

#[derive(Debug)]
pub struct Memory<'a> {
    pub chunks: Vec<MemChunk>,
    pub buttons: Vec<button::Button>,
    pub waiting_processes: Vec<Process>,
    pub running_processes: Vec<Process>,
    pub name: &'a str,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MemChunk {
    pub is_allocated: bool,
    pub mem_id: usize,
}

impl<'a> Memory<'a> {
    pub fn new(name: &str) -> Memory {
        let iter = (0..50).map(|a| MemChunk {
            is_allocated: false,
            mem_id: a,
        });
        Memory {
            chunks: Vec::from_iter(iter),
            buttons: Vec::new(),
            waiting_processes: generate_process_list(),
            running_processes: Vec::new(),
            name,
        }
    }

    pub fn mut_but(&mut self, index: usize) -> &mut button::Button {
        &mut self.buttons[index]
    }

    pub fn decrement_runtimes(&mut self) {
        self.running_processes
            .iter_mut()
            .for_each(|x| x.runtime -= 1);
    }

    pub fn get_allocated_chunks(&self) -> Vec<usize> {
        let ind = self
            .chunks
            .iter()
            .enumerate()
            .filter_map(|(index, r)| if r.is_allocated { Some(index) } else { None })
            .collect::<Vec<_>>();

        ind
    }

    pub fn get_deallocated_chunks(&self) -> Vec<usize> {
        let ind = self
            .chunks
            .iter()
            .enumerate()
            .filter_map(|(index, r)| if !r.is_allocated { Some(index) } else { None })
            .collect::<Vec<_>>();

        ind
    }

    pub fn search_for_available_space(
        &mut self,
        space_needed: i32,
        algo_type: AlgorithmType,
    ) -> i32 {
        let mut free_blocks = self.get_free_blocks();
        match algo_type {
            AlgorithmType::First => {
                free_blocks.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
            }
            AlgorithmType::Best => {
                free_blocks.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
            }
            AlgorithmType::Worst => {
                free_blocks.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
            }
        }
        for (position, available_space) in &free_blocks {
            if available_space >= &space_needed {
                return *position;
            }
        }
        -1
    }

    fn get_free_blocks(&mut self) -> Vec<(i32, i32)> {
        let mut blocks: Vec<(i32, i32)> = Vec::new();

        let mut index: i32 = 0;
        let mut start: i32;
        let mut end: i32;

        while index < self.chunks.len() as i32 {
            let is_allocated = self.chunks[index as usize].is_allocated;
            if !is_allocated {
                start = index;
                index += 1;
                loop {
                    if index >= self.chunks.len() as i32 || self.chunks[index as usize].is_allocated
                    {
                        break;
                    }
                    index += 1;
                }
                end = index;
                blocks.push((start, end - start));
            }
            index += 1;
        }
        blocks
    }

    pub fn get_process(&mut self) -> Option<Process> {
        self.waiting_processes.pop()
    }

    pub fn deallocate_memory(&mut self) {
        for process in &self.running_processes {
            if process.runtime <= 0 {
                for index in process.mem_position..process.mem_position + process.mem_size {
                    self.chunks[index as usize].is_allocated = false;
                }
            }
        }
        self.running_processes.retain(|x| x.runtime > 0);
    }

    pub fn allocate_memory(&mut self, position: i32, size: &i32, block_color: Rgb) {
        for index in position..position + size {
            let button = self.mut_but(index as usize);
            button.set_color(Color::from_rgb(
                block_color.get_red() as u8,
                block_color.get_green() as u8,
                block_color.get_blue() as u8,
            ));
            self.chunks[index as usize].is_allocated = true;
        }
    }

    pub fn peek_process(&mut self, algo_type: AlgorithmType) -> (bool, i32) {
        let mut space_needed: i32 = -1;
        if let Some(p) = &self.waiting_processes.last() {
            space_needed = p.mem_size;
        };
        if space_needed != -1 {
            let position = self.search_for_available_space(space_needed, algo_type);
            if position >= 0 {
                return (true, position);
            }
        }
        (false, -1)
    }
}
