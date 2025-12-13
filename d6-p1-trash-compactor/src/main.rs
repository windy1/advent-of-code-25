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
    let mut problems: Vec<Problem> = init_problems(&lines);

    load_operands(&lines, &mut problems);

    let answer: i64 = problems.iter().map(Problem::eval).sum();

    println!("Answer: {}", answer);
}

fn load_operands(lines: &[&str], problems: &mut [Problem]) {
    for line in &lines[..lines.len() - 1] {
        for (problem_index, operand_str) in line.split_whitespace().enumerate() {
            let problem = &mut problems[problem_index];
            let operand = operand_str.parse::<i64>().expect("invalid operand");
            problem.operands.push(operand);
        }
    }
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
