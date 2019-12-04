use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::collections::HashMap;

use failure::{bail, Error};

#[derive(Debug)]
enum Move {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Position {
    x: i64,
    y: i64,
}

fn read_input() -> Result<Vec<Vec<Move>>, Error> {
    let file = File::open("day03/input.txt")?;
    let reader = BufReader::new(file);
    let mut v = Vec::new();
    for line in reader.lines() {
        let mut moves = Vec::new();
        for m in line?.split(',') {
            let prefix = &m[0..1];
            let value = m[1..].parse::<i64>()?;
            let c = match prefix {
                "U" => Move::Up(value),
                "D" => Move::Down(value),
                "L" => Move::Left(value),
                "R" => Move::Right(value),
                x => bail!("Unrecognised prefix: {}", x),
            };
            moves.push(c);
        }
        v.push(moves);
    }
    Ok(v)
}

fn main() -> Result<(), Error> {
    let input = read_input()?;

    {
        let mut grid = HashMap::new();
        let mut min_dist = 999999999999;
        for (idx, wire) in input.iter().enumerate() {
            let mut position = Position { x: 0, y: 0 };
            for m in wire.iter() {
                let (len, xdelta, ydelta) = match m {
                    Move::Up(c) => (c, 0, -1),
                    Move::Down(c) => (c, 0, 1),
                    Move::Left(c) => (c, -1, 0),
                    Move::Right(c) => (c, 1, 0),
                };
                for c in 0..*len {
                    position.x += xdelta;
                    position.y += ydelta;
                    if let Some(old_idx) = grid.insert(position.clone(), idx) {
                        if old_idx != idx {
                            let dist = position.x.abs() + position.y.abs();
                            println!("Intersection: {:?} {}", position, dist);
                            if dist < min_dist {
                                min_dist = dist;
                            }
                        }
                    }
                }
            }
        }

        println!("{:?}", min_dist);
    }

    {
        let mut grid: HashMap<Position, (usize, usize)> = HashMap::new();
        let mut min_steps = 999999999999;
        for (idx, wire) in input.iter().enumerate() {
            let mut position = Position { x: 0, y: 0 };
            let mut steps = 0;
            for m in wire.iter() {
                let (len, xdelta, ydelta) = match m {
                    Move::Up(c) => (c, 0, -1),
                    Move::Down(c) => (c, 0, 1),
                    Move::Left(c) => (c, -1, 0),
                    Move::Right(c) => (c, 1, 0),
                };
                for c in 0..*len {
                    position.x += xdelta;
                    position.y += ydelta;
                    steps += 1;
                    if let Some((old_idx, old_steps)) = grid.get(&position) {
                        if *old_idx != idx {
                            let total_steps = old_steps + steps;
                            if total_steps < min_steps {
                                min_steps = total_steps;
                            }
                        }
                    } else {
                        grid.insert(position.clone(), (idx, steps));
                    }
                }
            }
        }

        println!("{:?}", min_steps);
    }

    Ok(())
}
