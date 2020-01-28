use criterion::criterion_group;
use criterion::Criterion;
use criterion::Fun;

pub fn burn_mem_slow() {
    // slow - requires relocating and copying the memory
    let mut v = Vec::new();
    let mut i = 0;
    loop {
        v.push("snot");
        if i >= 10000 {
            break;
        };
        i += 1;
    }
}

pub fn burn_mem_fast() {
    let mut v = Vec::with_capacity(10000);
    let mut i = 0;
    loop {
        v.push("snot");
        if i >= 10000 {
            break;
        };
        i += 1;
    }
}

pub fn compare_burn_mem(c: &mut Criterion) {
    let slow = Fun::new("Unbounded", |b, _i| b.iter(|| burn_mem_slow()));
    let fast = Fun::new("Bounded [Preallocated]", |b, _i| b.iter(|| burn_mem_fast()));

    let functions = vec![slow, fast];

    c.bench_functions("Bounded Vector", functions, 20);
}

criterion_group!(benches, compare_burn_mem,);
