// FIRST PASS

fn neighbors(
    grid: &[bool],
    width: usize,
    height: usize,
    x: i32,
    y: i32,
) -> impl Iterator<Item = bool> {
    #[rustfmt::skip]
    let indices = [
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y),                 (x + 1, y),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
    ];
    indices.into_iter().flat_map(move |(x, y)| {
        if x >= 0 && y >= 0 && x < width as i32 && y < height as i32 {
            Some(grid[(y * width as i32 + x) as usize])
        } else {
            None
        }
    })
}

pub fn part_one(input: &str) -> usize {
    let width = input.lines().next().unwrap().trim().len();
    let height = input.lines().count();
    let grid = input
        .lines()
        .flat_map(|line| line.trim().chars().map(|c| c == '@'))
        .collect::<Vec<_>>();

    assert_eq!(grid.len(), width * height);

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x]
                && neighbors(&grid, width, height, x as i32, y as i32)
                    .filter(|is_paper| *is_paper)
                    .count()
                    < 4
            {
                sum += 1;
            }
        }
    }
    sum
}

pub fn part_two(input: &str) -> usize {
    let width = input.lines().next().unwrap().trim().len();
    let height = input.lines().count();
    let mut grid = input
        .lines()
        .flat_map(|line| line.trim().chars().map(|c| c == '@'))
        .collect::<Vec<_>>();

    assert_eq!(grid.len(), width * height);

    let mut sum = 0;
    loop {
        let mut this_loop_sum = 0;
        for y in 0..height {
            for x in 0..width {
                if grid[y * width + x]
                    && neighbors(&grid, width, height, x as i32, y as i32)
                        .filter(|is_paper| *is_paper)
                        .count()
                        < 4
                {
                    sum += 1;
                    this_loop_sum += 1;
                    grid[y * width + x] = false;
                }
            }
        }

        if this_loop_sum == 0 {
            break;
        }
    }
    sum
}

#[allow(unused)]
pub fn part_two_visualized(input: &str) -> usize {
    use std::time::Duration;

    let width = input.lines().next().unwrap().trim().len();
    let height = input.lines().count();
    let mut grid = input
        .lines()
        .flat_map(|line| line.trim().chars().map(|c| c == '@'))
        .collect::<Vec<_>>();

    assert_eq!(grid.len(), width * height);

    let pause_dur = Duration::from_secs_f32(0.5);
    loop {
        let mut this_grid = grid.clone();

        print!("{}[2J", 27 as char);
        for row in grid.chunks(width) {
            let ascii_row = row
                .iter()
                .map(|is_paper| if *is_paper { '@' } else { '.' })
                .collect::<String>();
            println!("{ascii_row}");
        }
        std::thread::sleep(pause_dur);

        let mut this_loop_sum = 0;
        for y in 0..height {
            for x in 0..width {
                if grid[y * width + x]
                    && neighbors(&grid, width, height, x as i32, y as i32)
                        .filter(|is_paper| *is_paper)
                        .count()
                        < 4
                {
                    this_loop_sum += 1;
                    this_grid[y * width + x] = false;
                }
            }
        }

        print!("{}[2J", 27 as char);
        for (row, this_row) in grid.chunks(width).zip(this_grid.chunks(width)) {
            let ascii_row = row
                .iter()
                .zip(this_row.iter())
                .map(|(is_paper, was_removed)| {
                    if *is_paper && !*was_removed {
                        "\x1b[41mx\x1b[0m"
                    } else if *is_paper {
                        "@"
                    } else {
                        "."
                    }
                })
                .collect::<String>();
            println!("{ascii_row}");
        }
        std::thread::sleep(pause_dur);

        grid = this_grid;

        if this_loop_sum == 0 {
            grid = input
                .lines()
                .flat_map(|line| line.trim().chars().map(|c| c == '@'))
                .collect::<Vec<_>>();
        }
    }
}

// SECOND PASS

fn increment_neighbors(neighbors: &mut [usize], size: usize, x: i32, y: i32) {
    #[rustfmt::skip]
    let indices = [
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y),                 (x + 1, y),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
    ];
    for (x, y) in indices.into_iter() {
        if x >= 0 && y >= 0 && x < size as i32 && y < size as i32 {
            neighbors[(y * size as i32 + x) as usize] += 1;
        }
    }
}

pub fn part_one_bench(input: &str) -> usize {
    let input = input.as_bytes();
    let size = input.iter().position(|c| *c == b'\n').unwrap();

    let mut neighbors = vec![0; size * size];
    let mut paper_indices = Vec::with_capacity(size * size);

    for (y, row) in (0..size)
        // Unrolling the split by b'\n' saves ~4us
        .map(|y| {
            let start = y * size + y;
            &input[start..start + size]
        })
        .enumerate()
    {
        let y_index = y * size;
        for (x, b) in row.iter().enumerate() {
            if *b == b'@' {
                // This check reduces paper_indices by ~2k, save ~10us
                if neighbors[y_index + x] < 4 {
                    paper_indices.push(y_index + x);
                }
                increment_neighbors(&mut neighbors, size, x as i32, y as i32);
            }
        }
    }

    // Deffering this check improves performance greatly, ~200us -> ~174us
    //
    // SIMD neighbors is slow, roughly ~200us.
    paper_indices
        .iter()
        .map(|index| (neighbors[*index] < 4) as usize)
        .sum()
}

fn decrement_neighbors(neighbors: &mut [usize], size: usize, index: i32) {
    let x = index % size as i32;
    let y = index / size as i32;

    #[rustfmt::skip]
    let indices = [
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y),                 (x + 1, y),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
    ];
    for (x, y) in indices.into_iter() {
        if x >= 0 && y >= 0 && x < size as i32 && y < size as i32 {
            let value = &mut neighbors[(y * size as i32 + x) as usize];
            *value -= 1;
        }
    }
}

pub fn part_two_bench(input: &str) -> usize {
    let input = input.as_bytes();
    let size = input.iter().position(|c| *c == b'\n').unwrap();

    let mut neighbors = vec![0; size * size];
    let mut paper_indices = Vec::with_capacity(size * size);
    let mut removed_paper_indices = Vec::with_capacity(size * size);

    for (y, row) in (0..size)
        // Unrolling the split by b'\n' saves ~4us.
        .map(|y| {
            let start = y * size + y;
            &input[start..start + size]
        })
        .enumerate()
    {
        let y_index = y * size;
        for (x, b) in row.iter().enumerate() {
            if *b == b'@' {
                paper_indices.push(y_index + x);
                increment_neighbors(&mut neighbors, size, x as i32, y as i32);
            }
        }
    }

    let mut sum = 0;
    loop {
        let start_sum = sum;
        let mut i = 0;
        // Loop instead of `(0..paper_indices.len()).rev()` in order to retain
        // sequential forward memory accesses, saves ~60us.
        while i < paper_indices.len() {
            let index = paper_indices[i];
            let retain = neighbors[index] >= 4;
            sum += !retain as usize;
            if !retain {
                removed_paper_indices.push(index as i32);
                // swap_remove is ~120us faster than retain.
                paper_indices.swap_remove(i);
            }
            i += 1;
        }

        if start_sum == sum {
            break;
        }

        for removed_index in removed_paper_indices.drain(..) {
            decrement_neighbors(&mut neighbors, size, removed_index);
        }
    }
    sum
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("../../inputs/4.txt");
    #[test]
    fn part_one() {
        crate::test::verify_results(INPUT, &[super::part_one, super::part_one_bench], 1505);
    }
    #[test]
    fn part_two() {
        crate::test::verify_results(INPUT, &[super::part_two, super::part_two_bench], 9182);
    }
}
