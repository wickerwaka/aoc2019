use failure::Error;

fn main() -> Result<(), Error> {
    const MIN: i64 = 134564;
    const MAX: i64 = 585159;

    let mut part1_count = 0;
    let mut part2_count = 0;
    let mut digits = vec![0; 8];
    digits[0] = 10;
    digits[7] = -1;
    for x in MIN..MAX {
        digits[0] = 10;
        let mut val = x;
        for d in 0..6 {
            let digit = val % 10;
            val = val / 10;
            digits[d + 1] = digit;
        }

        let ascending = digits.as_slice().windows(2).all(|x| x[0] >= x[1]);
        let exclusive_pair = digits
            .as_slice()
            .windows(4)
            .any(|x| (x[0] != x[1]) && (x[1] == x[2]) && (x[2] != x[3]));
        let pair = digits.as_slice().windows(2).any(|x| (x[0] == x[1]));
        if ascending && pair {
            part1_count += 1;
        }
        if ascending && exclusive_pair {
            part2_count += 1;
        }
    }

    println!("{}", part1_count);
    println!("{}", part2_count);

    Ok(())
}
