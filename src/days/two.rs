use std::{
    collections::HashSet,
    hash::{BuildHasher, Hasher},
    sync::atomic::{AtomicUsize, Ordering},
};

// 21139440284
pub fn part_one(input: &str) -> usize {
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

#[allow(unused)]
pub fn part_one_hash(input: &str) -> usize {
    struct FastHashBuilder;
    struct FastHash(u32);
    impl BuildHasher for FastHashBuilder {
        type Hasher = FastHash;
        fn build_hasher(&self) -> Self::Hasher {
            FastHash(0x9e3779b9)
        }
    }
    impl Hasher for FastHash {
        fn finish(&self) -> u64 {
            self.0 as u64
        }
        fn write(&mut self, _: &[u8]) {
            panic!()
        }
        fn write_u32(&mut self, x: u32) {
            // https://stackoverflow.com/questions/664014/what-integer-hash-function-are-good-that-accepts-an-integer-hash-key#12996028
            let x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3b);
            let x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3b);
            let x = (x >> 16) ^ x;
            self.0 = x;
        }
    }

    fn insert_pattern(set: &mut HashSet<u32, FastHashBuilder>, digits: usize, pattern_len: usize) {
        for num in 10usize.pow(pattern_len as u32 - 1)..10usize.pow(pattern_len as u32) {
            let mut number = 0;
            for i in 0..digits / pattern_len {
                number += num * 10usize.pow((pattern_len * i) as u32);
            }
            set.insert(number as u32);
        }
    }

    let mut set = HashSet::with_capacity_and_hasher(99_999 * 3, FastHashBuilder);
    for digits in (2..=10).step_by(2) {
        insert_pattern(&mut set, digits, digits / 2);
    }

    let sum = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for range in input.split(",") {
            s.spawn(|| {
                let (start, end) = range.split_once("-").unwrap();
                let start = start.trim();
                let end = end.trim();
                let start_digit = start.parse::<usize>().unwrap();
                let end_digit = end.parse::<usize>().unwrap();

                // if start.len() == end.len() && !start.len().is_multiple_of(2) {
                //     return;
                // }

                for num in start_digit..=end_digit {
                    if set.contains(&(num as u32)) {
                        sum.fetch_add(num, Ordering::Relaxed);
                    }
                }
            });
        }
    });
    sum.load(Ordering::Relaxed)
}

// 38731915928
pub fn part_two(input: &str) -> usize {
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
