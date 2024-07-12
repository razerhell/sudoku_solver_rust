use std::{ sync::{ Arc, Mutex, RwLock }, thread, time::Instant };

use sudoku_solver_rust::{ str_to_vecu8, vecu8_to_str, GridTask };

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
        // println!("current tasks: {}", arc_tasks.len());
    }

    // println!("parallel time: {:?}", start.elapsed());

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
        // println!("current tasks: {}", tasks.len());
    }

    // println!("sequential time: {:?}", start.elapsed());
    let res = vecu8_to_str(arc_puzzle.read().unwrap().as_ref());
    Some(vec![res])
}

#[cfg(test)]
mod tests {
    use std::{ borrow::Borrow, string };

    use crate::{ solve_in_parallel, solve_sequentially };

    // todo: test 1 solution puzzle and 0 solution puzzle

    const CASES_1_SOLUTION: &str =
        ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4:1:652483917978162435314975628825736149791824563436519872269348751547291386183657294
2.6.3......1.65.7..471.8.5.5......29..8.194.6...42...1....428..6.93....5.7.....13:1:256734198891265374347198652514683729728519436963427581135942867689371245472856913
..45.21781...9..3....8....46..45.....7.9...128.12.35..4.......935..6.8.7.9.3..62.:1:964532178187694235235817964629451783573986412841273596416728359352169847798345621
59....147...9....8.72....3.7...4.29..2..3.8.68..17..5...5764..9.36..5...1..8....2:1:598326147314957628672481935753648291421539876869172453285764319936215784147893562
9...84.6.6.4..52.7.3..7..8.76...15...53.....1...4.96.31.5.26.9...2.4....8....371.:1:927384165684915237531672489769231548453768921218459673175826394392147856846593712
68.9.5.....3...5.84.21.87.339.72.8.........1..45..69...6.8.4..2..1..2.757...13...:1:687935241913247568452168793396721854278459316145386927569874132831692475724513689
...34...2..6.82.737..1..45..82..5.14....983..67......514.7.....9.5.3..2..3....8.6:1:851347692496582173723169458382675914514298367679413285148726539965834721237951846
6...5.....73..8.2.854.27...2.17..53.4...69..7.8....9...273.1.84.6.54...93.......1:1:612453798973618425854927163291784536435269817786135942527391684168542379349876251
..75..9.4....823.5..16....28...36.7..16..42..43.19..5.54...8....29.71.3.......6.9:1:287513964964782315351649782895236471716854293432197856543968127629471538178325649
........8..3...4...9..2..6.....79.......612...6.5.2.7...8...5...1.....2.4.5.....3:1:621943758783615492594728361142879635357461289869532174238197546916354827475286913
........2..8.1.9..5....3.4....1.93...6..3..8...37......4......53.1.7.8..2........:1:639847512478512963512693748724189356965234187183765294847921635351476829296358471
..2...7...1.....6.5......18....37.......49.....41.23....3.2.9...8.....5.6.......2:1:832416795417985263596273418951837624328649571764152389173528946289364157645791832
........7..4.2.6..8.....31......29...4..9..3...95.6....1......8..6.5.2..7......6.:1:963814527154327689827965314371482956645791832289536741512673498496158273738249165
..4..3....7..8....2.81....6..3....9..8..2....1..7....3......45....8..9....9..5..8:1:594263871671584329238197546763418295985326714142759683816932457357841962429675138
..6..1....5..3....9..4....7..1....2..3..9....4..5....13.....68....3..2....2..8..3:1:826971354754836192913425867571643928238197546469582731347219685185364279692758413
........3..1..9.6..5..8.4.....9...8...867.....1....2....6..7.2..3.8..5..4.......8:1:862714953741359862953286471374921685298675134615438297586147329137892546429563718
........5..6..87..3......9....1.7.4...7...8...4...6....9..8...3..16..4..5...2....:1:714963285926518734385274691238197546657342819149856327492785163871639452563421978
.....5..3..9....4..81.4.......7.......4..2..68...14.3.......2...4...6..79...5..1.:1:427165893539278641681349725216793458394582176875614932758431269143926587962857314";

    enum ExpectedOutput {
        SolutionsCount(usize),
        Solution(String),
    }

    struct Case {
        input: String,
        expected_output: ExpectedOutput,
        comment: String,
    }

    fn read_cases_resource(resource: &str) -> Vec<Case> {
        let mut cases = vec![];
        let resource: Vec<&str> = resource.split('\n').collect();
        for s in resource {
            let case: Vec<&str> = s.split(':').collect();
            assert_eq!(case.len(), 3);
            assert_eq!(case[1], "1");
            cases.push(Case {
                input: String::from(case[0]),
                expected_output: ExpectedOutput::Solution(String::from(case[2])),
                comment: String::new(),
            });
        }
        cases
    }

    #[test]
    fn test_solve_in_parallel_1() {
        let cases = read_cases_resource(CASES_1_SOLUTION);
        let mut failed: Vec<Case> = vec![];
        for case in cases {
            match case.expected_output {
                ExpectedOutput::Solution(expected_output) => {
                    let output = &solve_in_parallel(case.input.as_str()).unwrap()[0];
                    if !output.eq(&expected_output) {
                        let comment = format!(
                            "input: {}\n output: {}\nexpected output: {}\n",
                            case.input,
                            output,
                            expected_output
                        );
                        failed.push(Case {
                            input: case.input,
                            expected_output: ExpectedOutput::Solution(expected_output),
                            comment,
                        });
                    }
                }
                ExpectedOutput::SolutionsCount(n) => {
                    let comment = format!(
                        "input has unexpected solutions {}:{}",
                        case.input.clone(),
                        n
                    );
                    failed.push(Case {
                        input: case.input,
                        expected_output: case.expected_output,
                        comment,
                    });
                }
            }
        }

        for case in &failed {
            print!("{}", case.comment);
        }
        assert_eq!(failed.len(), 0)
    }

    #[test]
    fn test_solve_sequentially_1() {
        let cases = read_cases_resource(CASES_1_SOLUTION);
        let mut failed: Vec<Case> = vec![];
        for case in cases {
            match case.expected_output {
                ExpectedOutput::Solution(expected_output) => {
                    let output = &solve_sequentially(case.input.as_str()).unwrap()[0];
                    if !output.eq(&expected_output) {
                        let comment = format!(
                            "input: {}\n output: {}\nexpected output: {}\n",
                            case.input,
                            output,
                            expected_output
                        );
                        failed.push(Case {
                            input: case.input,
                            expected_output: ExpectedOutput::Solution(expected_output),
                            comment,
                        });
                    }
                }
                ExpectedOutput::SolutionsCount(n) => {
                    let comment = format!(
                        "input has unexpected solutions {}:{}",
                        case.input.clone(),
                        n
                    );
                    failed.push(Case {
                        input: case.input,
                        expected_output: case.expected_output,
                        comment,
                    });
                }
            }
        }

        for case in &failed {
            print!("{}", case.comment);
        }
        assert_eq!(failed.len(), 0)
    }
}
