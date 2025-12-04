use std::str::FromStr;
use crate::utils;
use crate::utils::ErrorMsg;

static SAMPLE: &str = "inputs/day04_sample.txt";
static ACTUAL: &str = "inputs/day04.txt";

struct Grid {
    cells: Vec<Vec<bool>>
}

impl Grid {
    fn get_at(&self, rowI: isize, colI: isize) -> bool {
        if rowI < 0 || colI < 0 {
            return false;
        }
        if let Some(row) = self.cells.get(rowI as usize) {
            if let Some(val) = row.get(colI as usize) {
                return *val;
            }
        }
        false
    }

    fn deactivate_at(&mut self, rowI: usize, colI: usize) {
        if let Some(row) = self.cells.get_mut(rowI) {
            if let Some(val) = row.get_mut(colI) {
                *val = false;
            }
        }
    }

    fn adjacent_rolls(&self, rowI: usize, colI: usize) -> u8 {
        let mut count = 0;
        for r in -1..=1 {
            for c in -1..=1 {
                if r == 0 && c == 0 {
                    continue;
                }
                if self.get_at(rowI as isize + r, colI as isize + c) {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn run_part_1(actual: bool) -> Result<String, ErrorMsg> {
    let grid = parse_input(actual)?;
    let mut accessible_rolls = 0;
    for (rowI, row) in grid.cells.iter().enumerate() {
        for colI in 0..row.len() {
            if grid.get_at(rowI as isize, colI as isize) {
                if grid.adjacent_rolls(rowI, colI) < 4 {
                    accessible_rolls += 1;
                }
            }
        }
    }
    Ok(accessible_rolls.to_string())
}

pub fn run_part_2(actual: bool) -> Result<String, ErrorMsg> {
    let mut grid = parse_input(actual)?;
    let width = grid.cells[0].len();
    let height = grid.cells.len();
    let mut accessible_rolls = 0;
    let mut any_deactivated = true;
    while any_deactivated {
        any_deactivated = false;
        for rowI in 0..height {
            for colI in 0..width {
                if grid.get_at(rowI as isize, colI as isize) {
                    if grid.adjacent_rolls(rowI, colI) < 4 {
                        accessible_rolls += 1;
                        grid.deactivate_at(rowI, colI);
                        any_deactivated = true;
                    }
                }
            }
        }
    }
    Ok(accessible_rolls.to_string())
}

impl FromStr for Grid {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s.trim()
            .split('\n')
            .map(|line| line.trim().chars().map(|c| c == '@').collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>();
        Ok(Grid{cells})
    }
}

fn parse_input(actual: bool) -> Result<Grid, ErrorMsg> {
    utils::read_file(if actual {ACTUAL} else {SAMPLE})?
        .parse()
}