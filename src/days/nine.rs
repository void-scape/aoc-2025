// FIRST PASS

#[derive(Debug)]
struct Quad {
    tl: (usize, usize),
    br: (usize, usize),
}

impl Quad {
    fn area(&self) -> usize {
        let width = self.br.0 - self.tl.0 + 1;
        let height = self.br.1 - self.tl.1 + 1;
        width * height
    }

    fn edges(&self) -> Vec<Edge> {
        [
            self.tl,
            (self.br.0, self.tl.1),
            self.br,
            (self.tl.0, self.br.1),
            self.tl,
        ]
        .windows(2)
        .map(|points| Edge {
            start: points[0],
            end: points[1],
        })
        .collect::<Vec<_>>()
    }

    fn is_inside(&self, edge: &Edge) -> bool {
        edge.start.0 > self.tl.0
            && edge.end.0 < self.br.0
            && edge.start.1 > self.tl.1
            && edge.end.1 < self.br.1
    }
}

pub fn part_one(input: &str) -> usize {
    //     let input = r#"7,1
    // 11,1
    // 11,7
    // 9,7
    // 9,5
    // 2,5
    // 2,3
    // 7,3"#;

    let mut points = Vec::new();
    for line in input.lines() {
        let mut iter = line.split(',');
        let x = iter.next().unwrap().parse::<usize>().unwrap();
        let y = iter.next().unwrap().parse::<usize>().unwrap();
        points.push((x, y));
    }

    let mut quads = Vec::new();
    for (i, point) in points.iter().enumerate() {
        for other in points.iter().skip(i + 1) {
            let tx = point.0.min(other.0);
            let ty = point.1.min(other.1);

            let bx = point.0.max(other.0);
            let by = point.1.max(other.1);

            quads.push(Quad {
                tl: (tx, ty),
                br: (bx, by),
            });
        }
    }

    quads.sort_by_key(|quad| quad.area());
    quads.last().unwrap().area()
}

#[derive(Debug)]
struct Edge {
    start: (usize, usize),
    end: (usize, usize),
}

impl Edge {
    fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        let tx = start.0.min(end.0);
        let ty = start.1.min(end.1);

        let bx = start.0.max(end.0);
        let by = start.1.max(end.1);

        let slf = Self {
            start: (tx, ty),
            end: (bx, by),
        };
        assert!(slf.is_yline() || slf.is_xline());
        slf
    }

    // fn corners_overlap(&self, rhs: &Self) -> bool {
    //     (self.is_yline() && (self.start.0..=self.end.0).contains(&rhs.start.0))
    //         || (self.is_xline() && (self.start.1..=self.end.1).contains(&rhs.start.1))
    //             && !self.overlap(rhs)
    // }

    fn overlap(&self, rhs: &Self) -> bool {
        if (self.is_xline() && rhs.is_yline()) || (self.is_yline() && rhs.is_xline()) {
            return false;
        }

        // let any_overlap = (self.is_yline() && (self.start.0..=self.end.0).contains(&rhs.start.0))
        //     || (self.is_xline() && (self.start.1..=self.end.1).contains(&rhs.start.1));
        // if !any_overlap {
        //     return false;
        // }
        //
        if self.is_yline() {
            let ymin = self.start.1.max(rhs.start.1);
            let ymax = self.end.1.min(rhs.end.1);
            if ymin > ymax { false } else { ymax - ymin > 1 }
        } else {
            let xmin = self.start.0.max(rhs.start.0);
            let xmax = self.end.0.min(rhs.end.0);

            if xmin > xmax { false } else { xmax - xmin > 1 }
        }
    }

    fn is_yline(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_xline(&self) -> bool {
        !self.is_yline()
    }

    fn perpendicular(&self, rhs: &Self) -> bool {
        if self.is_yline() {
            rhs.is_xline() && self.end.0 >= rhs.start.0 && self.end.0 <= rhs.end.0
        } else {
            rhs.is_yline() && self.end.1 >= rhs.start.1 && self.end.0 <= rhs.end.0
        }
    }
}

pub fn part_two(input: &str) -> usize {
    // 1393598336 # too low
    // 1393598336

    //     let input = r#"7,1
    // 11,1
    // 11,7
    // 9,7
    // 9,5
    // 2,5
    // 2,3
    // 7,3"#;

    let mut points = Vec::new();
    for line in input.lines() {
        let mut iter = line.split(',');
        let x = iter.next().unwrap().parse::<usize>().unwrap();
        let y = iter.next().unwrap().parse::<usize>().unwrap();
        points.push((x, y));
    }

    let edges = points
        .windows(2)
        .map(|points| Edge {
            start: points[0],
            end: points[1],
        })
        .collect::<Vec<_>>();

    let xmin = points.iter().min_by_key(|p| p.0).map(|p| p.0).unwrap();
    let xmax = points.iter().max_by_key(|p| p.0).map(|p| p.0).unwrap();
    let ymin = points.iter().min_by_key(|p| p.1).map(|p| p.1).unwrap();
    let ymax = points.iter().max_by_key(|p| p.1).map(|p| p.1).unwrap();

    println!("{}", points.len());
    let mut quads = Vec::new();
    for (i, point) in points.iter().enumerate() {
        println!("{i}");
        'outer: for other in points.iter() {
            if !(point.0 < other.0 && point.1 < other.1) {
                continue 'outer;
            }

            // let tx = point.0.min(other.0);
            // let ty = point.1.min(other.1);
            //
            // let bx = point.0.max(other.0);
            // let by = point.1.max(other.1);

            let quad = Quad {
                tl: *point,
                br: *other,
            };

            // verify

            let quad_edges = quad.edges();

            for edge in &quad_edges {
                if edges.iter().any(|e| e.overlap(edge)) {
                    break;
                }
                continue 'outer;
            }

            for edge in &quad_edges {
                if edges.iter().any(|e| {
                    let result = //edge.perpendicular(e) && 
                        quad.is_inside(e);
                    // if point.0 == 2
                    //     && point.1 == 3
                    //     && other.0 == 9
                    //     && other.1 == 7
                    //     && e.start.0 == 9
                    //     && e.start.1 == 5
                    // {
                    //     println!("perp: {}", edge.perpendicular(e));
                    //     println!("inside: {}", quad.is_inside(e));
                    //     println!("{result:#?}, {e:#?}");
                    //     println!("{:?}, {:?}", point, other);
                    // }
                    result
                }) {
                    for x in quad.tl.0 + 1..quad.br.0 {
                        for y in quad.tl.1 + 1..quad.br.1 {
                            let winding = edges
                                .iter()
                                .filter(|edge| {
                                    let tx = edge.start.0.min(edge.end.0);
                                    let ty = edge.start.1.min(edge.end.1);

                                    let bx = edge.start.0.max(edge.end.0);
                                    let by = edge.start.1.max(edge.end.1);

                                    // if point.0 == 9 && point.1 == 5 || point.0 == 2 && point.1 == 3 {
                                    //     println!("edge: {edge:#?}");
                                    // }

                                    if tx != bx {
                                        false
                                    } else {
                                        x > tx && y > ty && y < by

                                        // for x in x..=xmax {
                                        //     if x == tx && y > ty && y < by {
                                        //         return true;
                                        //     }
                                        // }
                                        // false
                                    }
                                })
                                .count();

                            // if point.0 == 9 && point.1 == 5 || point.0 == 2 && point.1 == 3 {
                            //     println!("{:?}, {:?}, {}", point, other, winding);
                            // }

                            if winding.is_multiple_of(2) {
                                continue 'outer;
                            }
                        }
                    }

                    // continue 'outer;
                }
            }

            // for y in quad.tl.1 + 1..quad.br.1 {
            //     for x in quad.tl.0 + 1..quad.br.0 {
            //         let winding = edges
            //             .iter()
            //             .filter(|edge| {
            //                 let tx = edge.start.0.min(edge.end.0);
            //                 let ty = edge.start.1.min(edge.end.1);
            //
            //                 let bx = edge.start.0.max(edge.end.0);
            //                 let by = edge.start.1.max(edge.end.1);
            //
            //                 // if point.0 == 9 && point.1 == 5 || point.0 == 2 && point.1 == 3 {
            //                 //     println!("edge: {edge:#?}");
            //                 // }
            //
            //                 if tx != bx {
            //                     false
            //                 } else {
            //                     x > tx && y > ty && y < by
            //
            //                     // for x in x..=xmax {
            //                     //     if x == tx && y > ty && y < by {
            //                     //         return true;
            //                     //     }
            //                     // }
            //                     // false
            //                 }
            //             })
            //             .count();
            //
            //         // if point.0 == 9 && point.1 == 5 || point.0 == 2 && point.1 == 3 {
            //         //     println!("{:?}, {:?}, {}", point, other, winding);
            //         // }
            //
            //         if winding.is_multiple_of(2) {
            //             continue 'outer;
            //         }
            //     }
            // }

            //

            quads.push(quad);
        }
    }

    quads.sort_by_key(|quad| quad.area());
    // println!("{quads:#?}");
    quads.last().unwrap().area()
}

// SECOND PASS

pub fn part_one_bench(input: &str) -> usize {
    69
}

pub fn part_two_bench(input: &str) -> usize {
    69
}

#[cfg(test)]
mod test {
    const INPUT: &str = include_str!("../../inputs/9.txt");
    #[test]
    fn part_one() {
        crate::test::verify_results(INPUT, &[super::part_one, super::part_one_bench], 127551);
    }
    #[test]
    fn part_two() {
        crate::test::verify_results(INPUT, &[super::part_two, super::part_two_bench], 2347225200);
    }
}
