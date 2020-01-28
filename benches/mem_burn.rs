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
