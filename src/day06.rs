use std::num::ParseIntError;
use std::str::FromStr;
use crate::day06::Operator::{Add, Mul};
use crate::utils;
use crate::utils::ErrorMsg;

static SAMPLE: &str = "inputs/day06_sample.txt";
static ACTUAL: &str = "inputs/day06.txt";

enum Operator {
    Mul, Add
}

impl FromStr for Operator {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "*" => Ok(Mul),
            "+" => Ok(Add),
            _ => Err(ErrorMsg{wrapped: format!("Invalid operator: {}", s)})
        }
    }
}

pub fn run_part_1(actual: bool) -> Result<String, ErrorMsg> {
    let file = utils::read_file(if actual {ACTUAL} else {SAMPLE})?;
    let lines = file.trim()
        .split('\n')
        .map(|l| l.split(" ")
            .filter(|e| !e.is_empty())
            .collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let (operators_line, argument_lines) = lines
        .split_last().ok_or_else(|| ErrorMsg{wrapped: "Input empty".to_string()})?;
    let arguments = argument_lines.iter()
        .map(|line| line.iter()
            .map(|e| e.parse::<u64>())
            .collect::<Result<Vec<u64>, ParseIntError>>())
        .collect::<Result<Vec<Vec<u64>>, ParseIntError>>()?;
    let operators = operators_line.iter()
        .map(|e| e.parse::<Operator>())
        .collect::<Result<Vec<Operator>, ErrorMsg>>()?;

    let mut sum = 0;
    for i in 0..operators.len() {
        sum += match operators[i] {
            Mul => arguments.iter().map(|a| a[i]).product::<u64>(),
            Add => arguments.iter().map(|a| a[i]).sum()
        };
    }

    Ok(sum.to_string())
}

pub fn run_part_2(actual: bool) -> Result<String, ErrorMsg> {
    let file = utils::read_file(if actual {ACTUAL} else {SAMPLE})?;
    let lines = file
        .split('\n')
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (operators_line, argument_lines) = lines
        .split_last().ok_or_else(|| ErrorMsg{wrapped: "Input empty".to_string()})?;

    let mut sum = 0;
    let mut args = vec![];
    for i in (0..operators_line.len()).rev() {
        let num = argument_lines.iter()
            .flat_map(|l| l[i].to_digit(10))
            .fold(0u64, |l, r| l * 10 + r as u64);
        if num != 0 {
            args.push(num);
        }
        if let Ok(op) = operators_line[i].to_string().parse::<Operator>(){
            println!("{:?}", args);
            sum += match op {
                Mul => args.iter().product::<u64>(),
                Add => args.iter().sum()
            };
            args.clear()
        }
    }

    Ok(sum.to_string())
}