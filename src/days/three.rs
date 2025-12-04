use std::{collections::HashSet, fmt::Display};

// FIRST PASS

pub fn part_one(input: &str) -> impl Display {
    let mut sum = 0;
    for line in input.lines() {
        let mut pairs = HashSet::new();
        for i in 0..line.len() {
            for j in i + 1..line.len() {
                pairs.insert(
                    format!("{}{}", &line[i..i + 1], &line[j..j + 1],)
                        .parse::<usize>()
                        .unwrap(),
                );
            }
        }
        let mut pairs = pairs.iter().collect::<Vec<_>>();
        pairs.sort();
        sum += *pairs.last().unwrap();
    }
    sum
}

pub fn part_two(input: &str) -> impl Display {
    let mut sum = 0;
    for line in input.lines() {
        let mut buffer = String::with_capacity(12);
        let mut biggest_num_index = 0;
        for i in (0..=11).rev() {
            let mut nums = line[..line.len() - i]
                .chars()
                .enumerate()
                .skip(biggest_num_index)
                .map(|(i, c)| (i, (c as u8 - b'0') as usize))
                .collect::<Vec<_>>();
            nums.sort_by(|a, b| b.0.cmp(&a.0));
            let nums_index = nums.iter().max_by_key(|(_, v)| *v).unwrap().0;
            buffer.push_str(&line[nums_index..nums_index + 1]);
            biggest_num_index = nums_index + 1;
        }

        sum += buffer.parse::<usize>().unwrap();
    }
    sum
}

// SECOND PASS

pub fn part_one_bench(input: &str) -> impl Display {
    joltage::<2>(input)
}

pub fn part_two_bench(input: &str) -> impl Display {
    joltage::<12>(input)
}

// NOTE: Threading joltage by splitting the input in half is roughly twice as slow
// for part one.
fn joltage<const BATTERIES: usize>(input: &str) -> impl Display {
    let mut sum = 0;
    // buffer that stores char data for the final joltage
    let mut buffer = [0; BATTERIES];

    for line in input.as_bytes().split(|c| *c == b'\n') {
        if line.is_empty() {
            continue;
        }

        // This loop searches for the index of the first max value.
        //
        // The search is constrained to a range of valid locations for the
        // current index in the joltage char buffer.
        let mut biggest_num_index = 0;
        for i in (0..BATTERIES).rev() {
            let mut nums_index = 0;
            let mut max = 0;
            for (index, num) in line[biggest_num_index..line.len() - i]
                .iter()
                .enumerate()
                .map(|(i, c)| (i, (c - b'0') as usize))
            {
                if num > max {
                    nums_index = index;
                    max = num;
                }
            }
            nums_index += biggest_num_index;

            buffer[BATTERIES - 1 - i] = line[nums_index];
            biggest_num_index = nums_index + 1;
        }

        let mut accum = 0;
        for digit in buffer.iter() {
            accum *= 10;
            accum += (*digit - b'0') as usize;
        }

        sum += accum;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let input = include_str!("../../inputs/3.txt");
        let p1 = super::part_one(input).to_string().parse::<usize>().unwrap();
        let p1_bench = part_one_bench(input).to_string().parse::<usize>().unwrap();
        assert_eq!(p1, p1_bench);
        assert_eq!(p1_bench, 17535);
    }

    #[test]
    fn part_two() {
        let input = include_str!("../../inputs/3.txt");
        let p2 = super::part_two(input).to_string().parse::<usize>().unwrap();
        let p2_bench = part_two_bench(input).to_string().parse::<usize>().unwrap();
        assert_eq!(p2, p2_bench);
        assert_eq!(p2_bench, 173577199527257);
    }
}
