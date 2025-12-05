use std::slice;

pub fn part_one(input: &str) -> i32 {
    let input = input.as_bytes();
    let mut times_zero = 0;
    let mut dial = 50;

    unsafe {
        let mut start = input.as_ptr();
        let str_end = input.as_ptr().byte_add(input.len());
        let mut end = libc::memchr(input.as_ptr().cast(), '\n' as _, input.len());

        while !end.is_null() {
            let len = end.byte_offset_from(start);
            let line = slice::from_raw_parts(start, len as usize);
            if line.is_empty() {
                break;
            }

            let mut amount = 0;
            for c in &line[1..] {
                amount *= 10;
                amount += (c - b'0') as i32;
            }

            let sign = if line[0] == b'L' { -1 } else { 1 };
            dial = (dial - amount * sign).rem_euclid(100);
            times_zero += (dial == 0) as i32;

            start = end.byte_add(1).cast();
            end = libc::memchr(
                start.cast(),
                '\n' as _,
                str_end.byte_offset_from(start) as _,
            );
        }
        times_zero
    }
}

pub fn part_two(input: &str) -> i32 {
    let input = input.as_bytes();
    let mut times_zero = 0;
    let mut dial = 50;

    unsafe {
        let mut start = input.as_ptr();
        let str_end = input.as_ptr().byte_add(input.len());
        let mut end = libc::memchr(input.as_ptr().cast(), '\n' as _, input.len());

        while !end.is_null() {
            let len = end.byte_offset_from(start);
            let line = slice::from_raw_parts(start, len as usize);
            if line.is_empty() {
                break;
            }

            let mut amount = 0;
            for c in &line[1..] {
                amount *= 10;
                amount += (c - b'0') as i32;
            }

            let sign = if line[0] == b'L' { -1 } else { 1 };
            let dial_long = dial + amount * sign;
            let mut revolutions = (dial_long / 100).abs();
            if dial != 0 && dial_long <= 0 {
                revolutions += 1;
            }

            dial = dial_long.rem_euclid(100);
            times_zero += revolutions;

            start = end.byte_add(1).cast();
            end = libc::memchr(
                start.cast(),
                '\n' as _,
                str_end.byte_offset_from(start) as _,
            );
        }
        times_zero
    }
}
