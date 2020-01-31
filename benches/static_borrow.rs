// Copyright 2018-2020, Wayfair GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use criterion::criterion_group;
use criterion::Criterion;
use criterion::Fun;
use rand::prelude::*;
use std::borrow::Cow;

fn user_input() -> &'static str {
    if rand::random::<u8>() % 3 == 0 {
        "test"
    } else {
        "badger"
    }
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
