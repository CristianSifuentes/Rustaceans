// Concept: Custom `Drop` semantics.
//
// Implementing `Drop::drop` lets a type intercept its own
// destruction, most commonly to release a resource that Rust's
// ownership model doesn't already know about - a raw allocation, a
// file descriptor, a lock, a metric counter. Drop order matters:
// fields are dropped in declaration order *after* the type's own
// `drop` body runs, and values go out of scope in reverse order of
// declaration within a block (RAII).

struct ResourcePool {
    name: &'static str,
    // Pretend this is something with real system cost: a socket, a
    // mmap'd region, an FFI handle - anything acquired in `new` and
    // requiring an explicit release.
    capacity: usize,
}

impl ResourcePool {
    fn new(name: &'static str, capacity: usize) -> Self {
        println!("acquire: {name} (capacity {capacity})");
        ResourcePool { name, capacity }
    }
}

impl Drop for ResourcePool {
    fn drop(&mut self) {
        // This runs automatically and deterministically the instant
        // the value goes out of scope, is explicitly `drop()`-ed, or
        // its owner (a Vec, a Box, ...) is itself dropped - no
        // garbage collector, no finalizer thread, no "eventually".
        println!("release: {} (freed {} units)", self.name, self.capacity);
    }
}

// A guard that logs entry/exit of a scope - the classic RAII pattern
// used for things like tracing spans or mutex guards.
struct ScopeGuard {
    label: &'static str,
}

impl ScopeGuard {
    fn enter(label: &'static str) -> Self {
        println!(">> entering {label}");
        ScopeGuard { label }
    }
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        println!("<< leaving {}", self.label);
    }
}

fn main() {
    let _outer = ScopeGuard::enter("main");

    {
        // Declared in this order...
        let first = ResourcePool::new("first", 10);
        let second = ResourcePool::new("second", 20);
        println!("using pools: {} + {}", first.name, second.name);
        // ...so they drop in *reverse* declaration order: "second"
        // is released before "first" when the block ends.
    }

    // `std::mem::drop` lets you force early, explicit destruction
    // instead of waiting for the end of scope - useful for releasing
    // a lock or file handle before doing more work in the same block.
    let early = ResourcePool::new("early", 5);
    println!("about to drop `early` on purpose");
    drop(early);
    println!("`early` is gone; continuing to do other work");

    // `_outer` drops last, right as `main` returns.
}
