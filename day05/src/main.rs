use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

use failure::{bail, Error};

fn read_input() -> Result<Vec<i64>, Error> {
    let s = fs::read_to_string("day05/input.txt")?;
    let v = s
        .split(',')
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    Ok(v)
}

fn read_parameter(pc: usize, idx: usize, mem: &[i64]) -> i64 {
    let opcode = mem[pc];
    let mode = (opcode / (10i64.pow((idx + 1) as u32))) % 10;
    let v = mem[pc + idx];
    if mode == 0 {
        return mem[v as usize];
    } else {
        v
    }
}

fn write_parameter(pc: usize, idx: usize, mem: &mut [i64], value: i64) {
    let v = mem[pc + idx];
    mem[v as usize] = value;
}

fn run(input: &[i64], param: i64) -> Result<i64, Error> {
    let mut mem = Vec::from(input);

    let mut pc = 0;
    loop {
        let opcode = mem[pc];
        let r = match opcode % 100 {
            1 => {
                let a = read_parameter(pc, 1, &mem);
                let b = read_parameter(pc, 2, &mem);
                write_parameter(pc, 3, &mut mem, a + b);
                Some(4)
            }
            2 => {
                let a = read_parameter(pc, 1, &mem);
                let b = read_parameter(pc, 2, &mem);
                write_parameter(pc, 3, &mut mem, a * b);
                Some(4)
            }
            3 => {
                write_parameter(pc, 1, &mut mem, param);
                Some(2)
            }
            4 => {
                let b = read_parameter(pc, 1, &mem);
                println!("Output: {}", b);
                Some(2)
            }
            5 => {
                let c = read_parameter(pc, 1, &mem);
                let d = read_parameter(pc, 2, &mem);
                if c != 0 {
                    pc = d as usize;
                    Some(0)
                } else {
                    Some(3)
                }
            }
            6 => {
                let c = read_parameter(pc, 1, &mem);
                let d = read_parameter(pc, 2, &mem);
                if c == 0 {
                    pc = d as usize;
                    Some(0)
                } else {
                    Some(3)
                }
            }
            7 => {
                let a = read_parameter(pc, 1, &mem);
                let b = read_parameter(pc, 2, &mem);
                if a < b {
                    write_parameter(pc, 3, &mut mem, 1);
                } else {
                    write_parameter(pc, 3, &mut mem, 0);
                }
                Some(4)
            }
            8 => {
                let a = read_parameter(pc, 1, &mem);
                let b = read_parameter(pc, 2, &mem);
                if a == b {
                    write_parameter(pc, 3, &mut mem, 1);
                } else {
                    write_parameter(pc, 3, &mut mem, 0);
                }
                Some(4)
            }

            99 => None,
            _ => {
                bail!("Error");
            }
        };
        if let Some(x) = r {
            pc += x;
            pc = pc % mem.len();
        } else {
            break;
        }
    }
    Ok(mem[0])
}
fn main() -> Result<(), Error> {
    let input = read_input()?;
    run(&input, 1)?;
    run(&input, 5)?;
    Ok(())
}

#[test]
fn test() -> Result<(), Error> {
    let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    run(&input, 8)?;
    let input = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    run(&input, 8)?;
    let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    run(&input, 8)?;
    let input = vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    run(&input, 9)?;
    Ok(())
}
