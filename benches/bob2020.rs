use criterion::criterion_main;

mod borrow_mut;
mod mem_burn;
mod mpsc_bcast;
mod static_borrow;
mod unchecked_get;

criterion_main! {
    borrow_mut::benches,
    mem_burn::benches,
    static_borrow::benches,
    unchecked_get::benches,
    mpsc_bcast::benches,
}
