// Concept: `UnsafeCell<T>` - the foundation of interior mutability.
//
// The Rust compiler assumes that data reached through a `&T` is never
// mutated for the lifetime of that borrow, and the optimizer is
// allowed to exploit that assumption (e.g. caching a read across
// calls). `UnsafeCell<T>` is the *only* language-level construct that
// opts a piece of memory out of that assumption: it is the sole way
// to legally turn a `&UnsafeCell<T>` into a `*mut T` and write
// through it. Every interior-mutability type in std - `Cell`,
// `RefCell`, `Mutex`, `RwLock`, `AtomicUsize` internally - is built on
// top of an `UnsafeCell<T>` doing this exact trick, plus its own
// safety protocol layered on top (runtime borrow counters, OS locks,
// hardware atomics) to make the raw capability sound to use safely.
//
// This example builds a tiny, deliberately simplified `Cell`-alike by
// hand to make that foundation visible.

use std::cell::UnsafeCell;

struct MyCell<T> {
    value: UnsafeCell<T>,
}

// SAFETY-relevant: real `Cell<T>` is intentionally `!Sync` because it
// has no synchronization at all - sharing it across threads would be
// a data race. We inherit that: `UnsafeCell<T>` is `!Sync` by
// default, so `MyCell<T>` is automatically `!Sync` too unless we
// unsafe-impl `Sync` ourselves (which we deliberately do not).
impl<T: Copy> MyCell<T> {
    fn new(value: T) -> Self {
        MyCell {
            value: UnsafeCell::new(value),
        }
    }

    fn get(&self) -> T {
        // SAFETY: `T: Copy`, so this reads a snapshot of the value
        // rather than handing out a reference. No reference to the
        // interior ever escapes this function, so there is no risk
        // of it aliasing a `set()` call happening concurrently on a
        // *different* thread - which is exactly why `MyCell` staying
        // `!Sync` (single-threaded only) is load-bearing here.
        unsafe { *self.value.get() }
    }

    fn set(&self, new_value: T) {
        // SAFETY: `UnsafeCell::get` returns a `*mut T` from a shared
        // `&self` - this is the one operation only `UnsafeCell`
        // itself is allowed to perform. Writing through it is sound
        // here because `MyCell` guarantees (by being `!Sync`) that no
        // other thread can be reading `value` concurrently, and
        // within one thread `get`/`set` never overlap in time since
        // Rust has no re-entrant aliasing without unsafe code.
        unsafe {
            *self.value.get() = new_value;
        }
    }
}

fn main() {
    // Mutation through a shared `&MyCell<i32>` - impossible with a
    // plain `&i32`, and only sound here because `UnsafeCell` is the
    // one type the compiler trusts not to apply its no-aliasing
    // optimizations to.
    let cell = MyCell::new(10);
    let shared_ref = &cell;

    shared_ref.set(shared_ref.get() + 5);
    shared_ref.set(shared_ref.get() * 2);
    println!("MyCell value after mutation through &: {}", cell.get());

    // Multiple shared references, all able to mutate - exactly the
    // "shared mutability" that a plain `&T` can never offer.
    let alias_one = &cell;
    let alias_two = &cell;
    alias_one.set(100);
    println!("seen through alias_two: {}", alias_two.get());
}
