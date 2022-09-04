use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use regex::Regex as SRegex;
use vec_reg::{vec_reg, CompiledRegex, Regex as VRegex};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_reg");
    for size in 1..=30 {
        group.bench_with_input(BenchmarkId::new("vec-reg", size), &size, |b, &n| {
            let target = build_target_vec_reg_array_with_size(n);
            let regex = build_target_vec_reg_with_size(n);
            let compiled_regex = regex.compile();
            b.iter(|| compiled_regex.is_full_match(&target));
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// Build a?^na^n regex in vec_reg
fn build_target_vec_reg_with_size(n: usize) -> VRegex<char> {
    let is_one = |x: &char| *x == 'a';
    let mut regs = vec![];
    for _ in 1..=n {
        regs.push(vec_reg!([is_one]?));
    }
    for _ in 1..=n {
        regs.push(vec_reg!([is_one]));
    }

    regs.into_iter()
        .reduce(VRegex::concat)
        .unwrap()
}

fn build_target_vec_reg_array_with_size(n: usize) -> Vec<char> {
    vec!['a'; n]
}

#[allow(dead_code)]
fn build_target_regex_with_size(n: usize) -> SRegex {
    let pattern = format!(".*?^({}{})$.*?", "a?".repeat(n), "a".repeat(n));
    SRegex::new(&pattern).unwrap()
}

#[allow(dead_code)]
fn build_target_regex_array_with_size(n: usize) -> String {
    "a".repeat(n)
}
