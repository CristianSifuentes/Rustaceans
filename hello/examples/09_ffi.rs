// Concept: Foreign Function Interface (FFI).
//
// FFI lets Rust call into (and be called from) code compiled with a
// different toolchain - typically C, since the "C ABI" is the
// closest thing to a universal calling convention every platform and
// language agrees on. Two things have to line up across the
// language boundary that Rust normally guarantees for you and C
// does not check at all:
//   1. Calling convention - which registers/stack slots hold
//      arguments and the return value. `extern "C"` on a function
//      pins it to that convention instead of Rust's own (unstable,
//      free to change) ABI.
//   2. Memory layout - `#[repr(C)]` pins a struct's field order,
//      size, and alignment to what a C compiler would produce,
//      since Rust's default struct layout is also unspecified and
//      may reorder fields for better packing.
//
// Every call across the boundary is `unsafe`, because the compiler
// can no longer verify the callee's side of the contract - a wrong
// signature here is silent undefined behavior, not a type error.

use std::os::raw::{c_char, c_int};

// Declares C standard library functions Rust links against
// automatically on every platform (libc is always present).
unsafe extern "C" {
    fn abs(n: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

// `#[repr(C)]` guarantees this struct's layout matches what a C
// compiler emits for the equivalent `struct { double x; double y; }`
// - field order preserved, standard alignment, no reordering for
// packing. Without it, Rust would be free to lay `Point` out however
// it likes, which would be silently wrong if handed to C code.
#[repr(C)]
pub struct Point {
    x: f64,
    y: f64,
}

// Exposes a Rust function to C callers: `extern "C"` fixes the
// calling convention, and `#[unsafe(no_mangle)]` keeps the symbol
// name `rust_distance` instead of the compiler's mangled name, so a
// C linker can find it by that exact name.
#[unsafe(no_mangle)]
pub extern "C" fn rust_distance(a: Point, b: Point) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

fn main() {
    unsafe {
        // SAFETY: `abs` is a pure function over `c_int` with no
        // pointer arguments - the signature above matches libc's
        // declaration exactly, so this call cannot violate memory
        // safety.
        println!("C abs(-42) = {}", abs(-42));

        // SAFETY: the pointer passed to `strlen` comes from a Rust
        // `CString`, which guarantees a single trailing NUL and no
        // interior NUL bytes - exactly what `strlen` requires.
        let text = std::ffi::CString::new("ffi across the boundary").unwrap();
        let length = strlen(text.as_ptr());
        println!("C strlen(...) = {length}");
    }

    // Calling our own `extern "C"` function directly (normally a C
    // caller would do this, but the ABI contract is identical either
    // way, so calling it from Rust exercises the same layout rules).
    let origin = Point { x: 0.0, y: 0.0 };
    let target = Point { x: 3.0, y: 4.0 };
    println!("rust_distance(origin, target) = {}", rust_distance(origin, target));
}
