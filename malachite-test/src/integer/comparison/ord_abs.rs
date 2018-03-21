use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_integers, rm_pairs_of_integers};
use malachite_base::num::SignificantBits;
use malachite_base::num::OrdAbs;
use std::cmp::{max, Ordering};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_cmp_abs);
    register_bench!(
        registry,
        Large,
        benchmark_integer_cmp_abs_library_comparison
    );
}

fn demo_integer_cmp_abs(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        match x.cmp_abs(&y) {
            Ordering::Less => println!("|{}| < |{}|", x, y),
            Ordering::Equal => println!("|{}| = |{}|", x, y),
            Ordering::Greater => println!("|{}| > |{}|", x, y),
        }
    }
}

fn benchmark_integer_cmp_abs_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.cmp_abs(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.cmp_abs(&y)))),
            ("rug", &mut (|((x, y), _)| no_out!(x.cmp_abs(&y)))),
        ],
    );
}
