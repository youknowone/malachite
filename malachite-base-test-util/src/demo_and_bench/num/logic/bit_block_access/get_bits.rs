use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base_test_util::bench::bucketers::get_bits_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    signed_unsigned_unsigned_triple_gen_var_2, unsigned_triple_gen_var_5,
};
use malachite_base_test_util::num::logic::bit_block_access::get_bits_naive;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_get_bits);
    register_signed_demos!(runner, demo_signed_get_bits);
    register_unsigned_benches!(runner, benchmark_unsigned_get_bits_algorithms);
    register_signed_benches!(runner, benchmark_signed_get_bits_algorithms);
}

fn demo_unsigned_get_bits<T: BitBlockAccess<Bits = T> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, start, end) in unsigned_triple_gen_var_5::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.get_bits({}, {}) = {}",
            n,
            start,
            end,
            n.get_bits(start, end)
        );
    }
}

fn demo_signed_get_bits<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    T::Bits: PrimitiveUnsigned,
{
    for (n, start, end) in signed_unsigned_unsigned_triple_gen_var_2::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "({}).get_bits({}, {}) = {}",
            n,
            start,
            end,
            n.get_bits(start, end)
        );
    }
}

fn benchmark_unsigned_get_bits_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.get_bits(u64, u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_triple_gen_var_5::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &get_bits_bucketer(),
        &mut [
            ("default", &mut |(n, start, end)| {
                no_out!(n.get_bits(start, end))
            }),
            ("naive", &mut |(n, start, end)| {
                no_out!(get_bits_naive::<T, T>(&n, start, end))
            }),
        ],
    );
}

fn benchmark_signed_get_bits_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.get_bits(u64, u64)", T::NAME),
        BenchmarkType::Algorithms,
        signed_unsigned_unsigned_triple_gen_var_2::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &get_bits_bucketer(),
        &mut [
            ("default", &mut |(n, start, end)| {
                no_out!(n.get_bits(start, end))
            }),
            ("naive", &mut |(n, start, end)| {
                no_out!(get_bits_naive::<T, T::UnsignedOfEqualWidth>(&n, start, end))
            }),
        ],
    );
}
