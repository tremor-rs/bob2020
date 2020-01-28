use std::borrow::Cow;
use std::collections::HashMap;
use std::mem::transmute;
use std::sync::mpsc::{self, Receiver, Sender};

pub fn borrow_mut() {
    let mut h = HashMap::new();
    h.insert("a", String::from("badger"));
    h.insert("b", String::from("snot "));

    /*
    //this doesn't work
    let a = h.get("a").unwrap();
    let b = h.get_mut("b").unwrap();
    b.push_str(a.as_str());
     */

    // this is slow
    let a = h.get("a").unwrap().clone();
    let b = h.get_mut("b").unwrap();
    b.push_str(a.as_str());

    // this is fast
    #[allow(mutable_transmutes)]
    unsafe {
        let a = h.get("a").unwrap();
        let b: &mut String = transmute(h.get("b").unwrap());
        b.push_str(a.as_str());
    }
}

fn maybe_continue() -> bool {
    false // simulate user input
}

pub fn burn_mem() {
    // slow - requires relocating and copying the memory
    let mut v = Vec::new();
    while maybe_continue() {
        v.push("snot");
    }

    // faster as long maybe_continue usually
    // returns false in less then 512 calls.
    let mut v = Vec::with_capacity(512);
    while maybe_continue() {
        v.push("snot");
    }
}

fn user_input() -> &'static str {
    "well"
}

pub fn static_borrows() {
    // slow
    let input = user_input();
    let s: Cow<'static, str> = Cow::Owned(String::from(input));

    // better when a large set of inputs are well know
    fn to_cow(s: &str) -> Cow<'static, str> {
        match s {
            "well" => Cow::Borrowed("well"),
            "know" => Cow::Borrowed("know"),
            _ => Cow::Owned(s.to_string()),
        }
    }
    let input = user_input();
    let s = to_cow(input);
}

pub fn unchecked_get() {
    let mut v = Vec::new();
    v.push(1);
    v.push(4);
    v.push(3);
    // we know the index 1 always exists! Rust might not
    if let Some(v) = v.get_mut(1) {
        *v = 2;
    };

    let mut v = Vec::new();
    v.push(1);
    v.push(4);
    v.push(3);
    // we know the index 1 always exists!
    unsafe { *(v.get_unchecked_mut(1)) = 2 };
}

#[derive(Clone)]
struct Msg {
    text: String,
}

fn mutliple_channels() -> Vec<Sender<Msg>> {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();
    vec![tx1, tx2, tx3]
}
pub fn no_clone_on_loop() {
    let txs: Vec<Sender<Msg>> = mutliple_channels();
    let msg = Msg {
        text: String::from("Hello world!"),
    };
    for tx in txs {
        tx.send(msg.clone());
    }
}

pub fn no_clone_on_loop_faster() {
    let txs: Vec<Sender<Msg>> = mutliple_channels();
    let msg = Msg {
        text: String::from("Hello world!"),
    };
    for tx in &txs[..txs.len() - 1] {
        tx.send(msg.clone());
    }
    if let Some(tx) = txs.last() {
        tx.send(msg);
    }
}

fn main() {}
