use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::triples_of_natural_natural_and_unsigned;
use malachite_base::num::SignificantBits;
use malachite_base::num::{AddMul, AddMulAssign};
use std::cmp::max;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_add_mul_assign_u32);
    register_demo!(registry, demo_natural_add_mul_assign_u32_ref);
    register_demo!(registry, demo_natural_add_mul_u32);
    register_demo!(registry, demo_natural_add_mul_u32_val_ref);
    register_demo!(registry, demo_natural_add_mul_u32_ref_val);
    register_demo!(registry, demo_natural_add_mul_u32_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_assign_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_assign_u32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_assign_u32_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_u32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_add_mul_u32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_u32_val_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_u32_ref_val_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_u32_ref_ref_algorithms
    );
}

fn demo_natural_add_mul_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_natural_natural_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.add_mul_assign(b, c);
        println!(
            "a := {}; x.add_mul_assign({}, {}); x = {}",
            a_old, b_old, c, a
        );
    }
}

fn demo_natural_add_mul_assign_u32_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_natural_natural_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        a.add_mul_assign(&b, c);
        println!("a := {}; x.add_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

fn demo_natural_add_mul_u32(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("{}.add_mul({}, {}) = {}", a_old, b_old, c, a.add_mul(b, c));
    }
}

fn demo_natural_add_mul_u32_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.add_mul(&{}, {}) = {}", a_old, b, c, a.add_mul(&b, c));
    }
}

fn demo_natural_add_mul_u32_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "(&{}).add_mul({}, {}) = {}",
            a_old,
            b_old,
            c,
            (&a).add_mul(b, c)
        );
    }
}

fn demo_natural_add_mul_u32_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).add_mul(&{}, {}) = {}",
            a_old,
            b,
            c,
            (&a).add_mul(&b, c)
        );
    }
}

fn benchmark_natural_add_mul_assign_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.add_mul_assign(Natural, u32)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.add_mul_assign(Natural, u32)",
                &mut (|(mut a, b, c)| a.add_mul_assign(b, c)),
            ),
            (
                "Natural.add_mul_assign(&Natural, u32)",
                &mut (|(mut a, b, c)| a.add_mul_assign(&b, c)),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_assign_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.add_mul_assign(Natural, u32)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.add_mul_assign(Natural, u32)",
                &mut (|(mut a, b, c)| a.add_mul_assign(b, c)),
            ),
            (
                "Natural += Natural * u32",
                &mut (|(mut a, b, c)| a += b * c),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_assign_u32_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.add_mul_assign(&Natural, u32)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.add_mul_assign(&Natural, u32)",
                &mut (|(mut a, b, c)| a.add_mul_assign(&b, c)),
            ),
            (
                "Natural += &Natural * u32",
                &mut (|(mut a, b, c)| a += &b * c),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.add_mul(Natural, u32)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.add_mul(Natural, u32)",
                &mut (|(a, b, c)| no_out!(a.add_mul(b, c))),
            ),
            (
                "Natural.add_mul(&Natural, u32)",
                &mut (|(a, b, c)| no_out!(a.add_mul(&b, c))),
            ),
            (
                "(&Natural).add_mul(Natural, u32)",
                &mut (|(a, b, c)| no_out!((&a).add_mul(b, c))),
            ),
            (
                "(&Natural).add_mul(&Natural, u32)",
                &mut (|(a, b, c)| no_out!((&a).add_mul(&b, c))),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.add_mul(Natural, u32)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.add_mul(Natural, u32)",
                &mut (|(a, b, c)| no_out!(a.add_mul(b, c))),
            ),
            (
                "Natural + Natural * u32",
                &mut (|(a, b, c)| no_out!(a + b * c)),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_u32_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.add_mul(&Natural, u32)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.add_mul(&Natural, u32)",
                &mut (|(a, b, c)| no_out!(a.add_mul(&b, c))),
            ),
            (
                "Natural + &Natural * u32",
                &mut (|(a, b, c)| no_out!(a + &b * c)),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_u32_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Natural).add_mul(Natural, u32)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "(&Natural).add_mul(Natural, u32)",
                &mut (|(a, b, c)| no_out!((&a).add_mul(b, c))),
            ),
            (
                "(&Natural) + Natural * u32",
                &mut (|(a, b, c)| no_out!(&a + b * c)),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_u32_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Natural).add_mul(&Natural, u32)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "(&Natural).add_mul(&Natural, u32)",
                &mut (|(a, b, c)| no_out!((&a).add_mul(&b, c))),
            ),
            (
                "(&Natural) + &Natural * u32",
                &mut (|(a, b, c)| no_out!(&a + &b * c)),
            ),
        ],
    );
}
