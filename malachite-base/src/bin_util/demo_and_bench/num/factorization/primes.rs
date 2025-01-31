use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_base::test_util::num::factorization::primes::primes_naive;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_primes_less_than);
    register_unsigned_demos!(runner, demo_primes_less_than_or_equal_to);
    register_unsigned_demos!(runner, demo_primes);

    register_unsigned_benches!(runner, benchmark_primes_less_than_algorithms);
    register_unsigned_benches!(runner, benchmark_primes_less_than_algorithms_2);
    register_unsigned_benches!(runner, benchmark_primes_less_than_or_equal_to_algorithms);
    register_unsigned_benches!(runner, benchmark_primes_less_than_or_equal_to_algorithms_2);
}

fn demo_primes_less_than<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for n in unsigned_gen_var_5::<T>().get(gm, &config).take(limit) {
        println!(
            "primes_less_than({}) = {:?}",
            n,
            T::primes_less_than(&n).collect_vec()
        );
    }
}

fn demo_primes_less_than_or_equal_to<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for n in unsigned_gen_var_5::<T>().get(gm, &config).take(limit) {
        println!(
            "primes_less_than_or_equal_to({}) = {:?}",
            n,
            T::primes_less_than_or_equal_to(&n).collect_vec()
        );
    }
}

fn demo_primes<T: PrimitiveUnsigned>(_gm: GenMode, _config: GenConfig, limit: usize) {
    for p in T::primes().take(limit) {
        println!("{}", p);
    }
}

fn benchmark_primes_less_than_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<T>,
{
    run_benchmark(
        &format!("{}::primes_less_than({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(T::primes_less_than(&n).count())),
            ("using primes", &mut |n| {
                no_out!(T::primes().take_while(|&p| p < n).count())
            }),
            ("naive", &mut |n| {
                no_out!(primes_naive::<T>().take_while(|&p| p < n).count())
            }),
        ],
    );
}

fn benchmark_primes_less_than_algorithms_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<T>,
{
    run_benchmark(
        &format!("{}::primes_less_than(&{})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(T::primes_less_than(&n).count())),
            ("using primes", &mut |n| {
                no_out!(T::primes().take_while(|&p| p < n).count())
            }),
        ],
    );
}

fn benchmark_primes_less_than_or_equal_to_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<T>,
{
    run_benchmark(
        &format!("{}::primes_less_than_or_equal_to(&{})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(T::primes_less_than_or_equal_to(&n).count())
            }),
            ("using primes", &mut |n| {
                no_out!(T::primes().take_while(|&p| p <= n).count())
            }),
            ("naive", &mut |n| {
                no_out!(primes_naive::<T>().take_while(|&p| p <= n).count())
            }),
        ],
    );
}

fn benchmark_primes_less_than_or_equal_to_algorithms_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<T>,
{
    run_benchmark(
        &format!("{}::primes_less_than_or_equal_to({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(T::primes_less_than_or_equal_to(&n).count())
            }),
            ("using primes", &mut |n| {
                no_out!(T::primes().take_while(|&p| p <= n).count())
            }),
        ],
    );
}
