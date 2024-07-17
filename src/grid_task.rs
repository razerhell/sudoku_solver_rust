use std::{ cell::RefCell, collections::HashMap, rc::Rc };

use crate::{ conjugate_block_index, conjugate_col_index, conjugate_row_index };

#[derive(Debug)]
pub struct GridTask {
    puzzle: Rc<RefCell<Vec<u8>>>,
    index: usize,
    done: bool,
    updated: bool,
    possible_values_map: Rc<RefCell<HashMap<usize, Rc<Vec<u8>>>>>,
}

impl GridTask {
    pub fn new(
        puzzle: Rc<RefCell<Vec<u8>>>,
        index: usize,
        possible_values_map: Rc<RefCell<HashMap<usize, Rc<Vec<u8>>>>>
    ) -> Self {
        GridTask {
            puzzle,
            index,
            done: false,
            updated: false,
            possible_values_map,
        }
    }

    pub fn generate_tasks(
        puzzle: Rc<RefCell<Vec<u8>>>,
        possible_values_map: Rc<RefCell<HashMap<usize, Rc<Vec<u8>>>>>
    ) -> Vec<GridTask> {
        let mut todo: Vec<GridTask> = Vec::new();

        puzzle
            .borrow()
            .iter()
            .enumerate()
            .for_each(|(index, num)| {
                match num {
                    0 => {
                        todo.push(
                            GridTask::new(
                                Rc::clone(&puzzle),
                                index,
                                Rc::clone(&possible_values_map)
                            )
                        )
                    }
                    _ => (),
                }
            });

        todo
    }

    pub fn possible_values(&self) -> Rc<Vec<u8>> {
        Rc::clone(self.possible_values_map.borrow().get(&self.index).unwrap())
    }

    fn calculate_possible_values(&self) -> Rc<Vec<u8>> {
        let mut possible_values = vec![];
        let mut possible_values_flag = vec![true; 10];
        let closure_update_flags = |i: &usize, possible_values_flag: &mut [bool]| {
            possible_values_flag[self.puzzle.borrow()[*i] as usize] = false;
        };

        conjugate_row_index(self.index)
            .iter()
            .for_each(|i| closure_update_flags(i, &mut possible_values_flag));
        conjugate_col_index(self.index)
            .iter()
            .for_each(|i| closure_update_flags(i, &mut possible_values_flag));
        conjugate_block_index(self.index)
            .iter()
            .for_each(|i| closure_update_flags(i, &mut possible_values_flag));

        possible_values_flag[1..10]
            .iter()
            .enumerate()
            .for_each(|(i, flag)| {
                if *flag {
                    possible_values.push((i + 1) as u8)
                }
            });

        let values_rc = Rc::new(possible_values);
        self.possible_values_map.borrow_mut().insert(self.index, Rc::clone(&values_rc));

        values_rc
    }

    fn calculate_exclusive_possible_values(&self) -> u8 {
        for indexs in vec![
            conjugate_row_index(self.index),
            conjugate_col_index(self.index),
            conjugate_block_index(self.index)
        ] {
            let mut possible_values_flag: Vec<bool> = vec![false; 10];
            self.possible_values()
                .iter()
                .for_each(|&i| {
                    possible_values_flag[i as usize] = true;
                });

            indexs
                .into_iter()
                .filter(|&i| {
                    let puzzle_rc = Rc::clone(&self.puzzle);
                    i != self.index && puzzle_rc.borrow()[i] == 0
                })
                .for_each(|i| {
                    if let Some(vs) = self.possible_values_map.borrow().get(&i) {
                        vs.iter().for_each(|&v| {
                            possible_values_flag[v as usize] = false;
                        })
                    } else {
                        possible_values_flag.fill(false);
                    }
                });
            let exclusive_possible_values = possible_values_flag
                .into_iter()
                .enumerate()
                .fold(Vec::<u8>::new(), |mut v, (i, f)| {
                    if f {
                        v.push(i as u8);
                    }
                    v
                });
            if exclusive_possible_values.len() == 1 {
                return exclusive_possible_values[0];
            }
        }

        0
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

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn run(&mut self) {
        let possible_values = self.calculate_possible_values();

        if possible_values.len() == 1 {
            self.puzzle.borrow_mut()[self.index] = possible_values[0];
            self.updated = true;
            self.done = true;
            return;
        }

        let exclusive_possible_value = self.calculate_exclusive_possible_values();
        if exclusive_possible_value != 0 {
            self.puzzle.borrow_mut()[self.index] = exclusive_possible_value;
            self.updated = true;
            self.done = true;
            return;
        }

        self.done = true;
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
            GridTask::generate_tasks(
                Rc::new(RefCell::new(str_to_vecu8(input))),
                Rc::new(RefCell::new(HashMap::<usize, Rc<Vec<u8>>>::new()))
            ).len(),
            expected_output_len
        )
    }
}
