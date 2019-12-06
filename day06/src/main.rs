use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use failure::{bail, Error};

fn read_input() -> Result<Vec<(String, String)>, Error> {
    let file = File::open("day06/input.txt")?;
    let reader = BufReader::new(file);
    let mut v = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let x = line.find(")").unwrap();
        let (c, p) = line.split_at(x);
        v.push((c.to_string(), p[1..].to_string()))
    }
    Ok(v)
}

fn counter(center: &str, depth: i64, system: &HashMap<String, HashSet<String>>) -> i64 {
    let mut count = 0;
    if let Some(orbiters) = system.get(center) {
        for body in orbiters.iter() {
            count += depth + counter(body, depth + 1, system);
        }
    }
    count
}

fn path_to_body(
    center: &str,
    target: &str,
    system: &HashMap<String, HashSet<String>>,
) -> Option<Vec<String>> {
    if let Some(orbiters) = system.get(center) {
        if orbiters.contains(target) {
            let v = vec![target.to_string(), center.to_string()];
            return Some(v);
        }
        for body in orbiters.iter() {
            if let Some(mut v) = path_to_body(body, target, system) {
                v.push(center.to_string());
                return Some(v);
            }
        }
    }
    None
}
fn main() -> Result<(), Error> {
    let input = read_input()?;

    let mut orbiters: HashMap<String, HashSet<String>> = HashMap::new();
    for (center, body) in input.iter() {
        let list = orbiters.entry(center.clone()).or_default();
        list.insert(body.clone());
    }

    println!("Part1 {}", counter("COM", 1, &orbiters));

    let mut to_santa = path_to_body("COM", "SAN", &orbiters).unwrap();
    let mut to_you = path_to_body("COM", "YOU", &orbiters).unwrap();

    to_santa.reverse();
    to_you.reverse();

    let mut idx = 0 as usize;
    loop {
        //println!("{} {}", to_santa[idx], to_you[idx]);
        if to_santa[idx] != to_you[idx] {
            break;
        }
        idx += 1;
    }

    println!(
        "Part2 {}",
        (to_santa.len() - idx) + (to_you.len() - idx) - 2
    );

    Ok(())
}

#[test]
fn test() -> Result<(), Error> {
    Ok(())
}
