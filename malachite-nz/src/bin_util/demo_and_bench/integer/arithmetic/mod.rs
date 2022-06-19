use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs::register(runner);
    add::register(runner);
    add_mul::register(runner);
    div::register(runner);
    div_exact::register(runner);
    div_mod::register(runner);
    div_round::register(runner);
    divisible_by::register(runner);
    divisible_by_power_of_2::register(runner);
    eq_mod::register(runner);
    eq_mod_power_of_2::register(runner);
    extended_gcd::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    mul::register(runner);
    neg::register(runner);
    parity::register(runner);
    pow::register(runner);
    power_of_2::register(runner);
    root::register(runner);
    round_to_multiple::register(runner);
    round_to_multiple_of_power_of_2::register(runner);
    shl::register(runner);
    shl_round::register(runner);
    shr::register(runner);
    shr_round::register(runner);
    sign::register(runner);
    sqrt::register(runner);
    square::register(runner);
    sub::register(runner);
    sub_mul::register(runner);
}

mod abs;
mod add;
mod add_mul;
mod div;
mod div_exact;
mod div_mod;
mod div_round;
mod divisible_by;
mod divisible_by_power_of_2;
mod eq_mod;
mod eq_mod_power_of_2;
mod extended_gcd;
mod mod_op;
mod mod_power_of_2;
mod mul;
mod neg;
mod parity;
mod pow;
mod power_of_2;
mod root;
mod round_to_multiple;
mod round_to_multiple_of_power_of_2;
mod shl;
mod shl_round;
mod shr;
mod shr_round;
mod sign;
mod sqrt;
mod square;
mod sub;
mod sub_mul;
