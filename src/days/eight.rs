// FIRST PASS

use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    let mut points = Vec::new();
    for line in input.lines() {
        let mut iter = line.split(",");
        points.push((
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        ));
    }

    let mut connections = HashMap::<usize, Vec<(usize, f32)>>::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate() {
            if i == j {
                continue;
            }

            let dist = (((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2))
                as f32)
                .sqrt();

            connections.entry(i).or_default().push((j, dist));
        }
    }

    for connections in connections.values_mut() {
        connections.sort_by(|a, b| a.1.total_cmp(&b.1));
    }

    let mut shortest_connection = || {
        let mut shortest_dist = f32::MAX;
        let mut i = 0;
        let mut j = 0;
        for (key, value) in connections.iter_mut() {
            if let Some((jj, dist)) = value.first()
                && *dist < shortest_dist
            {
                shortest_dist = *dist;
                i = *key;
                j = *jj;
            }
        }
        if shortest_dist == f32::MAX {
            return None;
        }
        let set = connections.get_mut(&i).unwrap();
        set.remove(0);
        Some((i, j))
    };

    let mut connections = Vec::new();
    while let Some(connection) = {
        _ = shortest_connection();
        shortest_connection()
    } && connections.len() < 1000
    {
        connections.push(connection);
    }

    let mut circuits =
        Vec::<HashSet<usize>>::from_iter((0..points.len()).map(|i| HashSet::from_iter([i])));

    'outer: for connection in connections.iter() {
        for i in 0..circuits.len() {
            let t1 = connection.0;
            let t2 = connection.1;

            if circuits[i].contains(&t1) && circuits[i].contains(&t2) {
                continue 'outer;
            }

            let target = if circuits[i].contains(&t1) {
                t1
            } else if circuits[i].contains(&t2) {
                t2
            } else {
                continue;
            };

            circuits[i].insert(connection.0);
            circuits[i].insert(connection.1);

            for j in 0..circuits.len() {
                if i == j {
                    continue;
                }

                if circuits[j].contains(&target) {
                    let values = circuits[j].iter().copied().collect::<Vec<_>>();
                    circuits[i].extend(values);
                    circuits.remove(j);
                    continue 'outer;
                }
            }
        }
    }

    circuits.sort_by_key(|circuit| circuit.len());

    for (i, circuit) in circuits.iter().enumerate() {
        for (j, c) in circuits.iter().enumerate() {
            if i == j {
                continue;
            }
            assert!(circuit.iter().all(|index| !c.contains(index)));
        }
    }

    let mut total_len = 1;
    for circuit in circuits.iter().rev().take(3) {
        total_len *= circuit.len();
    }

    total_len
}

pub fn part_two(input: &str) -> usize {
    let mut points = Vec::new();
    for line in input.lines() {
        let mut iter = line.split(",");
        points.push((
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        ));
    }

    let mut connections = HashMap::<usize, Vec<(usize, f32)>>::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate() {
            if i == j {
                continue;
            }

            let dist = (((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2))
                as f32)
                .sqrt();

            connections.entry(i).or_default().push((j, dist));
        }
    }

    for connections in connections.values_mut() {
        connections.sort_unstable_by(|a, b| a.1.total_cmp(&b.1));
    }

    let mut shortest_connection = || {
        let mut shortest_dist = f32::MAX;
        let mut i = 0;
        let mut j = 0;
        for (key, value) in connections.iter_mut() {
            if let Some((jj, dist)) = value.first()
                && *dist < shortest_dist
            {
                shortest_dist = *dist;
                i = *key;
                j = *jj;
            }
        }
        if shortest_dist == f32::MAX {
            return None;
        }
        let set = connections.get_mut(&i).unwrap();
        set.remove(0);
        Some((i, j))
    };

    let mut connections = Vec::new();
    while let Some(connection) = {
        _ = shortest_connection();
        shortest_connection()
    } {
        connections.push(connection);
    }

    let mut circuits =
        Vec::<HashSet<usize>>::from_iter((0..points.len()).map(|i| HashSet::from_iter([i])));

    'outer: for connection in connections.iter() {
        for i in 0..circuits.len() {
            let t1 = connection.0;
            let t2 = connection.1;

            if circuits[i].contains(&t1) && circuits[i].contains(&t2) {
                continue 'outer;
            }

            let target = if circuits[i].contains(&t1) {
                t1
            } else if circuits[i].contains(&t2) {
                t2
            } else {
                continue;
            };

            circuits[i].insert(connection.0);
            circuits[i].insert(connection.1);

            for j in 0..circuits.len() {
                if i == j {
                    continue;
                }

                if circuits[j].contains(&target) {
                    let values = circuits[j].iter().copied().collect::<Vec<_>>();
                    circuits[i].extend(values);

                    if circuits.len() == 2 {
                        return (points[connection.0].0 * points[connection.1].0) as usize;
                    }

                    circuits.remove(j);

                    continue 'outer;
                }
            }
        }
    }
    unreachable!()
}

// SECOND PASS

pub fn part_one_bench(input: &str) -> usize {
    find_junction_circuits::<false>(input)
}

pub fn part_two_bench(input: &str) -> usize {
    find_junction_circuits::<true>(input)
}

fn find_junction_circuits<const EARLY_EXIT: bool>(input: &str) -> usize {
    let input = input.as_bytes();

    let mut points = Vec::new();
    for line in input.split(|c| *c == b'\n') {
        if line.is_empty() {
            break;
        }
        let mut iter = line.split(|c| *c == b',');
        points.push((
            crate::parse_usize(iter.next().unwrap()) as i64,
            crate::parse_usize(iter.next().unwrap()) as i64,
            crate::parse_usize(iter.next().unwrap()) as i64,
        ));
    }

    let mut connections = vec![Vec::with_capacity(points.len()); points.len()];
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate().skip(i + 1) {
            if i == j {
                continue;
            }

            let dist = (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2);
            connections[i].push((j, dist));
        }
    }

    let mut connections = connections
        .into_iter()
        .enumerate()
        .flat_map(|(i, set)| set.into_iter().map(move |(j, dist)| (i, j, dist)))
        .collect::<Vec<_>>();
    connections.sort_unstable_by(|a, b| a.2.cmp(&b.2));

    let mut circuits = Vec::<Vec<usize>>::from_iter((0..points.len()).map(|i| vec![i]));
    let take = if !EARLY_EXIT { 1000 } else { connections.len() };
    for connection in connections.iter().take(take) {
        let set1 = circuits
            .iter()
            .position(|set| set.contains(&connection.0))
            .unwrap();

        if circuits[set1].contains(&connection.1) {
            continue;
        }

        if EARLY_EXIT && circuits.len() == 2 {
            return (points[connection.0].0 * points[connection.1].0) as usize;
        }

        let set2 = circuits
            .iter()
            .position(|set| set.contains(&connection.1))
            .unwrap();

        let circuits2 = circuits.remove(set2);
        circuits[(set1 as i64 + -((set1 > set2) as i64)) as usize].extend(circuits2.into_iter());
    }

    circuits.sort_unstable_by_key(|c2| std::cmp::Reverse(c2.len()));
    circuits
        .iter()
        .map(|circuit| circuit.len())
        .take(3)
        .product()
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("../../inputs/8.txt");
    #[test]
    fn part_one() {
        crate::test::verify_results(INPUT, &[super::part_one, super::part_one_bench], 127551);
    }
    #[test]
    fn part_two() {
        crate::test::verify_results(INPUT, &[super::part_two, super::part_two_bench], 2347225200);
    }
}
