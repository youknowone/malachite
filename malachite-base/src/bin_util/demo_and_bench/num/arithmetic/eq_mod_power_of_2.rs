use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_1_2_max_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_signed_unsigned_triple_gen_var_2, unsigned_triple_gen_var_4,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_eq_mod_power_of_2_unsigned);
    register_signed_demos!(runner, demo_eq_mod_power_of_2_signed);

    register_unsigned_benches!(runner, benchmark_eq_mod_power_of_2_unsigned);
    register_signed_benches!(runner, benchmark_eq_mod_power_of_2_signed);
}

fn demo_eq_mod_power_of_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (x, y, pow) in unsigned_triple_gen_var_4::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        if x.eq_mod_power_of_2(y, pow) {
            println!("{} is equal to {} mod 2^{}", x, y, pow);
        } else {
            println!("{} is not equal to {} mod 2^{}", x, y, pow);
        }
    }
}

fn demo_eq_mod_power_of_2_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, pow) in signed_signed_unsigned_triple_gen_var_2::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        if x.eq_mod_power_of_2(y, pow) {
            println!("{} is equal to {} mod 2^{}", x, y, pow);
        } else {
            println!("{} is not equal to {} mod 2^{}", x, y, pow);
        }
    }
}

fn benchmark_eq_mod_power_of_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.eq_mod_power_of_2({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_4::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y, pow)| {
            no_out!(x.eq_mod_power_of_2(y, pow))
        })],
    );
}

fn benchmark_eq_mod_power_of_2_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.eq_mod_power_of_2({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_signed_unsigned_triple_gen_var_2::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y, pow)| {
            no_out!(x.eq_mod_power_of_2(y, pow))
        })],
    );
}
