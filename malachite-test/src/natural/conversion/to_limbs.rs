use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::{naturals, pairs_of_natural_and_small_usize};
use malachite_base::num::SignificantBits;

pub fn demo_natural_to_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_limbs_asc({}) = {:?}", n, n.to_limbs_asc());
    }
}

pub fn demo_natural_to_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_limbs_desc({}) = {:?}", n, n.to_limbs_desc());
    }
}

pub fn demo_natural_into_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("into_limbs_asc({}) = {:?}", n, n.clone().into_limbs_asc());
    }
}

pub fn demo_natural_into_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("into_limbs_desc({}) = {:?}", n, n.clone().into_limbs_desc());
    }
}

pub fn demo_natural_limbs(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("limbs({}) = {:?}", n, n.limbs().collect::<Vec<u32>>());
    }
}

pub fn demo_natural_limbs_rev(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!(
            "limbs({}).rev() = {:?}",
            n,
            n.limbs().rev().collect::<Vec<u32>>()
        );
    }
}

pub fn demo_natural_limbs_size_hint(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("limbs({}).size_hint() = {:?}", n, n.limbs().size_hint());
    }
}

pub fn demo_natural_limbs_index(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_natural_and_small_usize(gm).take(limit) {
        println!("limbs({})[{}] = {:?}", n, i, n.limbs()[i]);
    }
}

#[allow(unknown_lints, unused_collect)]
pub fn benchmark_natural_limbs_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.limbs()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            (
                "Natural.to_limbs_asc()",
                &mut (|n| no_out!(n.to_limbs_asc())),
            ),
            (
                "Natural.into_limbs_asc()",
                &mut (|n| no_out!(n.into_limbs_asc())),
            ),
            (
                "Natural.limbs().collect::<Vec<u32>>()",
                &mut (|n| no_out!(n.limbs().collect::<Vec<u32>>())),
            ),
        ],
    );
}

#[allow(unknown_lints, unused_collect)]
pub fn benchmark_natural_limbs_rev_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.limbs().rev()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            (
                "Natural.to_limbs_desc()",
                &mut (|n| no_out!(n.to_limbs_desc())),
            ),
            (
                "Natural.into_limbs_desc()",
                &mut (|n| no_out!(n.into_limbs_desc())),
            ),
            (
                "Natural.limbs().rev().collect::<Vec<u32>>()",
                &mut (|n| no_out!(n.limbs().rev().collect::<Vec<u32>>())),
            ),
        ],
    );
}

pub fn benchmark_natural_limbs_size_hint(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.limbs().size_hint()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            (
                "Natural.limbs().size_hint()",
                &mut (|n| no_out!(n.limbs().size_hint())),
            ),
        ],
    );
}

pub fn benchmark_natural_limbs_index_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.limbs().index()",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_small_usize(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("Natural.limbs()[u]", &mut (|(n, u)| no_out!(n.limbs()[u]))),
            (
                "Natural.into_limbs_asc()[u]",
                &mut (|(n, u)| {
                    let limbs = n.into_limbs_asc();
                    if u >= limbs.len() {
                        0
                    } else {
                        limbs[u]
                    };
                }),
            ),
        ],
    );
}
