use std::cmp::{max, min};
use std::str::FromStr;
use crate::utils;
use crate::utils::ErrorMsg;

static SAMPLE: &str = "inputs/day09_sample.txt";
static ACTUAL: &str = "inputs/day09.txt";

struct Pos {
    x: u32,
    y: u32
}

struct Rect {
    min: Pos,
    max: Pos
}

#[derive(PartialEq)]
enum Dir {
    Up, Right, Down, Left
}

struct Line {
    dir: Dir,
    fixed: u32,
    from: u32,
    to: u32
}

impl Pos {

    fn rect_with(&self, other: &Self) -> Rect {
        Rect {
            min: Pos {x: min(self.x, other.x), y: min(self.y, other.y)},
            max: Pos {x: max(self.x, other.x), y: max(self.y, other.y)}
        }
    }

    fn line_to(&self, other: &Self) -> Line {
        if self.x == other.x {
            if self.y < other.y {
                Line { dir: Dir::Down, from: self.y, to: other.y, fixed: self.x}
            } else {
                Line { dir: Dir::Up, from: other.y, to: self.y, fixed: self.x}
            }
        } else {
            if self.x < other.x {
                Line { dir: Dir::Right, from: self.x, to: other.x, fixed: self.y}
            } else {
                Line { dir: Dir::Left, from: other.x, to: self.x, fixed: self.y}
            }
        }
    }
}

impl Rect {
    fn intersects(&self, line: &Line) -> bool {
        if line.dir == Dir::Up || line.dir == Dir::Down {
            if self.min.x > line.fixed || self.max.x < line.fixed || self.min.y >= line.to || self.max.y <= line.from {
                false
            } else if self.min.x == line.fixed {
                line.dir == Dir::Down
            } else if self.max.x == line.fixed {
                line.dir == Dir::Up
            } else {
                true
            }
        } else {
            if self.min.y > line.fixed || self.max.y < line.fixed || self.min.x >= line.to || self.max.x <= line.from {
                false
            } else if self.min.y == line.fixed {
                line.dir == Dir::Left
            } else if self.max.y == line.fixed {
                line.dir == Dir::Right
            } else {
                true
            }
        }
    }

    fn size(&self) -> u64 {
        let dx = (self.max.x - self.min.x + 1) as u64;
        let dy = (self.max.y - self.min.y + 1) as u64;
        dx * dy
    }
}

impl FromStr for Pos {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s.trim().split(',').collect::<Vec<&str>>();
        if vals.len() == 2 {
            Ok(Pos {x: vals[0].parse()?, y: vals[1].parse()?})
        } else {
            Err(ErrorMsg{wrapped: format!("Contains no single comma: {}", s)})
        }
    }
}

fn read_positions(actual: bool) -> Result<Vec<Pos>, ErrorMsg> {
    let file = utils::read_file(if actual {ACTUAL} else {SAMPLE})?;
    file.trim()
        .split('\n')
        .map(|l| l.parse::<Pos>())
        .collect::<Result<Vec<Pos>, ErrorMsg>>()
}

pub fn run_part_1(actual: bool) -> Result<String, ErrorMsg> {
    let positions = read_positions(actual)?;
    let mut max_rect = 0;
    for i in 0..(positions.len() - 1) {
        for j in (i + 1)..positions.len() {
            let rect = positions[i].rect_with(&positions[j]);
            let s = rect.size();
            if s > max_rect {
                max_rect = s;
            }
        }
    }
    Ok(format!("{:?}", max_rect))
}

pub fn run_part_2(actual: bool) -> Result<String, ErrorMsg> {
    let positions = read_positions(actual)?;
    let lines = (0..positions.len()).map(|i|
        positions[i].line_to(&positions[(i+1) % positions.len()])
    ).collect::<Vec<Line>>();
    let mut max_rect = 0;
    for i in 0..(positions.len() - 1) {
        for j in (i + 1)..positions.len() {
            let rect = positions[i].rect_with(&positions[j]);
            if lines.iter().any(|l| rect.intersects(l)) {
                continue;
            }
            let s = rect.size();
            if s > max_rect {
                max_rect = s;
            }
        }
    }
    Ok(format!("{:?}", max_rect))
}