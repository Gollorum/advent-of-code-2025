use std::collections::{HashMap, HashSet};
use crate::utils;
use crate::utils::ErrorMsg;

static SAMPLE: &str = "inputs/day07_sample.txt";
static ACTUAL: &str = "inputs/day07.txt";

pub fn run_part_1(actual: bool) -> Result<String, ErrorMsg> {
    let file = utils::read_file(if actual {ACTUAL} else {SAMPLE})?;
    let lines = file.trim()
        .split('\n')
        .collect::<Vec<&str>>();
    let start_loc = lines[0].find('S').ok_or_else(|| ErrorMsg{wrapped: "No start".to_string()})?;
    let mut beams = HashSet::new();
    beams.insert(start_loc);
    let mut split_count = 0;
    for l in lines.iter().skip(1) {
        let mut new_beams = HashSet::new();
        for &b in beams.iter() {
            match l.chars().nth(b).ok_or_else(|| ErrorMsg{wrapped: "Ran out of bounds".to_string()})? {
                '.' => {
                    new_beams.insert(b);
                },
                '^' => {
                    new_beams.insert(b + 1);
                    new_beams.insert(b - 1);
                    split_count += 1;
                }
                other => return Err(ErrorMsg{wrapped: format!("Unknown symbol {}", other)})
            }
        }
        beams = new_beams;
    }
    Ok(split_count.to_string())
}

pub fn run_part_2(actual: bool) -> Result<String, ErrorMsg> {
    let file = utils::read_file(if actual {ACTUAL} else {SAMPLE})?;
    let lines = file.trim()
        .split('\n')
        .collect::<Vec<&str>>();
    let start_loc = lines[0].find('S').ok_or_else(|| ErrorMsg{wrapped: "No start".to_string()})?;
    let mut beams = HashMap::new();
    beams.insert(start_loc, 1);
    for l in lines.iter().skip(1) {
        let mut new_beams = HashMap::new();
        for (&b, &timeline_count) in beams.iter() {
            match l.chars().nth(b).ok_or_else(|| ErrorMsg{wrapped: "Ran out of bounds".to_string()})? {
                '.' => {
                    *new_beams.entry(b).or_insert(0) += timeline_count;
                },
                '^' => {
                    *new_beams.entry(b + 1).or_insert(0) += timeline_count;
                    *new_beams.entry(b - 1).or_insert(0) += timeline_count;
                }
                other => return Err(ErrorMsg{wrapped: format!("Unknown symbol {}", other)})
            }
        }
        beams = new_beams;
    }
    Ok(beams.values().into_iter().sum::<i64>().to_string())
}