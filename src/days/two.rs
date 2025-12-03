use std::{
    fmt::Display,
    sync::atomic::{AtomicUsize, Ordering},
};

// 21139440284
pub fn part_one(input: &str) -> impl Display {
    fn sum_range(range: &str) -> usize {
        let (start, end) = range.split_once("-").unwrap();
        let start_digit = start.trim().parse::<usize>().unwrap();
        let end_digit = end.trim().parse::<usize>().unwrap();
        sum_usize_range(start_digit, end_digit)
    }

    fn sum_usize_range(start_digit: usize, end_digit: usize) -> usize {
        let mut sum = 0;
        let mut num_string = String::with_capacity(12);
        for num in start_digit..=end_digit {
            num_string.clear();
            let mut accum = num;
            while accum > 0 {
                num_string.push(((accum % 10) as u8 + b'0') as char);
                accum /= 10;
            }
            let num_bytes = num_string.as_bytes();
            let digits = num_bytes.len();

            if !digits.is_multiple_of(2) {
                continue;
            }

            let half = digits / 2;
            if num_string[..half] == num_string[half..] {
                sum += num;
            }
        }
        sum
    }

    let sum = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for range in input.split(",") {
            s.spawn(|| {
                sum.fetch_add(sum_range(range), Ordering::Relaxed);
            });
        }
    });
    sum.load(Ordering::Relaxed)
}

// 38731915928
pub fn part_two(input: &str) -> impl Display {
    fn sum_range(range: &str) -> usize {
        let (start, end) = range.split_once("-").unwrap();
        let start_digit = start.trim().parse::<usize>().unwrap();
        let end_digit = end.trim().parse::<usize>().unwrap();
        sum_usize_range(start_digit, end_digit)
    }

    fn sum_usize_range(start_digit: usize, end_digit: usize) -> usize {
        let mut sum = 0;
        let mut num_string = String::with_capacity(12);
        'outer: for num in start_digit..=end_digit {
            num_string.clear();
            let mut accum = num;
            while accum > 0 {
                num_string.push(((accum % 10) as u8 + b'0') as char);
                accum /= 10;
            }
            let num_bytes = num_string.as_bytes();
            let digits = num_bytes.len();

            'inner: for pattern_len in 1..digits / 2 + 1 {
                if !digits.is_multiple_of(pattern_len) {
                    continue;
                }

                let mut last_pattern: &[u8] = &[];

                for (pattern_count, i) in (0..digits - pattern_len + 1)
                    .step_by(pattern_len)
                    .enumerate()
                {
                    let pattern = &num_bytes[i..i + pattern_len];

                    if last_pattern != pattern && !last_pattern.is_empty() {
                        continue 'inner;
                    }

                    if pattern.len() * (pattern_count + 1) == digits {
                        sum += num;
                        continue 'outer;
                    }

                    last_pattern = pattern;
                }
            }
        }
        sum
    }

    let sum = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for range in input.split(",") {
            s.spawn(|| {
                sum.fetch_add(sum_range(range), Ordering::Relaxed);
            });
        }
    });
    sum.load(Ordering::Relaxed)
}
