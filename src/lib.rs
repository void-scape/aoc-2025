#![feature(portable_simd)]
#![feature(slice_split_once)]

pub mod days;

pub fn parse_usize(input: &[u8]) -> usize {
    let mut accum = 0;
    for b in input.iter() {
        accum *= 10;
        accum += (*b - b'0') as usize;
    }
    accum
}

#[cfg(test)]
pub mod test {
    use std::fmt::Debug;

    pub fn verify_results<D: Debug + PartialEq>(input: &str, fns: &[fn(&str) -> D], expected: D) {
        let results = fns.iter().map(|f| f(input)).collect::<Vec<_>>();
        assert_eq!(results[0], expected);
        assert!(
            results.iter().skip(1).all(|result| *result == results[0]),
            "{:?}",
            results
        );
    }
}
