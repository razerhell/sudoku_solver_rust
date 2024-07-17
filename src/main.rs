use std::{ cell::RefCell, collections::HashMap, rc::Rc, thread, time::Duration };

use sudoku_solver_rust::{ str_to_vecu8, vecu8_to_str, GridTask };

fn main() {
    let bar = logbar::ProgressBar::new(100);
    for i in 1..33 {
        bar.inc(3);
        thread::sleep(Duration::from_secs(1));
    }
    bar.finish();
}

fn solve_sequentially(puzzle: &str) -> Vec<String> {
    let puzzle_vec = Rc::new(RefCell::new(str_to_vecu8(puzzle)));
    let possible_values_map: Rc<_> = Rc::new(RefCell::new(HashMap::<usize, Rc<Vec<u8>>>::new()));
    let mut tasks = GridTask::generate_tasks(
        Rc::clone(&puzzle_vec),
        Rc::clone(&possible_values_map)
    );

    while tasks.iter().any(|t| !t.done()) {
        tasks.iter_mut().for_each(|t| t.run());

        let tasks_count_backup = tasks.len();
        tasks = tasks
            .into_iter()
            .filter(|t| !t.updated())
            .collect();

        if tasks.len() == 0 {
            let res = vecu8_to_str(puzzle_vec.try_borrow().unwrap().as_ref());
            return vec![res];
        }
        if tasks.iter().any(|t| t.possible_values().len() == 0) {
            return vec![];
        }

        if tasks.len() != tasks_count_backup {
            tasks.iter_mut().for_each(|t| t.reset_done());
        }
    }

    if tasks.len() < 1 {
        return vec![];
    }
    let mut recursion_puzzle = String::from(puzzle);
    let recursion_base = &tasks[0];
    recursion_base
        .possible_values()
        .iter()
        .fold(Vec::<String>::new(), |mut res, &v| {
            let i = recursion_base.index();
            recursion_puzzle.replace_range(i..i + 1, v.to_string().as_str());
            // println!("{}", recursion_puzzle);
            let recursion_solutions = &mut solve_sequentially(&recursion_puzzle);
            res.append(recursion_solutions);
            res
        })
}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::solve_sequentially;

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

    const CASES_0_SOLUTION: &str =
        "1...5.2.9..7.......6.......2...........5.1..2....2.39.3.4.9...15...1...3...8...4.:0
.2....7..4....9.3.6..723.4...83.........1....9....4.6..94....5.5.....6.3.....5...:0
.......8...6.8.1..7....3..2.47..5...5..32............53....4..7......9...1.9...6.:0
....5....4....92....9....1.2..6.39....6.....79....43...94...8.......8.9.8......23:0
....9..5..1.....3...23..7....35...7.8.....2.......64...9..1.....8..6......54....7:0
1....67.9.5.........9.....8....9..3.....1....9..6..8.1..27.....7..8...4.8...6.1.7:0
1...5.2.9..71......6.......2...........5.1..2....2.39.3...9...15...1...3...8...4.:0
..3......4...8..37..8...1...4..6..73...9...1......2.....4.7..686....4...7.....5..:0
.23.....94.....1...9..3..4.2..91...4.....78..9...4...23...9...1.6..........5.....:0
1....6.8....7......9..5.......56..3.3.............38.15....1.6.....2.4..8.2..5.1.:0";

    const CASES_N_SOLUTION: &str =
        "8.........95.......67..........2.485...4.3192......736...651947...732518...894263:125
8.........59.......76.........978245...653198...412736...591......836.1....724.8.:601
8.........95.......76.........624798...593142...718536.....6417....7.983...8..265:113
8.........95.......76.........974286...512943...638517...791.6....4237.8...856...:122
8.........59.......67.........728563...691487...534912...976.3....4127.8...853...:91
8.........59.......67.........756238...418796...923415...571.6....8921.7...634...:132
8.........59.......67..........3.265...4.6198......437...753916...891742...624583:13
8.........59.......76.........832765...476198...951243....9.516...6.4982......437:9
8.........95.......67.........472968...913245...856137....9.716...6.8492......583:3
8.........95.......67.........954286...712943...638517...476..5...521.3....8934..:102
.......8...6.8.1..7....3..4.47..5...5..34............53....4..7......9...1.9...6.:633
1....6.8..5.1..2....9...1...9..1......45.....6....2..8.4.....6.7....18.....3...7.:787
...4.......6.8.1..7....2..4.3.5.......9.6.8.......7.5...2....16....2...898....2..:838
1.......2....1..3...5..34....2..1..4....8.7..4..9.......1..5.4.8.....5..9...6....:826
...4......5..8.2.6.....3...2...4....3......1...5.3.8.25...6.3.8..6....95..8......:847";

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
            match case.len() {
                3 => {
                    cases.push(Case {
                        input: String::from(case[0]),
                        expected_output: ExpectedOutput::Solution(String::from(case[2])),
                        comment: String::new(),
                    });
                }
                2 => {
                    cases.push(Case {
                        input: String::from(case[0]),
                        expected_output: ExpectedOutput::SolutionsCount(
                            usize::from_str_radix(case[1], 10).unwrap()
                        ),
                        comment: String::new(),
                    });
                }
                _ => {
                    panic!("unsupported test case");
                }
            }
        }
        cases
    }

    #[test]
    fn test_solve_sequentially_1() {
        let cases = read_cases_resource(CASES_1_SOLUTION);
        let mut failed: Vec<Case> = vec![];
        for case in cases {
            match case.expected_output {
                ExpectedOutput::Solution(expected_output) => {
                    let output = &solve_sequentially(case.input.as_str())[0];
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
    fn test_solve_sequentially_0() {
        let cases = read_cases_resource(CASES_0_SOLUTION);
        let mut failed: Vec<Case> = vec![];
        for case in cases {
            match case.expected_output {
                ExpectedOutput::SolutionsCount(n) => {
                    let output = &solve_sequentially(case.input.as_str());
                    if !output.len().eq(&n) {
                        let comment = format!(
                            "input: {}\n output: {}\nexpected output: {}\n",
                            case.input,
                            output.len(),
                            &n
                        );
                        failed.push(Case {
                            input: case.input,
                            expected_output: ExpectedOutput::SolutionsCount(n),
                            comment,
                        });
                    }
                }
                ExpectedOutput::Solution(expected_output) => {
                    let comment = format!(
                        "input has unexpected solutions {}:{}\n",
                        case.input.clone(),
                        expected_output
                    );
                    failed.push(Case {
                        input: case.input,
                        expected_output: ExpectedOutput::Solution(expected_output),
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
    fn test_solve_sequentially_multiple() {
        let cases = read_cases_resource(CASES_N_SOLUTION);
        let mut failed: Vec<Case> = vec![];
        for case in cases {
            match case.expected_output {
                ExpectedOutput::SolutionsCount(n) => {
                    let output = &solve_sequentially(case.input.as_str());
                    if !output.len().eq(&n) {
                        let comment = format!(
                            "input: {}\n output: {}\nexpected output: {}\n",
                            case.input,
                            output.len(),
                            &n
                        );
                        failed.push(Case {
                            input: case.input,
                            expected_output: ExpectedOutput::SolutionsCount(n),
                            comment,
                        });
                    }
                }
                ExpectedOutput::Solution(expected_output) => {
                    let comment = format!(
                        "input has unexpected solutions {}:{}\n",
                        case.input.clone(),
                        expected_output
                    );
                    failed.push(Case {
                        input: case.input,
                        expected_output: ExpectedOutput::Solution(expected_output),
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
