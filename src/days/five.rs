// FIRST PASS

use crate::parse_usize;
use std::{
    collections::HashSet,
    ops::RangeInclusive,
    simd::{cmp::SimdPartialEq, u8x32},
};

pub fn part_one(input: &str) -> usize {
    let mut sum = 0;
    let mut ranges = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains("-") {
            let mut iter = line.split("-");
            let start = iter.next().unwrap().parse::<usize>().unwrap();
            let end = iter.next().unwrap().parse::<usize>().unwrap();
            ranges.push(start..=end);
        } else {
            let id = line.parse::<usize>().unwrap();
            if ranges.iter().any(|range| range.contains(&id)) {
                sum += 1;
            }
        }
    }
    sum
}

fn collapse_ranges(
    r1: &RangeInclusive<usize>,
    r2: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    let r1s = *r1.start();
    let r1e = *r1.end();

    let r2s = *r2.start();
    let r2e = *r2.end();

    let w1 = r1e - r1s + 1;
    let w2 = r2e - r2s + 1;
    let start = r1s.min(r2s);
    let end = r1e.max(r2e);

    if w1 + w2 > end - start {
        Some(start..=end)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> usize {
    let mut hashset = HashSet::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains("-") {
            let mut iter = line.split("-");
            let start = iter.next().unwrap().parse::<usize>().unwrap();
            let end = iter.next().unwrap().parse::<usize>().unwrap();
            hashset.insert(start..=end);
        }
    }

    let mut unique_ranges = Vec::from_iter(hashset);
    'outer: loop {
        for i in 0..unique_ranges.len() {
            for j in 0..unique_ranges.len() {
                if i != j
                    && let Some(new) = collapse_ranges(&unique_ranges[i], &unique_ranges[j])
                {
                    unique_ranges[i] = new;
                    unique_ranges.remove(j);
                    continue 'outer;
                }
            }
        }
        break;
    }

    unique_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

// SECOND PASS

// this shit is slow wtf
#[allow(unused)]
struct SimdSplitNL<'a>(&'a [u8]);
impl<'a> Iterator for SimdSplitNL<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        fn simd_newline(input: &[u8]) -> Option<usize> {
            const NL_MASK: u8x32 = u8x32::splat(b'\n');
            let nl_eq = NL_MASK.simd_eq(u8x32::load_or_default(input));
            nl_eq.first_set()
        }

        simd_newline(self.0).map(|index| {
            let out = &self.0[..index];
            self.0 = &self.0[index + 1..];
            out
        })
    }
}

pub fn part_one_bench(input: &str) -> usize {
    let input = input.as_bytes();
    let mut sum = 0;
    let mut ranges = Vec::with_capacity(200);
    let mut ids = Vec::with_capacity(200);

    // NOTE: SimdSplitNL is ~20us slower.
    for line in input.split(|c| *c == b'\n') {
        if line.is_empty() {
            continue;
        }

        // NOTE: For some reason, it is faster to always do this event after
        // the ranges are done parsing...
        //
        // NOTE: SIMD is ~20us slower.
        match line.split_once(|c| *c == b'-') {
            Some((start, end)) => {
                let start = parse_usize(start);
                let end = parse_usize(end);
                ranges.push(start..=end);
            }
            None => {
                ids.push(parse_usize(line));
            }
        }
    }

    // `into_iter` saves ~15us.
    for id in ids.into_iter() {
        // `range.contains` is faster than anything I could write.
        if ranges.iter().any(|range| range.contains(&id)) {
            sum += 1;
        }
    }

    sum
}

pub fn part_two_bench(input: &str) -> usize {
    let input = input.as_bytes();

    let mut ranges = Vec::with_capacity(200);
    let mut ranges_2 = Vec::<RangeInclusive<usize>>::with_capacity(200);
    for line in input.split(|c| *c == b'\n') {
        if line.is_empty() {
            continue;
        }

        match line.split_once(|c| *c == b'-') {
            Some((start, end)) => {
                let start = parse_usize(start);
                let end = parse_usize(end);
                // Why is RangeInclusive so much faster?
                ranges.push(start..=end);
            }
            None => {
                break;
            }
        }
    }

    loop {
        let mut collapsed = false;
        'outer: for range in ranges.drain(..) {
            for range_2 in ranges_2.iter_mut() {
                if let Some(new) = collapse_ranges(&range, range_2) {
                    collapsed = true;
                    *range_2 = new;
                    continue 'outer;
                }
            }
            ranges_2.push(range);
        }

        if !collapsed {
            break;
        }

        // NOTE: Double buffering DOUBLES performance here!
        std::mem::swap(&mut ranges, &mut ranges_2);
    }

    ranges_2
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("../../inputs/5.txt");
    #[test]
    fn part_one() {
        crate::test::verify_results(INPUT, &[super::part_one, super::part_one_bench], 868);
    }
    #[test]
    fn part_two() {
        crate::test::verify_results(
            INPUT,
            &[super::part_two, super::part_two_bench],
            354143734113772,
        );
    }
}
