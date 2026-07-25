// Concept: `Send` and `Sync`.
//
// These are auto traits - the compiler derives them automatically
// for any type whose fields are themselves Send/Sync, with no impl
// block required - that describe thread-safety, not the ownership
// rules `Send`/`Sync` are sometimes confused with:
//   - `Send`: ownership of a value of this type may be transferred to
//     another thread. Most types are `Send`; `Rc<T>` is the classic
//     exception, because its non-atomic reference count would race if
//     two threads cloned/dropped it concurrently.
//   - `Sync`: a `&T` reference to this type may be shared across
//     threads simultaneously (equivalently: `T` is `Sync` iff `&T` is
//     `Send`). `Cell<T>`/`RefCell<T>` are not `Sync`, because their
//     interior mutation has no thread synchronization at all.
//
// Violating either bound is a compile-time error, not a runtime
// race - the type system rules out the data race before the program
// ever runs.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    // `i32`, `String`, `Vec<T: Send>` etc. are all `Send` and `Sync`
    // automatically - no manual impl anywhere in std for these.
    assert_send::<i32>();
    assert_sync::<String>();
    assert_send::<Vec<u8>>();

    // `Arc<Mutex<T>>` is the standard "share across threads with
    // interior mutability" combo: `Arc`'s refcount is atomic (so
    // cloning/dropping across threads is race-free -> Send), and
    // `Mutex` provides the synchronization that makes concurrent
    // access to the interior sound -> Sync.
    assert_send::<Arc<Mutex<i32>>>();
    assert_sync::<Arc<Mutex<i32>>>();

    let shared = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..3)
        .map(|id| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || {
                let mut guard = shared.lock().unwrap();
                *guard += id;
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Arc<Mutex<i32>> total: {}", *shared.lock().unwrap());

    // `Rc<T>` is deliberately NOT `Send`: uncomment the block below
    // and the crate fails to compile, because `thread::spawn`
    // requires its closure's captures to be `Send`.
    //
    // let not_shareable = Rc::new(5);
    // thread::spawn(move || println!("{}", not_shareable));
    //   error[E0277]: `Rc<i32>` cannot be sent between threads safely
    let single_threaded = Rc::new(5);
    println!("Rc value (kept on this thread only): {single_threaded}");

    // `RefCell<T>` is `Send` (it can be moved to another thread
    // wholesale) but NOT `Sync` (it cannot be *shared* across
    // threads), because its borrow counter isn't atomic. Only one
    // thread may ever own it at a time.
    let cell = RefCell::new(0);
    println!("RefCell value (single owner only): {}", cell.borrow());

    println!("compile-time proof: this program only exists because every");
    println!("cross-thread value above satisfies Send/Sync - the Rc/RefCell");
    println!("counter-examples are left commented precisely because they");
    println!("would fail to compile, not fail at runtime.");
}
