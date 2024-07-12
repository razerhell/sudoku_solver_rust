use std::{ sync::{ Arc, Mutex, RwLock }, thread, time::Instant };

use sudoku_solver_rust::{ show, str_to_vecu8, vecu8_to_str, GridTask };

fn main() {
    let puzzle =
        ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4";

    solve_in_parallel(puzzle);
    solve_sequentially(puzzle);
}

fn solve_in_parallel(puzzle: &str) -> Option<Vec<String>> {
    let arc_puzzle = Arc::new(RwLock::new(str_to_vecu8(puzzle)));
    let tasks = GridTask::generate_tasks(Arc::clone(&arc_puzzle));

    let mut arc_tasks: Vec<Arc<Mutex<GridTask>>> = tasks
        .into_iter()
        .map(|t| Arc::new(Mutex::new(t)))
        .collect();
    let start = Instant::now();
    while arc_tasks.iter().any(|t| !t.lock().unwrap().done()) {
        let mut handles = vec![];
        arc_tasks.iter().for_each(|t| {
            let task_clone = Arc::clone(t);
            let handle = thread::spawn(move || task_clone.lock().unwrap().run());
            handles.push(handle)
        });
        handles.into_iter().for_each(|h| h.join().unwrap());
        let tasks_count_backup = arc_tasks.len();
        arc_tasks = arc_tasks
            .into_iter()
            .filter(|t| !t.lock().unwrap().updated())
            .collect();
        if arc_tasks.len() != tasks_count_backup {
            arc_tasks.iter().for_each(|t| t.lock().unwrap().reset_done());
        }
        println!("current tasks: {}", arc_tasks.len());
    }

    println!("parallel time: {:?}", start.elapsed());

    let res = vecu8_to_str(arc_puzzle.read().unwrap().as_ref());
    Some(vec![res])
}

fn solve_sequentially(puzzle: &str) -> Option<Vec<String>> {
    let arc_puzzle = Arc::new(RwLock::new(str_to_vecu8(puzzle)));
    let mut tasks = GridTask::generate_tasks(Arc::clone(&arc_puzzle));
    let start = Instant::now();

    while tasks.iter().any(|t| !t.done()) {
        tasks.iter_mut().for_each(|t| t.run());

        let tasks_count_backup = tasks.len();
        tasks = tasks
            .into_iter()
            .filter(|t| !t.updated())
            .collect();

        if tasks.len() != tasks_count_backup {
            tasks.iter_mut().for_each(|t| t.reset_done());
        }
        println!("current tasks: {}", tasks.len());
    }

    println!("sequential time: {:?}", start.elapsed());
    let res = vecu8_to_str(arc_puzzle.read().unwrap().as_ref());
    Some(vec![res])
}

#[cfg(test)]
mod tests {
    use crate::{ solve_in_parallel, solve_sequentially };

    // todo: test 1 solution puzzle and 0 solution puzzle

    #[test]
    fn test_solve_in_parallel() {
        let input =
            ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4";
        let expected_output =
            "652483917978162435314975628825736149791824563436519872269348751547291386183657294";
        assert_eq!(solve_in_parallel(input).unwrap().first().unwrap().as_str(), expected_output);
    }

    #[test]
    fn test_solve_sequentially() {
        let input =
            ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4";
        let expected_output =
            "652483917978162435314975628825736149791824563436519872269348751547291386183657294";
        assert_eq!(solve_sequentially(input).unwrap().first().unwrap().as_str(), expected_output);
    }
}
