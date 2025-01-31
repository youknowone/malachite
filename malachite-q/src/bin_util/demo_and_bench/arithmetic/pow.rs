use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::{
    pair_1_rational_bit_bucketer, triple_3_pair_1_rational_bits_times_abs_pair_2_bucketer,
    triple_3_pair_1_rational_bits_times_pair_2_bucketer,
};
use malachite_q::test_util::generators::{
    rational_signed_pair_gen_var_2, rational_signed_pair_gen_var_2_nrm,
    rational_unsigned_pair_gen_var_1, rational_unsigned_pair_gen_var_1_nrm,
};
use rug::ops::Pow as RugPow;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_pow_u64);
    register_demo!(runner, demo_rational_pow_u64_ref);
    register_demo!(runner, demo_rational_pow_assign_u64);
    register_demo!(runner, demo_rational_pow_i64);
    register_demo!(runner, demo_rational_pow_i64_ref);
    register_demo!(runner, demo_rational_pow_assign_i64);

    register_bench!(runner, benchmark_rational_pow_u64_evaluation_strategy);
    register_bench!(runner, benchmark_rational_pow_u64_library_comparison);
    register_bench!(runner, benchmark_rational_pow_u64_assign);
    register_bench!(runner, benchmark_rational_pow_i64_evaluation_strategy);
    register_bench!(runner, benchmark_rational_pow_i64_library_comparison);
    register_bench!(runner, benchmark_rational_pow_i64_assign);
}

fn demo_rational_pow_u64(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, exp) in rational_unsigned_pair_gen_var_1::<u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!("({}).pow({}) = {}", n.clone(), exp, n.pow(exp));
    }
}

fn demo_rational_pow_u64_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, exp) in rational_unsigned_pair_gen_var_1::<u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!("(&{}).pow({}) = {}", n, exp, (&n).pow(exp));
    }
}

fn demo_rational_pow_assign_u64(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut n, exp) in rational_unsigned_pair_gen_var_1::<u64>()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n.clone();
        n.pow_assign(exp);
        println!("n := {}; n.pow_assign({}); n = {}", n_old, exp, n);
    }
}

fn demo_rational_pow_i64(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, exp) in rational_signed_pair_gen_var_2::<i64>()
        .get(gm, &config)
        .take(limit)
    {
        println!("({}).pow({}) = {}", n.clone(), exp, n.pow(exp));
    }
}

fn demo_rational_pow_i64_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, exp) in rational_signed_pair_gen_var_2::<i64>()
        .get(gm, &config)
        .take(limit)
    {
        println!("(&{}).pow({}) = {}", n, exp, (&n).pow(exp));
    }
}

fn demo_rational_pow_assign_i64(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut n, exp) in rational_signed_pair_gen_var_2::<i64>()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n.clone();
        n.pow_assign(exp);
        println!("n := {}; n.pow_assign({}); n = {}", n_old, exp, n);
    }
}

fn benchmark_rational_pow_u64_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.pow_assign(u64)",
        BenchmarkType::LibraryComparison,
        rational_unsigned_pair_gen_var_1_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_rational_bits_times_pair_2_bucketer("n", "pow"),
        &mut [
            ("Malachite", &mut |(_, _, (x, exp))| no_out!(x.pow(exp))),
            ("num", &mut |((x, exp), _, _)| {
                no_out!(x.pow(i32::exact_from(exp)))
            }),
            ("rug", &mut |(_, (x, exp), _)| {
                no_out!(x.pow(u32::exact_from(exp)))
            }),
        ],
    );
}

fn benchmark_rational_pow_u64_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.pow(u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_1::<u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Rational.pow(u64)", &mut |(n, exp)| no_out!(n.pow(exp))),
            ("(&Rational).pow(u64)", &mut |(n, exp)| {
                no_out!((&n).pow(exp))
            }),
        ],
    );
}

fn benchmark_rational_pow_u64_assign(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.pow_assign(u64)",
        BenchmarkType::Single,
        rational_unsigned_pair_gen_var_1::<u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, exp)| n.pow_assign(exp))],
    );
}

fn benchmark_rational_pow_i64_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.pow_assign(i64)",
        BenchmarkType::LibraryComparison,
        rational_signed_pair_gen_var_2_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_rational_bits_times_abs_pair_2_bucketer("n", "pow"),
        &mut [
            ("Malachite", &mut |(_, _, (x, exp))| no_out!(x.pow(exp))),
            ("num", &mut |((x, exp), _, _)| {
                no_out!(x.pow(i32::exact_from(exp)))
            }),
            ("rug", &mut |(_, (x, exp), _)| {
                no_out!(x.pow(i32::exact_from(exp)))
            }),
        ],
    );
}

fn benchmark_rational_pow_i64_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.pow(i64)",
        BenchmarkType::EvaluationStrategy,
        rational_signed_pair_gen_var_2::<i64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            ("Rational.pow(u64)", &mut |(n, exp)| no_out!(n.pow(exp))),
            ("(&Rational).pow(u64)", &mut |(n, exp)| {
                no_out!((&n).pow(exp))
            }),
        ],
    );
}

fn benchmark_rational_pow_i64_assign(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.pow_assign(i64)",
        BenchmarkType::Single,
        rational_signed_pair_gen_var_2::<i64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, exp)| n.pow_assign(exp))],
    );
}
