use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use failure::{Error};

fn read_input() -> Result<Vec<i64>,Error> {
    let file = File::open("day01/input.txt")?;
    let reader = BufReader::new(file);
    let mut v = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let n = line.parse::<i64>()?;
        v.push(n);
    }

    Ok(v)
}
fn main() -> Result<(), Error> {
    let input = read_input()?;
    let mut sum = 0;
    for i in input.iter() {
        sum += ( i / 3 ) - 2;
    }
    println!( "{}", sum );

    let mut sum_fuel = 0;
    for i in input.iter() {
        let mut mass = *i;
        loop {
            let fuel = ( mass / 3 ) - 2;
            if fuel <= 0 {
                break;
            }
            sum_fuel += fuel;
            mass = fuel;
        }
    }
    println!( "{}", sum_fuel);
    Ok(())
}
