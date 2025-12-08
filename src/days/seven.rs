// FIRST PASS

use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    let beam_start = input.chars().position(|c| c == 'S').unwrap();
    let mut beams = HashSet::new();
    beams.insert(beam_start);

    let mut splits = 0;
    for line in input.lines() {
        for split_index in line
            .chars()
            .enumerate()
            .flat_map(|(i, c)| (c == '^').then_some(i))
        {
            if beams.remove(&split_index) {
                beams.insert(split_index - 1);
                beams.insert(split_index + 1);
                splits += 1;
            }
        }
    }

    splits
}

pub fn part_two(input: &str) -> usize {
    let beam_start = input.chars().position(|c| c == 'S').unwrap();
    let mut beams = HashMap::new();
    beams.insert(beam_start, 1);

    for line in input.lines() {
        for split_index in line
            .chars()
            .enumerate()
            .flat_map(|(i, c)| (c == '^').then_some(i))
        {
            if let Some(beam_count) = beams.remove(&split_index) {
                *beams.entry(split_index - 1).or_default() += beam_count;
                *beams.entry(split_index + 1).or_default() += beam_count;
            }
        }
    }

    beams.into_values().sum()
}

// SECOND PASS

pub fn part_one_bench(input: &str) -> usize {
    let input = input.as_bytes();

    let beam_start = input.iter().position(|c| *c == b'S').unwrap();
    // NOTE: bools are ~2us faster than usize.
    let mut beams = [false; 141];
    beams[beam_start] = true;

    let mut splits = 0;
    let mut start_line = 0;
    for (i, byte) in input[beam_start + 1..].iter().enumerate() {
        match byte {
            b'\n' => {
                start_line = i + 1;
            }
            b'^' => {
                let i = i - start_line;
                if beams[i] {
                    beams[i] = false;
                    beams[i - 1] = true;
                    beams[i + 1] = true;
                    splits += 1;
                }
            }
            _ => {}
        }
    }
    splits
}

pub fn part_two_bench(input: &str) -> usize {
    let input = input.as_bytes();

    let beam_start = input.iter().position(|c| *c == b'S').unwrap();
    let mut beams = [0; 141];
    beams[beam_start] = 1;

    let mut start_line = 0;
    for (i, byte) in input[beam_start + 1..].iter().enumerate() {
        match byte {
            b'\n' => {
                start_line = i + 1;
            }
            b'^' => {
                let i = i - start_line;
                let beam_count = beams[i];
                if beam_count != 0 {
                    beams[i] = 0;
                    beams[i - 1] += beam_count;
                    beams[i + 1] += beam_count;
                }
            }
            _ => {}
        }
    }
    beams.iter().sum()
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("../../inputs/7.txt");
    #[test]
    fn part_one() {
        crate::test::verify_results(INPUT, &[super::part_one, super::part_one_bench], 1656);
    }
    #[test]
    fn part_two() {
        crate::test::verify_results(
            INPUT,
            &[super::part_two, super::part_two_bench],
            76624086587804,
        );
    }
}
