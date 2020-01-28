use criterion::criterion_group;
use criterion::Criterion;
use criterion::Fun;

use std::borrow::Cow;

fn user_input() -> &'static str {
    "test" // fake corpus of well known inputs
}

pub fn static_borrow_slow() {
    // slow

    let mut i = 0;
    loop {
        let input = user_input();
        let _s: Cow<'static, str> = Cow::Owned(String::from(input));
        if i > 10000 {
            break;
        }
        i += 1;
    }
}

pub fn static_borrow_fast() {
    // better when a large set of inputs are well know
    fn to_cow(s: &str) -> Cow<'static, str> {
        match s {
            // Known
            "test" => Cow::Borrowed("test"),
            "well" => Cow::Borrowed("well"),
            "know" => Cow::Borrowed("know"),
            // Unknown
            _ => Cow::Owned(s.to_string()),
        }
    }

    let mut i = 0;
    loop {
        let input = user_input();
        let _s = to_cow(input);
        if i > 10000 {
            break;
        }
        i += 1
    }
}

pub fn compare_static_borrow(c: &mut Criterion) {
    let slow = Fun::new("Owned COW", |b, _i| b.iter(|| static_borrow_slow()));
    let fast = Fun::new("Borrowed COW", |b, _i| b.iter(|| static_borrow_fast()));

    let functions = vec![slow, fast];

    c.bench_functions("Static Borrow", functions, 20);
}

criterion_group!(benches, compare_static_borrow,);
