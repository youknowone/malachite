use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::bucketers::Bucketer;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

pub fn natural_bit_bucketer(var_name: &str) -> Bucketer<Natural> {
    Bucketer {
        bucketing_function: &|x| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn pair_1_natural_bit_bucketer<T>(var_name: &str) -> Bucketer<(Natural, T)> {
    Bucketer {
        bucketing_function: &|(x, _)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn triple_3_natural_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(T, U, Natural)> {
    Bucketer {
        bucketing_function: &|(_, _, x)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn integer_bit_bucketer(var_name: &str) -> Bucketer<Integer> {
    Bucketer {
        bucketing_function: &|x| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn triple_3_integer_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(T, U, Integer)> {
    Bucketer {
        bucketing_function: &|(_, _, x)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn natural_bit_ratio_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Natural, Natural)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(x.significant_bits() / y.significant_bits())
        },
        bucketing_label: format!(
            "{}.significant_bits() / {}.significant_bits()",
            x_name, y_name
        ),
    }
}
