use aoc25::char_grid::CharGrid;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Operator {
    Multiply,
    Add,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Problem {
    operands: Vec<i64>,
    operator: Operator,
}

impl Problem {
    pub fn new(operator: Operator) -> Self {
        Problem {
            operator,
            operands: vec![],
        }
    }

    pub fn eval(&self) -> i64 {
        let operands = &self.operands;

        if operands.is_empty() {
            return 0;
        }

        operands[1..]
            .iter()
            .fold(operands[0], |acc, operand| match self.operator {
                Operator::Multiply => acc * operand,
                Operator::Add => acc + operand,
            })
    }
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let lines: Vec<&str> = INPUT.trim().lines().collect();
    let mut problems = init_problems(&lines);
    let operand_grids = parse_operand_grids(&problems, &lines);

    load_operands(&operand_grids, &mut problems);

    let answer: i64 = problems.iter().map(Problem::eval).sum();

    println!("Answer: {}", answer);
}

fn load_operands(operand_grids: &[CharGrid], problems: &mut [Problem]) {
    for (i, grid) in operand_grids.iter().enumerate() {
        for column in grid.columns_iter().collect::<Vec<Vec<char>>>().iter().rev() {
            let mut operand_str = String::new();
            for digit in column {
                if *digit == ' ' {
                    continue;
                }
                operand_str.push(*digit);
            }

            let operand: i64 = operand_str.parse().expect("invalid operand");
            problems[i].operands.push(operand);
        }
    }
}

fn parse_operand_grids(problems: &[Problem], lines: &[&str]) -> Vec<CharGrid> {
    let mut operand_grids: Vec<CharGrid> = vec![];
    let mut scan_cursor = 0;

    for _ in 0..problems.len() {
        let mut grid: CharGrid = CharGrid::default();
        let operand_lines = &lines[..lines.len() - 1];

        for (y, line) in operand_lines.iter().enumerate() {
            grid.resize(grid.width(), y + 1);

            for (global_x, ch) in line.chars().enumerate().skip(scan_cursor) {
                let x = global_x - scan_cursor;

                if ch == ' ' {
                    let mut has_cell_below = false;
                    let mut has_cell_above = false;

                    for dy in 1..operand_lines.len() - y {
                        if !is_cell_empty(operand_lines, global_x, y + dy) {
                            has_cell_below = true;
                        }
                    }

                    if y > 0 {
                        for dy in (-(y as i64)..=-1).rev() {
                            if !is_cell_empty(operand_lines, global_x, (y as i64 + dy) as usize) {
                                has_cell_above = true;
                            }
                        }
                    }

                    if !has_cell_below && !has_cell_above {
                        break;
                    }
                }

                if grid.contains(x as i64, y as i64) {
                    grid.set(x, y, ch);
                    continue;
                }

                grid.push_x(y, ch);
            }
        }

        scan_cursor += grid.width() + 1;
        operand_grids.push(grid);
    }

    operand_grids
}

fn is_cell_empty(operand_lines: &[&str], x: usize, y: usize) -> bool {
    operand_lines[y].chars().nth(x).expect("should exist") == ' '
}

fn init_problems(lines: &[&str]) -> Vec<Problem> {
    lines
        .last()
        .expect("invalid input")
        .split_whitespace()
        .map(|operator| match operator {
            "*" => Operator::Multiply,
            "+" => Operator::Add,
            _ => panic!("unknown operator: {}", operator),
        })
        .map(Problem::new)
        .collect()
}
