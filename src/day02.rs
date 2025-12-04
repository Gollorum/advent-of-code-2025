use std::iter::Filter;
use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::utils::ErrorMsg;

static SAMPLE: &str = "inputs/day02_sample.txt";
static ACTUAL: &str = "inputs/day02.txt";

struct Range {
    first: u64,
    last: u64
}

impl FromStr for Range {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_str, last_str) = s.trim().split_once('-')
            .ok_or_else(|| ErrorMsg{wrapped: format!("Invalid range format: {}", s)})?;
        let first = first_str.parse()?;
        let last = last_str.parse()?;
        Ok(Range{first, last})
    }
}

fn is_valid_id_v1(id: &u64) -> bool {
    let mut digits = id.ilog10();
    if 10u64.pow(digits) != *id {
        digits += 1;
    }
    if (digits % 2) != 0 {
        return true;
    }
    let slice_size = digits / 2;
    let mask = 10u64.pow(slice_size);
    id % mask != id / mask
}

fn is_valid_id_v2(id: &u64) -> bool {
    let digits = id.ilog10() + 1;
    for slice_size in 1..=(digits / 2) {
        if digits % slice_size != 0 {
            continue;
        }
        let mask = 10u64.pow(slice_size);
        let head = id % mask;
        let mut tail = id / mask;
        let mut valid = false;
        for _ in 1..(digits / slice_size) {
            if tail % mask != head {
                valid = true;
                break;
            }
            tail /= mask
        }
        if !valid {
            return false;
        }
    }
    true
}

impl Range {
    fn invalids_v1(&self) -> Filter<RangeInclusive<u64>, fn(&u64) -> bool> {
        (self.first..=self.last)
            .filter(|id| !is_valid_id_v1(id))
    }
    fn invalids_v2(&self) -> Filter<RangeInclusive<u64>, fn(&u64) -> bool> {
        (self.first..=self.last)
            .filter(|id| !is_valid_id_v2(id))
    }
}

fn collect_ranges(actual: bool) -> Result<Vec<Range>, ErrorMsg> {
    let ranges = crate::utils::read_file(if actual {ACTUAL} else {SAMPLE})?
        .split(',')
        .map(str::parse::<Range>)
        .collect::<Result<Vec<Range>, ErrorMsg>>()?;
    Ok(ranges)
}

pub fn run_part_1(actual: bool) -> Result<String, ErrorMsg> {
    let ranges = collect_ranges(actual)?;
    let num_invalids: u64 = ranges.iter().flat_map(|r| r.invalids_v1()).sum();
    Ok(num_invalids.to_string())
}

pub fn run_part_2(actual: bool) -> Result<String, ErrorMsg> {
    let ranges = collect_ranges(actual)?;
    let num_invalids: u64 = ranges.iter().flat_map(|r| r.invalids_v2()).sum();
    Ok(num_invalids.to_string())
}