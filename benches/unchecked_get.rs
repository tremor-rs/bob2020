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

pub fn unchecked_get_slow() {
    let mut v = Vec::new();
    v.push(1);
    v.push(4);
    v.push(3);
    // we know the index 1 always exists! Rust might not
    let mut i = 0;
    loop {
        if let Some(v) = v.get_mut(1) {
            *v = 2;
        };
        if i > 10000 {
            break;
        }
        i += 1;
    }
}

pub fn unchecked_get_fast() {
    let mut v = Vec::new();
    v.push(1);
    v.push(4);
    v.push(3);
    // we know the index 1 always exists!
    let mut i = 0;
    loop {
        unsafe { *(v.get_unchecked_mut(1)) = 2 };
        if i > 10000 {
            break;
        }
        i += 1;
    }
}

pub fn compare_unchecked_get(c: &mut Criterion) {
    let slow = Fun::new("Checked", |b, _i| b.iter(|| unchecked_get_slow()));
    let fast = Fun::new("Unchecked", |b, _i| b.iter(|| unchecked_get_fast()));

    let functions = vec![slow, fast];

    c.bench_functions("Vector Get", functions, 200);
}

criterion_group!(benches, compare_unchecked_get,);
