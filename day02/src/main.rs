use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

use failure::{bail, Error};

fn read_input() -> Result<Vec<i64>, Error> {
    let s = fs::read_to_string("day02/input.txt")?;
    let v = s
        .split(',')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect::<Vec<_>>();
    Ok(v)
}

fn run(input: &[i64], noun: i64, verb: i64) -> Result<i64, Error> {
    let mut mem = Vec::from(input);
    mem[1] = noun;
    mem[2] = verb;

    let mut pc = 0;
    loop {
        let r = match &mem[pc..pc + 4] {
            &[1, a, b, dst] => {
                let va = mem[a as usize];
                let vb = mem[b as usize];
                mem[dst as usize] = va + vb;
                Some(4)
            }
            &[2, a, b, dst] => {
                let va = mem[a as usize];
                let vb = mem[b as usize];
                mem[dst as usize] = va * vb;
                Some(4)
            }
            &[99, _, _, _] => None,
            _ => {
                bail!("Error");
            }
        };
        if let Some(x) = r {
            pc += x;
        } else {
            break;
        }
    }
    Ok(mem[0])
}

fn main() -> Result<(), Error> {
    let input = read_input()?;

    let part1 = run(&input, 12, 2)?;
    println!("Part 1: {}", part1);

    for noun in 0..100 {
        for verb in 0..100 {
            let r = run(&input, noun, verb)?;
            if r == 19690720 {
                println!("Part 2: {}", (100 * noun) + verb);
            }
        }
    }

    Ok(())
}
