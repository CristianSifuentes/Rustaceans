// Concept: Interior mutability.
//
// Rust's aliasing rule - "shared XOR mutable" - is normally enforced
// at compile time by the borrow checker. Interior mutability moves
// that enforcement to runtime instead, allowing you to mutate data
// reached through a shared (`&T`) reference as long as the actual
// access pattern is checked (and, if violated, rejected) while the
// program is running.
//
// * `Cell<T>`   - no runtime checks at all; only supports copying the
//                 whole value in/out, so aliasing is trivially safe.
// * `RefCell<T>` - tracks borrows dynamically and panics on conflict;
//                 single-threaded only.
// * `Mutex<T>` / `RwLock<T>` - the multithreaded analogues of
//                 `RefCell`: instead of panicking, a conflicting
//                 access blocks until the lock is released.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn refcell_demo() {
    // `Rc<RefCell<T>>` is the classic combo for shared, mutable,
    // single-threaded state: `Rc` allows multiple owners, `RefCell`
    // allows mutation through any one of those shared owners.
    let shared_log: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

    let writer_a = Rc::clone(&shared_log);
    writer_a.borrow_mut().push("writer_a: started".to_string());

    let writer_b = Rc::clone(&shared_log);
    writer_b.borrow_mut().push("writer_b: started".to_string());

    // Borrow rules are enforced dynamically here: this immutable
    // borrow would panic at runtime (not compile time) if a
    // `borrow_mut()` guard from above were still alive.
    for line in shared_log.borrow().iter() {
        println!("refcell log: {line}");
    }
}

fn cell_demo() {
    // `Cell<T>` never hands out a reference to the interior value at
    // all - `get`/`set` copy the value in and out - so there is
    // nothing for the borrow checker to police, no matter how many
    // shared references exist.
    let hits = Cell::new(0u32);
    let counter_ref = &hits;
    counter_ref.set(counter_ref.get() + 1);
    counter_ref.set(counter_ref.get() + 1);
    println!("cell hits: {}", hits.get());
}

fn mutex_demo() {
    // `Mutex<T>` is `RefCell`'s thread-safe sibling: instead of
    // panicking on a conflicting borrow, a second thread trying to
    // lock an already-locked mutex simply blocks until it is free.
    let counter = Arc::new(Mutex::new(0i64));

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..1000 {
                    let mut guard = counter.lock().unwrap();
                    *guard += 1;
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
    println!("mutex counter: {}", *counter.lock().unwrap());
}

fn rwlock_demo() {
    // `RwLock<T>` differentiates readers from writers: any number of
    // readers may hold the lock concurrently, but a writer needs
    // exclusive access - useful when reads vastly outnumber writes.
    let config = Arc::new(RwLock::new(String::from("v1")));

    let reader_handles: Vec<_> = (0..3)
        .map(|id| {
            let config = Arc::clone(&config);
            thread::spawn(move || {
                let value = config.read().unwrap();
                println!("rwlock reader {id} sees: {value}");
            })
        })
        .collect();
    for handle in reader_handles {
        handle.join().unwrap();
    }

    *config.write().unwrap() = String::from("v2");
    println!("rwlock after write: {}", config.read().unwrap());
}

fn main() {
    refcell_demo();
    cell_demo();
    mutex_demo();
    rwlock_demo();
}
