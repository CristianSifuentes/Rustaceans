// Concept: Lock-free programming with atomics.
//
// A `Mutex` guarantees exclusive access by potentially *blocking* a
// thread until the lock is free. `std::sync::atomic` types instead
// use hardware-level atomic CPU instructions (compare-and-swap,
// fetch-and-add) to update memory without ever blocking - at the cost
// of the programmer having to reason explicitly about memory
// ordering: how much a given atomic operation is allowed to be
// reordered relative to other memory accesses on the same thread,
// and what a different thread is guaranteed to observe once it sees
// the new value.
//
//   - `Ordering::Relaxed`   - only the atomicity of this one
//     operation is guaranteed; no ordering relative to other memory
//     accesses. Correct for independent counters nobody synchronizes
//     other data through.
//   - `Ordering::Acquire` / `Release` - a `Release` store paired with
//     an `Acquire` load creates a happens-before edge: everything the
//     writing thread did *before* the `Release` store is guaranteed
//     visible to the reading thread *after* the matching `Acquire`
//     load. This is how a flag can safely "publish" other, non-atomic
//     writes to another thread.
//   - `Ordering::SeqCst`    - the strongest ordering: all threads
//     agree on a single global order of every SeqCst operation.
//     Easiest to reason about, but the most expensive on some
//     architectures.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;

fn relaxed_counter_demo() {
    // A pure counter with no other data depending on it: `Relaxed` is
    // sufficient because `fetch_add` is atomic regardless of
    // ordering, and no other memory access needs to be synchronized
    // against these increments.
    let counter = Arc::new(AtomicU64::new(0));

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..10_000 {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    println!("relaxed lock-free counter: {}", counter.load(Ordering::Relaxed));
}

fn acquire_release_demo() {
    // `payload` is ordinary, non-atomic data. `ready` is the atomic
    // flag that "publishes" it: the writer thread performs a
    // `Release` store only after finishing the write, and the reader
    // thread performs an `Acquire` load before reading the payload -
    // that pairing is what guarantees the reader sees the finished
    // write and not a torn or stale one, without taking a lock.
    static mut PAYLOAD: u64 = 0;
    let ready = Arc::new(AtomicBool::new(false));

    let writer_ready = Arc::clone(&ready);
    let writer = thread::spawn(move || {
        // SAFETY: this write happens strictly before the `Release`
        // store below, and no other thread reads `PAYLOAD` until it
        // observes that store via a matching `Acquire` load.
        unsafe {
            PAYLOAD = 42;
        }
        writer_ready.store(true, Ordering::Release);
    });
    writer.join().unwrap();

    // Spin until the flag is published, using `Acquire` to establish
    // the happens-before edge with the writer's `Release` store.
    while !ready.load(Ordering::Acquire) {
        thread::yield_now();
    }
    // SAFETY: the `Acquire` load above happened-after the writer's
    // `Release` store, so this read is guaranteed to observe
    // `PAYLOAD = 42`, not an uninitialized or torn value.
    let observed = unsafe { PAYLOAD };
    println!("acquire/release published payload: {observed}");
}

fn compare_exchange_demo() {
    // Compare-and-swap is the primitive lock-free data structures
    // (queues, stacks) are built from: "update the value only if it
    // still matches what I last read", retrying on failure instead
    // of blocking.
    let state = AtomicU64::new(0);
    let mut current = state.load(Ordering::SeqCst);
    loop {
        let next = current + 100;
        match state.compare_exchange(current, next, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => break,
            Err(actual) => current = actual, // someone else won the race; retry
        }
    }
    println!("compare_exchange result: {}", state.load(Ordering::SeqCst));
}

fn main() {
    relaxed_counter_demo();
    acquire_release_demo();
    compare_exchange_demo();
}
