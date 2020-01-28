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

use std::collections::HashMap;
use std::mem::transmute;

use criterion::criterion_group;
use criterion::Criterion;
use criterion::Fun;

pub fn borrow_mut_slow() {
    let mut h = HashMap::new();
    h.insert("a", String::from("badger"));
    h.insert("b", String::from("snot "));

    /*
    //this doesn't work
    let a = h.get("a").unwrap();
    let b = h.get_mut("b").unwrap();
    b.push_str(a.as_str());
     */

    let a = h.get("a").unwrap().clone();
    let b = h.get_mut("b").unwrap();
    b.push_str(a.as_str());
}

pub fn borrow_mut_fast() {
    let mut h = HashMap::new();
    h.insert("a", String::from("badger"));
    h.insert("b", String::from("snot "));

    #[allow(mutable_transmutes)]
    unsafe {
        let a = h.get("a").unwrap();
        let b: &mut String = transmute(h.get("b").unwrap());
        b.push_str(a.as_str());
    }
}

pub fn compare_borrow_mut(c: &mut Criterion) {
    let slow = Fun::new("Clone", |b, _i| b.iter(|| borrow_mut_slow()));
    let fast = Fun::new("Mutable Transmute", |b, _i| b.iter(|| borrow_mut_fast()));

    let functions = vec![slow, fast];

    c.bench_functions("Borrow Mut", functions, 20);
}

criterion_group!(benches, compare_borrow_mut,);
