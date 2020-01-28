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

use std::sync::mpsc::{self, Sender};

#[derive(Clone)]
struct Msg {
    text: String,
}
fn mutliple_channels() -> Vec<Sender<Msg>> {
    let (tx1, _rx1) = mpsc::channel();
    let (tx2, _rx2) = mpsc::channel();
    let (tx3, _rx3) = mpsc::channel();
    vec![tx1, tx2, tx3]
}
pub fn mpsc_bcast_slow() {
    let txs: Vec<Sender<Msg>> = mutliple_channels();
    let msg = Msg {
        text: String::from("Hello world!"),
    };
    for tx in txs {
        let _ = tx.send(msg.clone());
    }
}

pub fn mpsc_bcast_fast() {
    let txs: Vec<Sender<Msg>> = mutliple_channels();
    let msg = Msg {
        text: String::from("Hello world!"),
    };
    for tx in &txs[..txs.len() - 1] {
        let _ = tx.send(msg.clone());
    }
    if let Some(tx) = txs.last() {
        let _ = tx.send(msg);
    }
}

pub fn compare_mscp_bcast(c: &mut Criterion) {
    let slow = Fun::new("Clone Send", |b, _i| b.iter(|| mpsc_bcast_slow()));
    let fast = Fun::new("No Clone Send", |b, _i| b.iter(|| mpsc_bcast_fast()));

    let functions = vec![slow, fast];

    c.bench_functions("MPSC Multi-Send", functions, 200);
}

criterion_group!(benches, compare_mscp_bcast,);
