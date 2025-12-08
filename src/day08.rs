use crate::utils;
use crate::utils::ErrorMsg;
use std::cmp::{max, min};
use std::str::FromStr;

static SAMPLE: &str = "inputs/day08_sample.txt";
static ACTUAL: &str = "inputs/day08.txt";

struct Pos {
    x: i32,
    y: i32,
    z: i32
}

impl Pos {
    fn sqr_distance_to(&self, other: &Self) -> u64 {
        let dx = (other.x - self.x).abs() as u64;
        let dy = (other.y - self.y).abs() as u64;
        let dz = (other.z - self.z).abs() as u64;
        dx * dx + dy * dy + dz * dz
    }
}

impl FromStr for Pos {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s.trim().split(',').collect::<Vec<&str>>();
        if vals.len() == 3 {
            Ok(Pos {x: vals[0].parse()?, y: vals[1].parse()?, z: vals[2].parse()?})
        } else {
            Err(ErrorMsg{wrapped: format!("Contains no 2 commas: {}", s)})
        }
    }
}

struct Connection {
    from: u16,
    to: u16
}

impl Connection {
    fn sqr_distance(&self, coords: &Vec<Pos>) -> u64 {
        coords[self.from as usize]
            .sqr_distance_to(&coords[self.to as usize])
    }
}

fn read_coords(actual: bool) -> Result<Vec<Pos>, ErrorMsg> {
    let file = utils::read_file(if actual {ACTUAL} else {SAMPLE})?;
    file.trim()
        .split('\n')
        .map(|l| l.parse::<Pos>())
        .collect::<Result<Vec<Pos>, ErrorMsg>>()
}

fn all_connections_sorted(coords: &Vec<Pos>) -> Vec<Connection> {
    let mut connections = (0..coords.len() as u16)
        .flat_map(|i| ((i+1)..coords.len() as u16)
            .map(move |j| Connection {from: i, to: j}))
        .collect::<Vec<Connection>>();
    connections.sort_by_key(|c| c.sqr_distance(&coords));
    connections
}

fn combine_circuits(connection: &Connection, circuits: &mut Vec<i16>, circuit_size: &mut Vec<u16>) {
    let ca = circuits[connection.from as usize];
    let cb = circuits[connection.to as usize];
    if ca != -1 && ca == cb {
        return;
    }

    match (ca, cb) {
        (-1, -1) => {
            let new_circuit = circuit_size.len() as i16;
            circuits[connection.from as usize] = new_circuit;
            circuits[connection.to as usize] = new_circuit;
            circuit_size.push(2);
        },
        (a, -1) => {
            circuits[connection.to as usize] = a;
            circuit_size[a as usize] += 1;
        },
        (-1, b) => {
            circuits[connection.from as usize] = b;
            circuit_size[b as usize] += 1;
        },
        (a, b) => {
            let keep = min(a, b);
            let drop = max(a, b);
            circuit_size[keep as usize] += circuit_size[drop as usize];
            for e in circuits {
                if *e == drop {
                    *e = keep;
                } else if *e > drop {
                    *e -= 1;
                }
            }
            circuit_size.remove(drop as usize);
        }
    }
}

pub fn run_part_1(actual: bool, count: usize) -> Result<String, ErrorMsg> {
    let coords = read_coords(actual)?;
    let connections = all_connections_sorted(&coords);

    let mut circuits = vec![-1i16; coords.len()];
    let mut circuit_size: Vec<u16> = vec![];

    for connection in connections.iter().take(count) {
        combine_circuits(connection, &mut circuits, &mut circuit_size);
    }

    circuit_size.sort_by_key(|&i| -(i as i32));
    Ok(circuit_size.iter().take(3).map(|&i| i as u32).product::<u32>().to_string())
}

pub fn run_part_2(actual: bool) -> Result<String, ErrorMsg> {
    let coords = read_coords(actual)?;
    let connections = all_connections_sorted(&coords);

    let mut circuits = vec![-1i16; coords.len()];
    let mut circuit_size: Vec<u16> = vec![];

    for connection in connections.iter() {
        combine_circuits(connection, &mut circuits, &mut circuit_size);

        if circuit_size.len() == 1 && circuit_size[0] == coords.len() as u16 {
            return Ok((coords[connection.from as usize].x as u32 * coords[connection.to as usize].x as u32).to_string())
        }
    }

    Err(ErrorMsg{wrapped: "End reached without total connection".to_string() })
}
