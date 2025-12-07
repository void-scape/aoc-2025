// FIRST PASS

use std::collections::HashMap;

enum Item {
    Arg(usize),
    Op(String),
}

pub fn part_one(input: &str) -> usize {
    let mut columns = HashMap::<usize, Vec<Item>>::new();
    for line in input.lines() {
        for (i, num) in line.split_whitespace().enumerate() {
            columns
                .entry(i)
                .or_default()
                .push(match num.parse::<usize>() {
                    Ok(num) => Item::Arg(num),
                    Err(_) => Item::Op(num.to_string()),
                });
        }
    }

    let mut total = 0;
    for items in columns.into_values() {
        let op = items.last().unwrap();
        match op {
            Item::Op(op) => match op.as_str() {
                "*" => {
                    let mut sum = 1;
                    for val in items[..items.len() - 1].iter() {
                        match val {
                            Item::Arg(arg) => sum *= *arg,
                            Item::Op(_) => unreachable!(),
                        }
                    }
                    total += sum;
                }
                "+" => {
                    let mut sum = 0;
                    for val in items[..items.len() - 1].iter() {
                        match val {
                            Item::Arg(arg) => sum += *arg,
                            Item::Op(_) => unreachable!(),
                        }
                    }
                    total += sum;
                }
                _ => panic!("{op}"),
            },
            _ => unreachable!(),
        }
    }
    total
}

#[derive(Debug)]
enum Item2 {
    Arg { offset: usize, string: String },
    Op(String),
}

pub fn part_two(input: &str) -> usize {
    let first_op = input
        .as_bytes()
        .iter()
        .position(|c| *c == b'*' || *c == b'+')
        .unwrap();
    let last_col = &input[first_op..];

    let mut columns = HashMap::<usize, Vec<Item2>>::new();

    let mut col_lens = Vec::new();
    let mut is_first = true;
    let mut col_len = 0;
    for char in last_col.chars() {
        if char.is_whitespace() {
            col_len += 1;
        } else {
            if is_first {
                columns
                    .entry(0)
                    .or_default()
                    .push(Item2::Op(String::from(char)));
                col_len += 1;
                is_first = false;
                continue;
            } else {
                columns
                    .entry(columns.len())
                    .or_default()
                    .push(Item2::Op(String::from(char)));
                col_lens.push(col_len);
                col_len = 0;
            }
            col_len += 1;
        }
    }
    col_lens.push(col_len);

    for line in input.lines() {
        if line.contains("*") || line.contains("+") {
            break;
        }

        let mut line_offset = 0;
        for (column, col_len) in col_lens.iter().enumerate() {
            let this_col = &line.as_bytes()[line_offset..];
            let offset = this_col
                .iter()
                .take_while(|c| c.is_ascii_whitespace())
                .count();

            let mut string = String::new();

            for i in 0..*col_len {
                if i + line_offset >= line.len() {
                    break;
                }

                let byte = line.as_bytes()[i + line_offset];
                if byte.is_ascii_digit() {
                    string.push(byte as char);
                }
            }
            line_offset += col_len;

            columns
                .entry(column)
                .or_default()
                .push(Item2::Arg { offset, string });
        }
    }

    let mut total = 0;
    for items in columns.into_values() {
        let mut largest_digit_count = 0;
        for item in items.iter() {
            if let Item2::Arg { offset, string, .. } = item
                && largest_digit_count < string.len() + offset
            {
                largest_digit_count = string.len() + offset;
            }
        }

        let mut accs = vec![0; largest_digit_count];
        for digit in (0..largest_digit_count).rev() {
            for item in items.iter() {
                if let Item2::Arg { offset, string, .. } = item {
                    if string.len() - 1 + offset < digit {
                        continue;
                    }

                    if *offset > digit {
                        continue;
                    }

                    accs[digit] *= 10;
                    accs[digit] += (string.as_bytes()[digit - offset] - b'0') as usize;
                }
            }
        }

        let op = items.first().unwrap();
        match op {
            Item2::Op(op) => match op.as_str() {
                "*" => {
                    let mut sum = 1;
                    for arg in accs.iter() {
                        sum *= *arg;
                    }
                    total += sum;
                }
                "+" => {
                    total += accs.iter().sum::<usize>();
                }
                _ => panic!("{op}"),
            },
            _ => unreachable!(),
        }
    }
    total
}

// SECOND PASS

pub fn part_one_bench(input: &str) -> usize {
    let input = input.as_bytes();

    let mut first_op_index = 0;
    let mut operators = [false; 1100];

    // NOTE: 70% of the runtime is spent here, parsing the numbers.
    let mut values = Vec::with_capacity(4100);
    let mut accum = 0;
    for (i, byte) in input.iter().enumerate() {
        if byte.is_ascii_whitespace() {
            if accum != 0 {
                values.push(accum);
            }
            accum = 0;
        } else if *byte == b'*' || *byte == b'+' {
            first_op_index = i;
            break;
        } else {
            accum *= 10;
            accum += (byte - b'0') as usize;
        }
    }

    let mut op_index = 0;
    for byte in input[first_op_index..].iter() {
        if *byte == b'*' {
            operators[op_index] = true;
            op_index += 1;
        } else if *byte == b'+' {
            op_index += 1;
        }
    }

    let mut total = 0;

    let stride = op_index;
    let rows = values.len() / stride;

    for column in 0..op_index {
        if operators[column] {
            let mut sum = 1;
            for offset in 0..rows {
                let value = values[offset * stride + column];
                sum *= value;
            }
            total += sum;
        } else {
            for offset in 0..rows {
                total += values[offset * stride + column];
            }
        }
    }

    total
}

pub fn part_two_bench(input: &str) -> usize {
    // I want to work on my voxel renderer :(
    part_two(input)
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("../../inputs/6.txt");
    #[test]
    fn part_one() {
        crate::test::verify_results(
            INPUT,
            &[super::part_one, super::part_one_bench],
            5552221122013,
        );
    }
    #[test]
    fn part_two() {
        crate::test::verify_results(
            INPUT,
            &[super::part_two, super::part_two_bench],
            11371597126232,
        );
    }
}
