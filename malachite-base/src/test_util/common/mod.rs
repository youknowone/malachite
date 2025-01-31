use std::cmp::Ordering;
use std::fmt::Debug;
use std::str::FromStr;

#[macro_export]
macro_rules! assert_panic {
    ($e: expr) => {
        let result = catch_unwind(|| $e);
        assert!(result.is_err());
    };
}

fn read_strings<T: FromStr>(strings: &[&str]) -> Vec<T>
where
    T::Err: Debug,
{
    strings.iter().map(|s| s.parse().unwrap()).collect()
}

fn test_helper_helper<
    T: Debug + FromStr,
    U: Debug + Eq,
    F: FnMut(usize, usize) -> U,
    G: FnMut(&T, &T) -> U,
>(
    strings: &[&str],
    mut compare_indices: F,
    mut compare_elements: G,
) where
    T::Err: Debug,
{
    let xs = read_strings::<T>(strings);
    let ys = read_strings::<T>(strings);
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            assert_eq!(compare_indices(i, j), compare_elements(x, y));
        }
    }
}

pub fn test_eq_helper<T: Debug + Eq + FromStr>(strings: &[&str])
where
    T::Err: Debug,
{
    test_helper_helper(strings, |i, j| i == j, |x: &T, y: &T| x == y);
}

pub fn test_cmp_helper<T: Debug + FromStr + Ord>(strings: &[&str])
where
    T::Err: Debug,
{
    test_helper_helper::<T, _, _, _>(strings, |i, j| i.cmp(&j), Ord::cmp);
}

pub fn test_custom_cmp_helper<T: Debug + FromStr, F: FnMut(&T, &T) -> Ordering>(
    strings: &[&str],
    compare: F,
) where
    T::Err: Debug,
{
    test_helper_helper(strings, |i, j| i.cmp(&j), compare);
}

#[macro_export]
macro_rules! triple_significant_bits_fn {
    ($t:ident, $bucketing_function:ident) => {
        fn $bucketing_function(t: &($t, $t, $t)) -> usize {
            usize::exact_from(max!(
                t.0.significant_bits(),
                t.1.significant_bits(),
                t.2.significant_bits()
            ))
        }
    };
}

pub const TRIPLE_SIGNIFICANT_BITS_LABEL: &str =
    "max(a.significant_bits(), b.significant_bits(), c.significant_bits())";

pub fn rle_decode<T: Clone>(ps: &[(T, usize)]) -> Vec<T> {
    let mut out = Vec::new();
    for (x, count) in ps.iter() {
        for _ in 0..*count {
            out.push(x.clone());
        }
    }
    out
}
