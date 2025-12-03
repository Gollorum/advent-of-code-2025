use std::str::FromStr;
use crate::utils;
use crate::utils::ErrorMsg;

static SAMPLE: &str = "inputs/day01_sample.txt";
static ACTUAL: &str = "inputs/day01.txt";

struct Rotation {
    diff: i32,
}

impl FromStr for Rotation {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, val_str) = s.trim().split_at(1);
        let val = val_str.parse::<i32>()?;
        let diff = match dir {
            "L" => -val,
            "R" => val,
            _ => return Err(ErrorMsg{wrapped: format!("Invalid rotation direction: {}", dir)}),
        };
        Ok(Rotation{diff})
    }
}

fn run_part_1(actual: bool) -> Result<String, ErrorMsg> {
    let mut password = 0;
    let mut position = 50;

    for line in utils::read_file(if actual {ACTUAL} else {SAMPLE})?.split('\n') {
        if line.len() > 0 {
            let rot = line.parse::<Rotation>()?;
            position = (position + rot.diff) % 100;
            if position == 0 {
                password += 1;
            }
        }
    }
    Ok(password.to_string())
}

pub(crate) fn run_part_2(actual: bool) -> Result<String, ErrorMsg> {
    let mut password = 0;
    let mut position = 50;

    for line in utils::read_file(if actual {ACTUAL} else {SAMPLE})?.split('\n') {
        if line.len() > 0 {
            let rot = line.parse::<Rotation>()?;
            let prev_position = position;
            position += rot.diff;
            match position {
                0 => password += 1,
                _ if position < 0 => {
                    if prev_position > 0 {
                        password += 1;
                    }
                    while position < 0 {
                        position += 100;
                        if position <= 0 {
                            password += 1;
                        }
                    }
                },
                _ if position >= 100 => {
                    while position >= 100 {
                        password += 1;
                        position -= 100;
                    }
                }
                _ => (),
            }
        }
    }
    Ok(password.to_string())
}