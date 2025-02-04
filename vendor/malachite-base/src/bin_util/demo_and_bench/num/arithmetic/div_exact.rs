// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_pair_gen_var_3, unsigned_pair_gen_var_11};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_div_exact_unsigned);
    register_signed_demos!(runner, demo_div_exact_signed);
    register_unsigned_demos!(runner, demo_div_exact_assign_unsigned);
    register_signed_demos!(runner, demo_div_exact_assign_signed);

    register_unsigned_benches!(runner, benchmark_div_exact_unsigned);
    register_signed_benches!(runner, benchmark_div_exact_signed);
    register_unsigned_benches!(runner, benchmark_div_exact_assign_unsigned);
    register_signed_benches!(runner, benchmark_div_exact_assign_signed);
}

fn demo_div_exact_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_11::<T>().get(gm, config).take(limit) {
        println!("{}.div_exact({}) = {}", x, y, x.div_exact(y));
    }
}

fn demo_div_exact_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen_var_3::<T>().get(gm, config).take(limit) {
        println!("({}).div_exact({}) = {}", x, y, x.div_exact(y));
    }
}

fn demo_div_exact_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y) in unsigned_pair_gen_var_11::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.div_exact_assign(y);
        println!("x := {old_x}; x.div_exact_assign({y}); x = {x}");
    }
}

fn demo_div_exact_assign_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in signed_pair_gen_var_3::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.div_exact_assign(y);
        println!("x := {old_x}; x.div_exact_assign({y}); x = {x}");
    }
}

fn benchmark_div_exact_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_exact({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_11::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.div_exact(y)))],
    );
}

fn benchmark_div_exact_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_exact({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.div_exact(y)))],
    );
}

fn benchmark_div_exact_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_exact_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_11::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| x.div_exact_assign(y))],
    );
}

fn benchmark_div_exact_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_exact_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| x.div_exact_assign(y))],
    );
}
