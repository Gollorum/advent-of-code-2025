use crate::utils;
use crate::utils::ErrorMsg;

pub fn run_sample() -> Result<String, ErrorMsg> {
    run("inputs/day01_sample.txt")
}

pub fn run_actual() -> Result<String, ErrorMsg> {
    run("inputs/day01.txt")
}

fn run(path: &str) -> Result<String, ErrorMsg> {
    let lines = utils::read_file(path)?;
    Ok(lines)
}