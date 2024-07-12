use std::sync::{ Arc, RwLock };

use crate::{ same_block_index, same_col_index, same_row_index };

#[derive(Debug)]
pub struct GridTask {
    puzzle: Arc<RwLock<Vec<u8>>>,
    index: usize,
    done: bool,
    updated: bool,
}

impl GridTask {
    pub fn new(arc_puzzle: Arc<RwLock<Vec<u8>>>, index: usize) -> Self {
        GridTask {
            puzzle: arc_puzzle,
            index,
            done: false,
            updated: false,
        }
    }

    pub fn generate_tasks(arc_puzzle: Arc<RwLock<Vec<u8>>>) -> Vec<GridTask> {
        let mut todo: Vec<GridTask> = Vec::new();

        arc_puzzle
            .read()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(index, num)| {
                match num {
                    0 => { todo.push(GridTask::new(Arc::clone(&arc_puzzle), index)) }
                    _ => (),
                }
            });

        todo
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn reset_done(&mut self) {
        self.done = false;
    }

    pub fn updated(&self) -> bool {
        self.updated
    }

    pub fn run(&mut self) {
        let puzzle_read = self.puzzle.read().unwrap();
        let mut possible_values_flag = vec![true; 10];
        let closure_update_flags = |i: &usize, possible_values_flag: &mut [bool]| {
            possible_values_flag[puzzle_read[*i] as usize] = false;
        };

        same_row_index(self.index)
            .iter()
            .for_each(|i| closure_update_flags(i, &mut possible_values_flag));
        same_col_index(self.index)
            .iter()
            .for_each(|i| closure_update_flags(i, &mut possible_values_flag));
        same_block_index(self.index)
            .iter()
            .for_each(|i| closure_update_flags(i, &mut possible_values_flag));

        let mut posslble_values: Vec<u8> = vec![];
        possible_values_flag
            .iter()
            .enumerate()
            .for_each(|(i, flag)| {
                if *flag {
                    posslble_values.push(i as u8)
                }
            });

        if posslble_values.len() == 1 {
            std::mem::drop(puzzle_read);
            let mut puzzle_write = self.puzzle.write().unwrap();
            puzzle_write[self.index] = *posslble_values.first().unwrap();
            self.updated = true;
        } else {
            self.done = true;
        }
    }
}

impl Default for GridTask {
    fn default() -> Self {
        GridTask {
            puzzle: Arc::new(RwLock::new(vec![])),
            index: 0,
            done: false,
            updated: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::str_to_vecu8;

    use super::*;

    #[test]
    fn test_generate_tasks() {
        let input =
            ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4";
        let expected_output_len = 47;
        assert_eq!(
            GridTask::generate_tasks(Arc::new(RwLock::new(str_to_vecu8(input)))).len(),
            expected_output_len
        )
    }
}
