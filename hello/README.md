# 👋 hello — Rust Fundamentals Playground

A small, heavily-commented Rust program used to learn and demonstrate core language
concepts: variables, mutability, primitive types, constants, control flow, and two
common collections (`Vec` and `HashSet`).

Source: [`src/main.rs`](./src/main.rs)

## Table of Contents

- [Requirements](#requirements)
- [Running the Project](#running-the-project)
- [Concepts Covered](#concepts-covered)
  - [1. Comments](#1-comments)
  - [2. The `println!` Macro](#2-the-println-macro)
  - [3. Variables & Mutability](#3-variables--mutability)
  - [4. `String` vs `&str`](#4-string-vs-str)
  - [5. Integer Types](#5-integer-types)
  - [6. Floating-Point Types](#6-floating-point-types)
  - [7. Booleans](#7-booleans)
  - [8. Constants](#8-constants)
  - [9. Control Flow (`if` / `else if` / `else`)](#9-control-flow-if--else-if--else)
  - [10. Collections: `Vec<T>`](#10-collections-vect)
  - [11. Collections: `HashSet<T>`](#11-collections-hashsett)
- [Formatting Cheat Sheet](#formatting-cheat-sheet)
- [Further Reading](#further-reading)

---

## Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install) (`rustc`, `cargo`)

## Running the Project

From the repository root:

```bash
cargo run --package hello --bin hello
```

Or from inside `hello/`:

```bash
cargo run
```

---

## Concepts Covered

### 1. Comments

```rust
// This is main point entry
/* This is comment */
```

Rust supports line comments (`//`) and block comments (`/* ... */`). Comments are
ignored by the compiler and used purely for documentation.

### 2. The `println!` Macro

```rust
println!("Hello, world!");
println!("Variable: {}", my_variable);
println!("{my_variable}");
```

`println!` is a macro (note the `!`) that prints formatted text to standard output
followed by a newline. Values can be interpolated either positionally with `{}` and
a trailing argument, or directly by name using **captured identifiers** (`{my_variable}`),
available since Rust 2021.

> ⚠️ A bare `println!({value})` (without quotes) is invalid — the first argument
> must always be a string literal format template, e.g. `println!("{value}")`.

### 3. Variables & Mutability

```rust
let mut my_variable: &str = "This is a string";
my_variable = "Change";
```

Variables are **immutable by default** in Rust. Adding `mut` allows the value bound
to the variable to be reassigned. Attempting to reassign a variable without `mut`
produces a compile-time error:

```
cannot assign twice to immutable variable `my_variable`
```

This is a core safety feature — immutability is the default, and mutability must be
explicit.

### 4. `String` vs `&str`

```rust
let my_variable2: String = String::from("This is other string");
```

| Type   | Ownership | Mutability   | Storage        |
|--------|-----------|--------------|----------------|
| `String` | Owned    | Growable      | Heap-allocated, UTF-8 |
| `&str`   | Borrowed | Immutable view | Points to UTF-8 bytes owned elsewhere |

- `String` is an owned, growable buffer — use it when you need to build or own text.
- `&str` (string slice) is a borrowed, immutable reference to string data — use it
  for read-only views (e.g. string literals, or slices of a `String`).

### 5. Integer Types

```rust
let mut my_int: i32 = 7;
my_int = my_int + 4;
my_int = 10;

let my_int64: i64 = 7;
```

Rust has explicitly-sized integer types: `i8`/`i16`/`i32`/`i64`/`i128` (signed) and
`u8`/`u16`/`u32`/`u64`/`u128` (unsigned). `i32` is the default inference target when
no other type is implied. Arithmetic (`+`, `-`, etc.) works as expected and can be
reassigned when the binding is `mut`.

### 6. Floating-Point Types

```rust
let my_float: f64 = 6.5;   // double precision (default)
let my_float2: f32 = 6.5;  // single precision
```

Rust supports `f32` and `f64` for floating-point numbers, with `f64` being the
default. Mixing types (e.g. adding an `f64` to an `i32`) is a compile error — Rust
requires explicit conversions between numeric types, and values must also be
declared `mut` before they can be reassigned:

```
cannot mutate immutable variable `my_float`  // E0384
```

### 7. Booleans

```rust
let mut my_bool: bool = false;
my_bool = true;
```

`bool` has exactly two values: `true` and `false`. Like any other type, it's
immutable unless declared with `mut`.

### 8. Constants

```rust
const MY_CONST: &str = "MY";
```

`const` declares a compile-time constant that:
- Is **always immutable** (no `mut` allowed, ever).
- **Must** have an explicit type annotation — unlike `let`, its type is never inferred.
- Can be declared in any scope, including global scope.

### 9. Control Flow (`if` / `else if` / `else`)

```rust
if my_int == 10 && my_bool {
    println!("10");
} else if my_int == 11 {
    println!("11");
} else {
    println!("no 10");
}
```

`if` in Rust is an expression and does not require parentheses around the
condition, but curly braces are mandatory. Logical operators (`&&`, `||`, `==`)
combine boolean conditions just like in most C-family languages.

### 10. Collections: `Vec<T>`

```rust
let mut my_list: Vec<&str> = vec!["Angular", "React", "Astro"];
my_list.push("Python");
my_list.push("NET");
my_list.push("Go");
my_list.push("SQL");

println!("{:?}", my_list);   // debug-print the whole vector
println!("{}", my_list[0]);  // index access
```

`Vec<T>` is a growable, heap-allocated list. The `vec!` macro creates one with
initial elements, `.push()` appends to it, and indexing (`my_list[0]`) accesses
elements by position (panics if out of bounds).

### 11. Collections: `HashSet<T>`

```rust
use std::collections::HashSet;

let mut my_hash: HashSet<&str> = vec!["Angular", "React", "Astro"].into_iter().collect();
my_hash.insert("Go");

println!("{:?}", my_hash);
```

`HashSet<T>` stores unique, unordered values. Here a `Vec` is turned into a
`HashSet` via `.into_iter().collect()`, which also de-duplicates any repeated
values. `.insert()` adds a new element (a no-op if it already exists). Because
sets are unordered, printed output order is not guaranteed.

---

## Formatting Cheat Sheet

| Syntax             | Meaning                                   |
|--------------------|--------------------------------------------|
| `{}`               | `Display` formatting (positional argument) |
| `{name}`           | Captured identifier formatting (Rust 2021+) |
| `{:?}`             | `Debug` formatting (needed for `Vec`, `HashSet`, etc.) |

---

## Further Reading

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rust Book — Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [The Rust Book — Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [The Rust Book — Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
