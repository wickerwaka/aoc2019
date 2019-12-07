use permutohedron::Heap;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

use failure::{bail, Error};

fn read_input() -> Result<Vec<i64>, Error> {
    let s = fs::read_to_string("day07/input.txt")?;
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

struct IntState {
    output: i64,
    state: Vec<i64>,
    params: Vec<i64>,
    pc: usize,
}

fn run(state: &IntState) -> Result<Option<IntState>, Error> {
    let mut mem = Vec::from(state.state.clone());

    let mut param_iter = state.params.iter();

    let mut pc = state.pc;
    let mut output = None;

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
                write_parameter(pc, 1, &mut mem, *param_iter.next().unwrap());
                Some(2)
            }
            4 => {
                let b = read_parameter(pc, 1, &mem);
                //println!("Output: {}", b);
                output = Some(b);
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
        if output.is_some() {
            return Ok(Some(IntState {
                output: output.unwrap(),
                state: mem.clone(),
                pc: pc,
                params: param_iter.cloned().collect(),
            }));
        }
    }
    Ok(None)
}
fn main() -> Result<(), Error> {
    let input = read_input()?;

    {
        let mut values = vec![0, 1, 2, 3, 4];

        let heap = Heap::new(&mut values);

        let mut max_thrust = 0;
        for data in heap {
            let mut thrust = 0;
            for phase in data.iter() {
                let state = IntState {
                    pc: 0,
                    state: input.clone(),
                    params: vec![*phase, thrust],
                    output: 0,
                };
                thrust = run(&state)?.unwrap().output;
            }
            //println!("Thrust {}", thrust);
            max_thrust = thrust.max(max_thrust);
        }
        println!("max_thrust {}", max_thrust);
    }

    {
        let mut values = vec![5, 6, 7, 8, 9];

        let heap = Heap::new(&mut values);

        let mut max_thrust = 0;
        for data in heap {
            let mut states = Vec::new();
            for x in 0..5 {
                states.push(IntState {
                    output: 0,
                    pc: 0,
                    state: input.clone(),
                    params: vec![data[x]],
                })
            }
            let mut thrust = 0;
            let mut complete = false;
            while complete == false {
                for idx in 0..5 {
                    let in_state = states.get_mut(idx).unwrap();
                    in_state.params.push(thrust);
                    if let Some(out_state) = run(in_state)? {
                        thrust = out_state.output;
                        *in_state = out_state;
                    } else {
                        complete = true;
                    }
                }
            }
            //println!("Thrust {}", thrust);
            max_thrust = thrust.max(max_thrust);
        }
        println!("max_thrust {}", max_thrust);
    }

    Ok(())
}
