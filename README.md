# Rustaceans

A hands-on learning log for the [Rust programming language](https://www.rust-lang.org/), following [Rust by Example](https://doc.rust-lang.org/rust-by-example/) and, at the advanced end, [The Rustonomicon](https://doc.rust-lang.org/nomicon/) and [Rust for Rustaceans](https://rust-for-rustaceans.com/). Each commit on this repo captures a single concept, building incrementally from "Hello, world!" toward advanced type-system, unsafe, and async internals.

> **Status:** Actively learning — this repo grows one concept (and one commit) at a time.

---

## Table of Contents

- [Overview](#overview)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Running the project](#running-the-project)
- [Concepts Covered](#concepts-covered)
  - [1. Hello, World!](#1-hello-world)
  - [2. Variables: `let` and `println!`](#2-variables-let-and-println)
  - [3. Mutability with `mut`](#3-mutability-with-mut)
  - [4. `String` vs `&str`](#4-string-vs-str)
  - [5. Numeric Types & Interpolation](#5-numeric-types--interpolation)
  - [6. Constants](#6-constants)
  - [7. Control Flow (`if` / `else if` / `else`)](#7-control-flow-if--else-if--else)
  - [8. Collections: `Vec<T>`](#8-collections-vect)
  - [9. Collections: `HashSet<T>`](#9-collections-hashsett)
  - [10. Collections: `HashMap<K, V>`](#10-collections-hashmapk-v)
  - [11. Iterating with `for` and `while`](#11-iterating-with-for-and-while)
  - [12. Functions](#12-functions)
  - [13. Structs, `impl`, and Lifetimes](#13-structs-impl-and-lifetimes)
- [Advanced Concepts](#advanced-concepts)
  - [14. Lifetime Subtyping & Variance](#14-lifetime-subtyping--variance)
  - [15. Higher-Rank Trait Bounds (HRTBs)](#15-higher-rank-trait-bounds-hrtbs)
  - [16. Dynamically Sized Types & `?Sized`](#16-dynamically-sized-types--sized)
  - [17. Interior Mutability](#17-interior-mutability)
  - [18. The `Pin` Type & Self-Referential Structs](#18-the-pin-type--self-referential-structs)
  - [19. Custom `Drop` Semantics](#19-custom-drop-semantics)
  - [20. Raw Pointers](#20-raw-pointers)
  - [21. The `UnsafeCell<T>` Core](#21-the-unsafecellt-core)
  - [22. Foreign Function Interface (FFI)](#22-foreign-function-interface-ffi)
  - [23. Associated Types vs. Generics](#23-associated-types-vs-generics)
  - [24. Fully Qualified Syntax](#24-fully-qualified-syntax)
  - [25. Procedural Macros](#25-procedural-macros)
  - [26. `Send` and `Sync`](#26-send-and-sync)
  - [27. Lock-Free Programming with Atomics](#27-lock-free-programming-with-atomics)
  - [28. Custom `Future` and `Poll` Logic](#28-custom-future-and-poll-logic)
- [Commit History Map](#commit-history-map)
- [Key Takeaways & Gotchas](#key-takeaways--gotchas)
- [Resources](#resources)
- [License](#license)

---

## Overview

This repository is a personal sandbox for learning Rust, split into two tiers:

- **Fundamentals** — ownership, types, mutability, control flow, core collections, loops, functions, structs, and lifetimes, layered incrementally on top of each other directly in [`hello/src/main.rs`](hello/src/main.rs).
- **Advanced concepts** — variance, higher-rank trait bounds, DSTs, interior mutability, `Pin`, `Drop`, raw pointers, `UnsafeCell`, FFI, associated types, fully qualified syntax, procedural macros, `Send`/`Sync`, lock-free atomics, and hand-rolled `Future`s — each isolated in its own runnable file under [`hello/examples/`](hello/examples/), plus a real proc-macro crate, [`hello_macros`](hello_macros/).

Every file is heavily commented explaining *why* something behaves the way it does (not just what it does), and every concept is one atomic, independently reviewable commit.

## Project Structure

```
Rustaceans/
├── LICENSE
├── README.md                          ← you are here
├── hello/                             ← the learning crate
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs                    ← fundamentals, layered incrementally
│   └── examples/                      ← one file per advanced concept
│       ├── 01_variance_lifetimes.rs
│       ├── 02_hrtbs.rs
│       ├── 03_dst_unsized.rs
│       ├── 04_interior_mutability.rs
│       ├── 05_pin_self_referential.rs
│       ├── 06_custom_drop.rs
│       ├── 07_raw_pointers.rs
│       ├── 08_unsafe_cell.rs
│       ├── 09_ffi.rs
│       ├── 10_associated_types_vs_generics.rs
│       ├── 11_fully_qualified_syntax.rs
│       ├── 13_procedural_macro.rs
│       ├── 14_send_sync.rs
│       ├── 15_atomics_lockfree.rs
│       └── 16_custom_future.rs
└── hello_macros/                      ← proc-macro crate: #[derive(Describe)]
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

## Getting Started

### Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) installed (edition `2024` — Rust 1.85+ recommended)

Verify your installation:

```bash
rustc --version
cargo --version
```

### Running the project

Clone the repo and run the `hello` binary (the fundamentals) via Cargo's workspace/package flags:

```bash
cargo run --package hello --bin hello
```

Or, from inside the crate directory:

```bash
cd hello
cargo run
```

Each advanced concept lives in its own runnable example — run any of them with `cargo run --example <name>` from inside `hello/`:

```bash
cd hello
cargo run --example 05_pin_self_referential
cargo run --example 15_atomics_lockfree
cargo run --example 16_custom_future
```

List every available example:

```bash
ls hello/examples/
```

---

## Concepts Covered

Each section below maps to concrete code in [`hello/src/main.rs`](hello/src/main.rs) and the commit that introduced it.

### 1. Hello, World!
*Commit: `30e11a1`, `3c9f68b`*

The canonical first program using the [`println!`](https://doc.rust-lang.org/std/macro.println.html) macro:

```rust
println!("Hello, world!");
```

### 2. Variables: `let` and `println!`
*Commit: `3c9f68b`*

Variables are declared with `let` and are **immutable by default** — a core Rust safety principle.

```rust
let my_variable: &str = "This is a string";
println!("Variable: {}", my_variable);
```

### 3. Mutability with `mut`
*Commit: `73399ea`*

To reassign a variable, it must be explicitly marked `mut`. Without it, the compiler rejects the reassignment with `E0384: cannot assign twice to immutable variable`.

```rust
let mut my_variable: &str = "This is a string";
my_variable = "Change";
```

### 4. `String` vs `&str`
*Commit: `44129a3`*

Two different ways to represent text in Rust:

| Type   | Description |
|--------|--------------|
| `String` | Owned, growable, heap-allocated buffer of UTF-8 bytes |
| `&str`   | Immutable, borrowed reference (slice) into UTF-8 bytes stored elsewhere |

```rust
let my_variable2: String = String::from("This is other string");
```

### 5. Numeric Types & Interpolation
*Commit: `295ee9c`, `30702f2`*

Rust is a strongly, statically typed language — numeric types must be explicit or inferred unambiguously, and mixing types (e.g. `f64` + `i32`) is a compile error.

```rust
let mut my_int: i32 = 7;
let my_int64: i64 = 7;
let my_float: f64 = 6.5;
let my_float2: f32 = 6.5;
let mut my_bool: bool = false;
```

Modern inline interpolation is used throughout via `{variable}` syntax:

```rust
println!("{my_int}");
```

> **Gotcha:** `my_float = my_float + my_int` fails to compile — Rust does **not** implicitly coerce between numeric types like `f64` and `i32`.

### 6. Constants
*Commit: `35d0ced`, `1551e5e`*

Declared with `const`, constants **require an explicit type** — unlike `let`, Rust will not infer it.

```rust
const MY_CONST: &str = "MY"; // type annotation is mandatory
println!("{MY_CONST}");
```

### 7. Control Flow (`if` / `else if` / `else`)
*Commit: `4ca58fc`*

Standard branching, combined with boolean logic (`&&`):

```rust
if my_int == 10 && my_bool {
    println!("10");
} else if my_int == 11 {
    println!("11");
} else {
    println!("no 10");
}
```

### 8. Collections: `Vec<T>`
*Commit: `7bfc2f6`*

A growable, heap-allocated array. Built with the `vec!` macro and extended with `.push(...)`.

```rust
let mut my_list: Vec<&str> = vec!["Angular", "React", "Astro"];
my_list.push("Python");
my_list.push("NET");
println!("{:?}", my_list);
println!("{}", my_list[0]); // indexed access
```

### 9. Collections: `HashSet<T>`
*Commit: `4456861`*

An unordered collection of unique values, built here from a `Vec` iterator via `.collect()`.

```rust
use std::collections::HashSet;

let mut my_hash: HashSet<&str> = vec!["Angular", "React", "Astro"].into_iter().collect();
my_hash.insert("Go");
println!("{:?}", my_hash);
```

### 10. Collections: `HashMap<K, V>`
*Commit: `7afd322`*

An unordered key-value store for rapid, out-of-order lookups — built the same way as the `HashSet` above, via `.collect()` on an iterator of tuples.

```rust
use std::collections::HashMap;

let mut my_map: HashMap<&str, i32> = vec![
    ("Cris", 36),
    ("Other", 56),
    ("Other3", 16),
].into_iter().collect();

my_map.insert("insert", 76);
println!("{:?}", my_map);
```

> **Gotcha:** like `HashSet`, a `HashMap` gives no ordering guarantee — iteration order can (and will) differ between runs.

### 11. Iterating with `for` and `while`
*Commit: `f1e1530`, `ba80f90`*

`for` loops consume an iterator; prefixing the collection with `&` borrows it instead of moving it, so it remains usable afterward.

```rust
// & borrows my_list — ownership stays with the caller, so my_list
// can still be used after the loop.
for value in &my_list {
    println!("list {value}");
}

// No & here: my_hash is moved into the loop and dropped when it ends.
for value in my_hash {
    println!("hash {value}");
}

for (key, value) in my_map {
    println!("key {key} value {value}");
}
```

`while` loops are used for manual, index-based iteration:

```rust
let mut my_counter: usize = 0;
while my_counter < my_list.len() {
    print!("{}", my_list[my_counter]);
    my_counter += 1;
}
```

> **Gotcha:** iterating `for value in my_hash` (without `&`) moves `my_hash` — trying to use it again afterward is a compile error (`E0382: use of moved value`).

### 12. Functions
*Commit: `69d569f`*

Free functions are declared with `fn` and called by name; no forward declaration is needed since Rust resolves items anywhere in scope.

```rust
fn my_function() {
    println!("This is a function");
}

my_function();
```

### 13. Structs, `impl`, and Lifetimes
*Commit: `8e696c1`, `f9a670d`*

A `struct` groups related data. When a field borrows data (like `&str`) instead of owning it, the compiler requires a **lifetime annotation** (`'a`) so it can guarantee the borrowed data outlives the struct — preventing a **dangling reference** (a pointer to memory that has already been freed or reassigned).

```rust
struct MyStruct<'a> {
    name: &'a str,
    age: i32,
}

impl<'a> MyStruct<'a> {
    fn new(name: &'a str, age: i32) -> MyStruct<'a> {
        MyStruct { name, age }
    }
}

let my_struct = MyStruct::new("Cris", 36);
println!("{} is {} years old", my_struct.name, my_struct.age);
```

> **Gotcha:** dropping the `<'a>` annotation gives `E0106: missing lifetime specifier`. The compiler cannot let a struct hold a `&str` without knowing how long that borrow is valid for.

---

## Advanced Concepts

Mastering Rust beyond ownership basics means understanding how the type system enforces safety at a low level, how `unsafe` code stays sound, how async execution is actually built, and how compile-time metaprogramming works. Each concept below is its own runnable file in [`hello/examples/`](hello/examples/) (run with `cargo run --example <name>` from inside `hello/`), introduced in one atomic commit.

### 14. Lifetime Subtyping & Variance
*Commit: `f469f03`* · [`01_variance_lifetimes.rs`](hello/examples/01_variance_lifetimes.rs)

Variance dictates how subtyping between lifetimes applies to compound types. `&'a T` is **covariant** over `'a` — a `&'static str` can stand in anywhere a shorter-lived `&'a str` is expected. `&'a mut T` is **invariant** over `T` — the compiler forbids substituting a short-lived reference into a slot that expects a longer-lived one, which is exactly what stops a dangling reference from being smuggled through a mutable borrow.

```rust
fn assign_through<'b>(dest: &mut &'b str, value: &'b str) {
    *dest = value;
}
```

### 15. Higher-Rank Trait Bounds (HRTBs)
*Commit: `bc32d18`* · [`02_hrtbs.rs`](hello/examples/02_hrtbs.rs)

`for<'a> Fn(&'a str)` expresses a bound that must hold for **every** possible lifetime, not one fixed lifetime chosen by the caller. Needed whenever a closure is handed a borrow the callee creates internally (e.g. a fresh `&str` on every loop iteration) — no single external lifetime could ever satisfy that.

```rust
fn process_lines<F>(text: &str, f: F)
where
    F: for<'a> Fn(&'a str) -> usize,
{ /* ... */ }
```

### 16. Dynamically Sized Types & `?Sized`
*Commit: `90b226f`* · [`03_dst_unsized.rs`](hello/examples/03_dst_unsized.rs)

`str`, `[T]`, and `dyn Trait` have no compile-time-known size — they only exist behind a fat pointer (data pointer + length/vtable). Every generic parameter implicitly requires `T: Sized`; accepting an unsized type means opting out with `T: ?Sized`.

```rust
fn describe<T: Display + ?Sized>(value: &T) {
    println!("describe: {value}");
}
```

### 17. Interior Mutability
*Commit: `4f9ef2b`* · [`04_interior_mutability.rs`](hello/examples/04_interior_mutability.rs)

Mutating data behind a shared `&T` reference by deferring the safety check to runtime instead of compile time. `Cell<T>` does no checking at all (copy in/out only); `RefCell<T>` panics on a conflicting borrow; `Mutex<T>`/`RwLock<T>` are the thread-safe versions, which block instead of panicking.

```rust
let shared_log: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
shared_log.borrow_mut().push("writer_a: started".to_string());
```

### 18. The `Pin` Type & Self-Referential Structs
*Commit: `197cb44`* · [`05_pin_self_referential.rs`](hello/examples/05_pin_self_referential.rs)

`Pin<P>` guarantees a value's memory address never changes once pinned — required for self-referential structs, including the compiler-generated state machines behind `async fn`. The example hand-builds a struct holding a raw pointer into its own field and pins it on the heap to keep that pointer valid.

```rust
struct SelfReferential {
    value: String,
    value_ptr: NonNull<String>, // points back into `value`
    _pinned: PhantomPinned,
}
```

### 19. Custom `Drop` Semantics
*Commit: `99b002a`* · [`06_custom_drop.rs`](hello/examples/06_custom_drop.rs)

Implementing `Drop` intercepts value destruction — releasing external resources, running cleanup logic, or just observing RAII in action. Values drop deterministically, in reverse declaration order, the instant they go out of scope (or are dropped explicitly with `drop(value)`).

```rust
impl Drop for ResourcePool {
    fn drop(&mut self) {
        println!("release: {} (freed {} units)", self.name, self.capacity);
    }
}
```

### 20. Raw Pointers
*Commit: `8347376`* · [`07_raw_pointers.rs`](hello/examples/07_raw_pointers.rs)

`*const T` and `*mut T` bypass the borrow checker entirely: no lifetimes, nullable, potentially dangling. Creating one is safe; dereferencing one requires an `unsafe` block and a hand-written safety justification.

```rust
let mut_ptr: *mut i32 = &mut value;
unsafe { *mut_ptr += 1; }
```

### 21. The `UnsafeCell<T>` Core
*Commit: `8423c15`* · [`08_unsafe_cell.rs`](hello/examples/08_unsafe_cell.rs)

The one language construct allowed to turn a shared `&T` into a `*mut T` — the structural foundation every interior-mutability type in `std` (`Cell`, `RefCell`, `Mutex`, atomics) is built on top of. The example hand-rolls a minimal `Cell`-alike to make that mechanism explicit.

```rust
fn set(&self, new_value: T) {
    unsafe { *self.value.get() = new_value; }
}
```

### 22. Foreign Function Interface (FFI)
*Commit: `17d0cb7`* · [`09_ffi.rs`](hello/examples/09_ffi.rs)

Interfacing with C: `extern "C"` pins a function to the C calling convention, `#[repr(C)]` pins a struct's memory layout to match what a C compiler would emit, and every cross-boundary call is `unsafe` because the compiler can no longer verify the other side's contract.

```rust
unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

#[repr(C)]
pub struct Point { x: f64, y: f64 }
```

### 23. Associated Types vs. Generics
*Commit: `3d44f82`* · [`10_associated_types_vs_generics.rs`](hello/examples/10_associated_types_vs_generics.rs)

Associated types (`trait Graph { type Node; }`) are trait *outputs* — a type can implement the trait exactly once. Generic parameters (`trait Converter<T>`) are trait *inputs* — a single type can implement the trait multiple times, once per `T`.

```rust
impl Converter<f64> for Measurement { /* meters */ }
impl Converter<String> for Measurement { /* label */ }
```

### 24. Fully Qualified Syntax
*Commit: `0a8780b`* · [`11_fully_qualified_syntax.rs`](hello/examples/11_fully_qualified_syntax.rs)

Resolves ambiguity when multiple traits define a method with an identical name on the same type — `value.method()` can't pick one, so you call through the trait explicitly.

```rust
println!("{}", <Duck as Fly>::launch(&duck));
println!("{}", <Duck as Swim>::launch(&duck));
```

### 25. Procedural Macros
*Commits: `bd18228`, `7b6b676`* · [`hello_macros/src/lib.rs`](hello_macros/src/lib.rs), [`13_procedural_macro.rs`](hello/examples/13_procedural_macro.rs)

Compiler plugins that manipulate token streams at compile time. [`hello_macros`](hello_macros/) is a real `proc-macro = true` crate implementing `#[derive(Describe)]`: it parses a struct's fields with `syn` and generates an `impl Describe` block with `quote!` — the same mechanism built-in derives like `#[derive(Debug)]` use.

```rust
#[derive(Describe)]
struct Package { name: &'static str, version: &'static str, downloads: u64 }

println!("{}", package.describe()); // generated entirely at compile time
```

### 26. `Send` and `Sync`
*Commit: `ee64a4f`* · [`14_send_sync.rs`](hello/examples/14_send_sync.rs)

Auto traits governing thread safety: `Send` means ownership can cross a thread boundary; `Sync` means `&T` can be shared across threads. `Arc<Mutex<T>>` satisfies both; `Rc<T>` (non-atomic refcount) and `RefCell<T>` (unsynchronized borrow counter) deliberately do not — enforced entirely at compile time.

```rust
assert_send::<Arc<Mutex<i32>>>();
assert_sync::<Arc<Mutex<i32>>>();
```

### 27. Lock-Free Programming with Atomics
*Commit: `1c853a1`* · [`15_atomics_lockfree.rs`](hello/examples/15_atomics_lockfree.rs)

Trading a blocking `Mutex` for hardware atomic instructions. Requires picking a memory `Ordering` explicitly: `Relaxed` (atomicity only), `Acquire`/`Release` (publishes other memory writes across threads via a happens-before edge), and `SeqCst` (a single global order, strongest and most expensive).

```rust
writer_ready.store(true, Ordering::Release);
while !ready.load(Ordering::Acquire) { thread::yield_now(); }
```

### 28. Custom `Future` and `Poll` Logic
*Commit: `234e542`* · [`16_custom_future.rs`](hello/examples/16_custom_future.rs)

`async fn` is sugar over a hand-writable `Future` trait. The example builds a `Delay` future backed by a background thread, a `Task` that bridges `std::task::Wake` to a ready-queue, and a minimal single-threaded `MiniExecutor` that polls tasks and relies entirely on `Waker::wake()` to know when a `Pending` future can make progress again — no async runtime dependency.

```rust
fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    if self.done.load(Ordering::Acquire) { return Poll::Ready("delay complete"); }
    // ...spawn background work, stash cx.waker().clone(), return Pending...
}
```

---

## Commit History Map

A chronological view of how this repo evolved, concept by concept:

| Commit | Concept |
|--------|---------|
| `8adee1d` | Initial commit |
| `750991b` | Add greeting 'hola' to README |
| `30e11a1` | Initial commit (crate scaffold) |
| `3c9f68b` | `let` bindings and `println!` |
| `73399ea` | `mut` for changing strings |
| `44129a3` | `String` vs `&str` |
| `295ee9c` | `i32` and string interpolation |
| `30702f2` | Primitive types and interpolation |
| `1551e5e` | Constants: type inference limitations |
| `35d0ced` | Working with `const` |
| `4ca58fc` | Control flow: `if` / `else if` / `else` |
| `7bfc2f6` | Working with `Vec<T>` |
| `4456861` | `HashSet` and `Vec` basic operations |
| `7afd322` | Working with `HashMap` and `insert` |
| `f1e1530` | Working with `for` and list |
| `ba80f90` | Working with `&` and loops using `Vec`, `HashSet`, `HashMap` |
| `69d569f` | Getting context for `fn` in Rust |
| `8e696c1` | Working with `struct` and `'a` |
| `f9a670d` | Comments on dangling pointers and the `impl` keyword |
| `f469f03` | Lifetime subtyping & variance |
| `bc32d18` | Higher-rank trait bounds (HRTB) |
| `90b226f` | Dynamically sized types & `?Sized` |
| `4f9ef2b` | Interior mutability (`Cell`/`RefCell`/`Mutex`/`RwLock`) |
| `197cb44` | `Pin` and a hand-built self-referential struct |
| `99b002a` | Custom `Drop` semantics |
| `8347376` | Raw pointers (`*const T` / `*mut T`) |
| `8423c15` | `UnsafeCell<T>`, the foundation of interior mutability |
| `17d0cb7` | FFI (C ABI interop) |
| `3d44f82` | Associated types vs. generics |
| `0a8780b` | Fully qualified syntax |
| `bd18228` | `hello_macros`: a `#[derive(Describe)]` procedural macro crate |
| `7b6b676` | Procedural macro usage example |
| `ee64a4f` | `Send`/`Sync` auto trait example |
| `1c853a1` | Lock-free atomics (`Relaxed`/`Acquire-Release`/`SeqCst`) |
| `234e542` | Custom `Future`/`Poll` with a hand-rolled executor |

## Key Takeaways & Gotchas

- **Immutability is the default.** Use `mut` deliberately — it documents intent.
- **`const` requires an explicit type annotation**; `let` can often infer it.
- **No implicit numeric coercion.** `i32` + `f64` won't compile — convert explicitly (e.g. `as f64`).
- **`String` vs `&str`** is really an ownership question: owned & growable vs. borrowed & fixed.
- **`{:?}`** (debug formatting) is essential for printing collections like `Vec`, `HashSet`, and `HashMap`.
- **`HashSet`/`HashMap` have no guaranteed order** — don't rely on print output ordering.
- **`&` in a `for` loop borrows instead of moves.** Iterating a collection by value consumes it; iterating `&collection` keeps it usable afterward.
- **A struct holding a borrowed field needs a lifetime (`'a`).** It's how the compiler proves the borrowed data can't outlive the struct that references it, ruling out dangling references at compile time.
- **`&T` is covariant, `&mut T` is invariant.** Invariance on `&mut T` is what stops a short-lived reference from being smuggled into a slot that expects a longer-lived one.
- **`for<'a>` bounds are needed when a closure borrows something the callee creates internally** — no single fixed lifetime chosen by the caller could ever be valid for every call.
- **Interior mutability trades a compile-time check for a runtime one.** `RefCell` panics on conflict; `Mutex`/`RwLock` block instead — pick based on whether you're single- or multi-threaded.
- **`UnsafeCell<T>` is the *only* legal way to get a `*mut T` from a `&T`.** Every safe interior-mutability wrapper in `std` is a safety protocol layered on top of it.
- **`Send` and `Sync` are compile-time, not runtime, guarantees.** `Rc<T>`/`RefCell<T>` failing to compile inside `thread::spawn` is the type system ruling out a data race before the program ever runs.
- **Atomic memory ordering is a real design decision, not boilerplate.** `Relaxed` for independent counters, `Acquire`/`Release` to publish other memory across threads, `SeqCst` when you need one global order everyone agrees on.
- **A `Future` does nothing until polled**, and returning `Pending` is a promise: something must call `Waker::wake()` later, or the task never runs again.

## Resources

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — the primary guide for the fundamentals in this repo
- [The Rust Programming Language (the book)](https://doc.rust-lang.org/book/)
- [Rust Standard Library docs](https://doc.rust-lang.org/std/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — the authoritative guide to `unsafe` Rust and low-level memory layout guarantees, behind the [Advanced Concepts](#advanced-concepts) section
- [Rust for Rustaceans](https://rust-for-rustaceans.com/) — Jon Gjengset's book on idiomatic library API design for intermediate+ Rust developers

## License

See [LICENSE](LICENSE) for details.
