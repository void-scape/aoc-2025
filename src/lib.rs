#![feature(portable_simd)]

pub mod days;

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
