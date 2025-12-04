use std::str::FromStr;
use crate::utils;
use crate::utils::ErrorMsg;

static SAMPLE: &str = "inputs/day03_sample.txt";
static ACTUAL: &str = "inputs/day03.txt";

struct Bank {
    batteries: Vec<u8>
}

impl FromStr for Bank {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s.trim().chars()
            .map(|c| c.to_digit(10)
                .map(|d| d as u8)
                .ok_or_else(|| ErrorMsg{wrapped: format!("Invalid battery character: {}", c)}))
            .collect::<Result<Vec<u8>, ErrorMsg>>()?;
        Ok(Bank{batteries})
    }
}

pub fn run_part_1(actual: bool) -> Result<String, ErrorMsg> {
    run(actual, 2)
}

pub fn run_part_2(actual: bool) -> Result<String, ErrorMsg> {
    run(actual, 12)
}

fn run(actual: bool, count: u8) -> Result<String, ErrorMsg> {
    let banks = utils::read_file(if actual {ACTUAL} else {SAMPLE})?
        .trim()
        .split('\n')
        .map(|s| s.parse::<Bank>())
        .collect::<Result<Vec<Bank>, ErrorMsg>>()?;
    let mut sum = 0;
    for bank in banks {
        let mut last: Option<u8> = None;
        let mut accum: u64 = 0;
        for i in 0..count {
            let mut start = last.map(|l| l + 1).unwrap_or(0);
            for j in start..=(bank.batteries.len() as u8 - count + i) {
                if bank.batteries[j as usize] > bank.batteries[start as usize] {
                    start = j;
                }
            }
            accum = accum * 10 + bank.batteries[start as usize] as u64;
            last = Some(start);
        }
        sum += accum;
    }
    Ok(sum.to_string())
}