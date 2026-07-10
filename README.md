# Rustaceans

A hands-on learning log for the [Rust programming language](https://www.rust-lang.org/), following [Rust by Example](https://doc.rust-lang.org/rust-by-example/). Each commit on this repo captures a single concept, building incrementally from "Hello, world!" toward more complete Rust programs.

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
- [Commit History Map](#commit-history-map)
- [Key Takeaways & Gotchas](#key-takeaways--gotchas)
- [Resources](#resources)
- [License](#license)

---

## Overview

This repository is a personal sandbox for learning Rust fundamentals: ownership, types, mutability, control flow, and core collections. Code lives in a single evolving binary crate (`hello`), and each concept is layered on top of the previous one directly in `main.rs`, with comments explaining *why* something behaves the way it does (not just what it does).

## Project Structure

```
Rustaceans/
├── LICENSE
├── README.md              ← you are here
└── hello/                 ← the learning crate
    ├── Cargo.toml
    └── src/
        └── main.rs        ← all concepts explored so far
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

Clone the repo and run the `hello` binary via Cargo's workspace/package flags:

```bash
cargo run --package hello --bin hello
```

Or, from inside the crate directory:

```bash
cd hello
cargo run
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

## Key Takeaways & Gotchas

- **Immutability is the default.** Use `mut` deliberately — it documents intent.
- **`const` requires an explicit type annotation**; `let` can often infer it.
- **No implicit numeric coercion.** `i32` + `f64` won't compile — convert explicitly (e.g. `as f64`).
- **`String` vs `&str`** is really an ownership question: owned & growable vs. borrowed & fixed.
- **`{:?}`** (debug formatting) is essential for printing collections like `Vec` and `HashSet`.
- **`HashSet` has no guaranteed order** — don't rely on print output ordering.

## Resources

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — the primary guide followed in this repo
- [The Rust Programming Language (the book)](https://doc.rust-lang.org/book/)
- [Rust Standard Library docs](https://doc.rust-lang.org/std/)

## License

See [LICENSE](LICENSE) for details.
