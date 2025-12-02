use std::fmt::Display;

pub fn part_one(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut times_zero = 0;
    let mut dial = 50;
    for line in input.split(|c| *c == b'\n') {
        if line.is_empty() {
            continue;
        }
        let mut amount = 0;
        for c in &line[1..] {
            amount *= 10;
            amount += (c - b'0') as i32;
        }
        if line[0] == b'L' {
            dial -= amount;
            if dial < 0 {
                while dial < 0 {
                    dial += 100;
                }
            }
        } else {
            dial += amount;
            if dial > 99 {
                while dial > 99 {
                    dial -= 100;
                }
            }
        }
        if dial == 0 {
            times_zero += 1;
        }
    }
    times_zero
}

pub fn part_two(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut times_zero = 0;
    let mut dial = 50;
    for line in input.split(|c| *c == b'\n') {
        if line.is_empty() {
            continue;
        }
        let mut amount = 0;
        for c in &line[1..] {
            amount *= 10;
            amount += (c - b'0') as i32;
        }
        if line[0] == b'L' {
            for _ in 0..amount {
                dial -= 1;
                if dial == 0 {
                    times_zero += 1;
                }
                if dial < 0 {
                    dial = 99;
                }
            }
        } else {
            dial += amount;
            if dial > 99 {
                while dial > 99 {
                    dial -= 100;
                    times_zero += 1;
                }
            }
        }
    }
    times_zero
}
