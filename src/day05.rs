use std::cmp::{min,max};
use std::num::ParseIntError;
use std::str::FromStr;
use crate::utils;
use crate::utils::ErrorMsg;

static SAMPLE: &str = "inputs/day05_sample.txt";
static ACTUAL: &str = "inputs/day05.txt";

struct Range {
    first: u64,
    last: u64
}

impl Range {
    fn contains(&self, id: u64) -> bool {
        id >= self.first && id <= self.last
    }

    fn overlap_with(&self, other: &Range) -> Option<Range> {
        if self.first <= other.first && self.last >= other.first {
            Some(Range{first: other.first, last: min(self.last, other.last)})
        } else if other.first <= self.first && other.last >= self.first {
            Some(Range{first: self.first, last: min(self.last, other.last)})
        } else {
            None
        }
    }
}

impl FromStr for Range {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_str, last_str) = s.trim().split_once('-')
            .ok_or_else(|| ErrorMsg{wrapped: format!("No dash: {}", s)})?;
        let first = first_str.parse()?;
        let last = last_str.parse()?;
        Ok(Range{first, last})
    }}

pub fn run_part_1(actual: bool) -> Result<String, ErrorMsg> {
    let input = utils::read_file(if actual {ACTUAL} else {SAMPLE})?;
    let (ranges_str, ids_str) = input
        .trim()
        .split_once("\n\n")
        .ok_or_else(|| ErrorMsg{wrapped: format!("No empty line: {}", input.trim())})?;
    let ranges = ranges_str.trim()
        .split('\n')
        .map(|s| s.trim().parse::<Range>())
        .collect::<Result<Vec<Range>, ErrorMsg>>()?;
    let ids = ids_str.trim()
        .split('\n')
        .map(|s| s.trim().parse::<u64>())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;

    let fresh = ids.iter()
        .filter(|&&id| ranges.iter().any(|r| r.contains(id)))
        .count();
    Ok(fresh.to_string())
}

pub fn run_part_2(actual: bool) -> Result<String, ErrorMsg> {
    let input = utils::read_file(if actual {ACTUAL} else {SAMPLE})?;
    let (ranges_str, ids_str) = input
        .trim()
        .split_once("\n\n")
        .ok_or_else(|| ErrorMsg{wrapped: format!("No empty line: {}", input.trim())})?;

    let mut ranges = ranges_str.trim()
        .split('\n')
        .map(|s| s.trim().parse::<Range>())
        .collect::<Result<Vec<Range>, ErrorMsg>>()?;

    ranges.sort_by(|a, b| a.first.partial_cmp(&b.first).unwrap());

    let mut i = 0;
    while i < ranges.len() - 1 {
        if ranges[i].last < ranges[i + 1].first {
            i += 1;
        } else {
            ranges[i].last = max(ranges[i].last, ranges[i + 1].last);
            ranges.remove(i + 1);
        }
    }

    let fresh: u64 = ranges.iter()
        .map(|r| r.last - r.first + 1)
        .sum();
    Ok(fresh.to_string())
}