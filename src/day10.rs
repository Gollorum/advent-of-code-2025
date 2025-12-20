use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::num::ParseIntError;
use std::ops;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use substring::Substring;
use crate::utils;
use crate::utils::ErrorMsg;
use memoize::memoize;
use itertools::Itertools;

static SAMPLE: &str = "inputs/day10_sample.txt";
static ACTUAL: &str = "inputs/day10.txt";

#[derive(Clone)]
struct Machine {
    light_diagram: Vec<bool>,
    light_diagram_mask: u16,
    button_wiring: Vec<Joltage>,
    button_wiring_masks: Vec<u16>,
    joltage: Joltage
}

lazy_static! {
    static ref MACHINE_REGEX: Regex = Regex::new(r"\[(.*)\] (.*) \{(.*)\}").unwrap();
}
impl FromStr for Machine {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = MACHINE_REGEX.captures(s).ok_or(ErrorMsg{wrapped:format!("Failed to capture machine regex in {s}")})?;
        let lights = captures[1].chars().map(|s| match s {
            '#' => Ok(true),
            '.' => Ok(false),
            _ => Err(ErrorMsg{wrapped: format!("Unrecognized light {}", s)})
        }).collect::<Result<Vec<bool>, ErrorMsg>>()?;
        let mut lights_mask = 0;
        for (i, activate) in lights.iter().enumerate() {
            if *activate {
                lights_mask |= 1 << i
            }
        }
        let buttons = captures[2].split(' ').map(|s|
            s.substring(1, s.len() - 1).split(',').map(|s2| s2.parse::<u16>()).collect::<Result<Vec<u16>, ParseIntError>>())
            .collect::<Result<Vec<Vec<u16>>, ParseIntError>>()?;
        let button_masks = buttons.iter().map(|b| {
            let mut mask = 0;
            for &i in b {
                mask |= 1 << (i as u16)
            }
            mask
        }).collect::<Vec<u16>>();
        let joltage = captures[3].split(',').map(|s| s.parse::<u16>())
            .collect::<Result<Vec<u16>, ParseIntError>>()?;
        let mut button_wiring = buttons.iter().map(|b| {
            let mut values = vec![0; lights.len()];
            for bb in b {
                values[*bb as usize] = 1;
            }
            Joltage {values}
        }).collect::<Vec<Joltage>>();
        // button_wiring.sort_by_key(|b| b.values.len());
        Ok(Machine {
            light_diagram: lights,
            light_diagram_mask: lights_mask,
            button_wiring,
            button_wiring_masks: button_masks,
            joltage: Joltage{values:joltage}
        })
    }
}

fn read_input(actual: bool) -> Result<Vec<Machine>, ErrorMsg> {
    let file = utils::read_file(if actual {ACTUAL} else {SAMPLE})?;
    file.trim()
        .split('\n')
        .map(|l| l.parse::<Machine>())
        .collect::<Result<Vec<Machine>, ErrorMsg>>()
}

fn subsets(total_len: usize, subset_size: usize) -> Vec<usize> {
    if subset_size == 0 {
        return vec![0];
    }
    if total_len == 0 {
        return vec![];
    } else if subset_size == 0 {
        return vec![0];
    }
    let mut ret = subsets(total_len - 1, subset_size);
    for i  in subsets(total_len - 1, subset_size - 1) {
        ret.push(i | (1 << (total_len - 1)))
    }
    ret
}

fn can_satisfy_lights_in(machine: &Machine, presses: usize) -> bool {
    for subset in subsets(machine.button_wiring.len(), presses) {
        let mut lights = 0;
        for (i, b) in machine.button_wiring_masks.iter().enumerate() {
            if (1 << i) & subset != 0 {
                lights ^= b;
            }
        }
        if lights == machine.light_diagram_mask {
            return true;
        }
    }
    false
}

// fn min_button_presses_only(machine: &Machine, current_joltage: &Vec<u16>, i: usize) -> Option<u32> {
//     let first_diff = machine.joltage.values.iter().zip(current_joltage).enumerate()
//         .find(|(i, (a, b))| **a != **b)?;
//     if !machine.button_wiring[i].iter().any(|&b| b == (first_diff.0 as u16)) {
//         return None
//     }
//     let diff = first_diff.1.0 - first_diff.1.1;
//     if machine.joltage.iter().zip(current_joltage).enumerate().all(|(i2, (&a, &b))| if machine.button_wiring[i].iter().any(|&b| b == i2 as u16) { a - b == diff} else {a == b}) {
//         Some(diff as u32)
//     } else {
//         None
//     }
// }

// fn min_button_presses(machine: &Machine, remaining_joltage: &Joltage, buttons_left: u8, mut cache: &mut HashMap<(Joltage, u8), Option<u32>>) -> Option<u32> {
//     // if buttons_left == 1 {
//     //     let mut differences = vec![];
//     //     let buttons_0 = &machine.button_wiring[0];
//     //     let buttons_1 = &machine.button_wiring[1];
//     //     let foo = (buttons_0).contains(23);
//     //     for b in 0..machine.light_diagram.len() {
//     //         if machine.button_wiring[1].iter().any(|bb| *bb == b as u16) ^ buttons_0.iter().any(|bb| *bb == b as u16) {
//     //             differences.push(b);
//     //         }
//     //     }
//     //
//     // }
//     let t = (remaining_joltage.clone(), buttons_left);
//     if let Some(res) = cache.get(&t) {
//         return *res;
//     }
//     let mut new_joltage = t.0;
//     let mut ret = None;
//     let mut own_presses = 0;
//     loop {
//         if buttons_left > 0 {
//             let other_ret = min_button_presses(machine, &new_joltage, buttons_left - 1, &mut cache);
//             if let Some(val) = ret {
//                 if let Some(val2) = other_ret {
//                     ret = Some(min(val, val2 + own_presses))
//                 }
//             } else {
//                 ret = other_ret.map(|val2| val2 + own_presses);
//             }
//         }
//
//         if None == new_joltage.sub(&machine.button_wiring[buttons_left as usize]) {
//             break
//         }
//         own_presses += 1;
//
//         if new_joltage.is_zero() {
//             ret = Some(match ret {
//                 None => own_presses,
//                 Some(val) => min(val, own_presses)
//             });
//             break
//         }
//     }
//     cache.insert((remaining_joltage.clone(), buttons_left), ret);
//     ret
// }

#[memoize]
fn distributions(max_rem: u8, to_distribute: u8, receivers: usize) -> Vec<Vec<u8>> {
    if receivers == 1 {
        if max_rem < to_distribute {
            vec![]
        } else {
            vec![vec![to_distribute]]
        }
    } else {
        // let mut me_only = vec![0; receivers - 1];
        // me_only.push(min(max_rem, to_distribute));
        // let mut ret = vec![me_only];
        let mut ret = vec![];
        for i in 0..min(max_rem, to_distribute) {
            for mut other in distributions(max_rem, to_distribute - i, receivers - 1) {
                other.push(i);
                ret.push(other);
            }
        }
        ret
    }
}

fn min_button_presses(button_bundles: &Vec<Vec<&Joltage>>, remaining_joltage: Joltage, bundle_index: u8, mut cache: &mut HashMap<(Joltage, u8), Option<u32>>) -> Option<u32> {
    let t = (remaining_joltage.clone(), bundle_index);
    if let Some(res) = cache.get(&t) {
        return *res;
    }
    let bundle = &button_bundles[bundle_index as usize];
    let max_rem = *remaining_joltage.values.iter().max().unwrap() as u32;
    let max_presses = max_rem * bundle.len() as u32;
    let min_presses = if bundle_index == 0 { max_rem } else { 0 };
    let mut ret = None;
    'outer: for i in (min_presses..=max_presses).rev() {
        'inner: for dist in distributions(max_rem as u8, i as u8, bundle.len()) {
            let mut new_joltage = remaining_joltage.clone();
            for j in 0..bundle.len() {
                for _ in 0..dist[j] {
                    if new_joltage.sub(&bundle[j]) == None {
                        continue 'inner;
                    }
                }
            }
            if new_joltage.is_zero() {
                ret = Some(i as u32);
                break 'outer;
            }
            if bundle_index > 0 {
                let local_res = min_button_presses(&button_bundles, new_joltage, bundle_index - 1, &mut cache);
                if let Some(mut lr) = local_res {
                    lr += i;
                    if ret == None || ret.unwrap() > lr {
                        ret = Some(lr);
                    }
                }
            }
        }
        if ret != None {
            break;
        }
    }
    cache.insert((remaining_joltage.clone(), bundle_index), ret);
    ret
}

pub fn run_part_1(actual: bool) -> Result<String, ErrorMsg> {
    let machines = read_input(actual)?;
    let mut result = 0;
    for machine in machines {
        for i in 1..machine.button_wiring_masks.len() {
            if can_satisfy_lights_in(&machine, i) {
                result += i;
                break;
            }
        }
    }
    Ok(result.to_string())
}

#[derive(Eq, PartialEq, Clone, Hash)]
#[derive(Debug)]
struct Joltage {
    values: Vec<u16>
}

#[derive(Eq, PartialEq)]
#[derive(Debug)]
struct JoltageEntry {
    joltage_remaining: Joltage,
    presses_so_far: u32
}

impl Ord for JoltageEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.presses_so_far).cmp(&(self.presses_so_far))
    }
}

impl PartialOrd for JoltageEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl JoltageEntry {
    fn from(joltage_remaining: Joltage, presses_so_far: u32) -> JoltageEntry{
        JoltageEntry{joltage_remaining, presses_so_far}
    }
}

impl ops::Sub for &Joltage {
    type Output = Option<Joltage>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut values = vec![];
        for i in 0..self.values.len() {
            values.push(self.values[i].checked_sub(rhs.values[i])?)
        }
        Some(Joltage{values})
    }
}

impl ops::Add for &Joltage {
    type Output = Joltage;
    fn add(self, rhs: Self) -> Self::Output {
        let mut values = vec![];
        for i in 0..self.values.len() {
            values.push(self.values[i] + rhs.values[i])
        }
        Joltage{values}
    }
}

impl Joltage {
    fn is_zero(&self) -> bool {
        for v in &self.values {
            if *v != 0 {
                return false
            }
        }
        true
    }
    fn add(&mut self, rhs: &Self) {
        for i in 0..self.values.len() {
            self.values[i] += rhs.values[i]
        }
    }
    fn sub(&mut self, rhs: &Self) -> Option<()> {
        for i in 0..self.values.len() {
            self.values[i] = self.values[i].checked_sub(rhs.values[i])?
        }
        Some(())
    }
}

pub fn run_part_2(actual: bool) -> Result<String, ErrorMsg> {
    let machines = read_input(actual)?;
    let mut result = 0u32;
    for machine in machines {
        // let mut seen = HashSet::new();
        // let mut heap = BinaryHeap::new();
        // heap.push(JoltageEntry::from(machine.joltage, 0));
        // while let Some(entry) = heap.pop() {
        //     if seen.contains(&entry.joltage_remaining) {
        //         continue;
        //     }
        //     if entry.joltage_remaining.is_zero() {
        //         result += entry.presses_so_far;
        //         println!("{}", entry.presses_so_far);
        //         seen.insert(entry.joltage_remaining);
        //         break;
        //     }
        //     let new_presses = entry.presses_so_far + 1;
        //     for b in &machine.button_wiring {
        //         if let Some(new_joltage) = &entry.joltage_remaining - b {
        //             if !seen.contains(&new_joltage) {
        //                 let new_entry = JoltageEntry::from(new_joltage, new_presses);
        //                 heap.push(new_entry)
        //             }
        //         }
        //     }
        //     seen.insert(entry.joltage_remaining);
        // }
        // if !seen.contains(&Joltage{values: vec![0; machine.light_diagram.len()]}) {
        //     println!("AAAAA")
        // }
        let buttons = machine.button_wiring.iter()
            .sorted_by_key(|bw| bw.values.iter().sum::<u16>())
            .chunk_by(|bw| bw.values.iter().sum::<u16>())
            .into_iter()
            .map(|g| g.1.collect::<Vec<&Joltage>>())
            .collect::<Vec<Vec<&Joltage>>>();
        let min_o = min_button_presses(&buttons, machine.joltage, buttons.len() as u8 - 1, &mut HashMap::new())
            .ok_or_else(|| ErrorMsg{wrapped: format!("Failed to reach joltage")})?;
        println!("{}", min_o);
        result += min_o;
    }
    Ok(result.to_string())
}