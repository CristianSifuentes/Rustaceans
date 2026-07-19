// Concept: Raw pointers (`*const T` / `*mut T`).
//
// Raw pointers are Rust's escape hatch from the borrow checker: they
// carry no lifetime, no aliasing guarantees, and are allowed to be
// null or dangling. Creating one is always safe (it's just an
// address), but *dereferencing* one requires an `unsafe` block,
// because the compiler can no longer prove the access is valid - that
// burden shifts entirely onto the programmer.

fn main() {
    let mut value = 42i32;

    // Safe to create: taking a raw pointer's address doesn't touch
    // memory. `&raw const` / `&raw mut` are unsafe-free even though
    // the pointer they produce is not safe to use unchecked.
    let const_ptr: *const i32 = &value;
    let mut_ptr: *mut i32 = &mut value;

    unsafe {
        // SAFETY: `const_ptr` was derived from a live `&i32` a moment
        // ago and nothing has invalidated it since.
        println!("read through *const i32: {}", *const_ptr);

        // SAFETY: `mut_ptr` is the sole live pointer to `value` at
        // this point (the borrow checker isn't tracking it, but we
        // are, by hand).
        *mut_ptr += 1;
        println!("wrote through *mut i32: {}", *mut_ptr);
    }
    println!("value after unsafe write: {value}");

    // Raw pointers can be null - something `&T` can never be.
    let null_ptr: *const i32 = std::ptr::null();
    println!("is_null: {}", null_ptr.is_null());

    // Pointer arithmetic on a raw pointer into an array/slice: this
    // is how indexing is implemented under the hood, made explicit.
    let numbers = [10, 20, 30, 40, 50];
    let base_ptr: *const i32 = numbers.as_ptr();
    unsafe {
        for i in 0..numbers.len() {
            // SAFETY: `i` never exceeds `numbers.len()`, so `offset`
            // stays within the bounds of the original allocation.
            let element_ptr = base_ptr.add(i);
            print!("{} ", *element_ptr);
        }
        println!();
    }

    // A dangling raw pointer is easy to construct and completely
    // inert until dereferenced - proof that *creating* one is safe,
    // only *using* one is not.
    let dangling: *const i32 = {
        let temp = 99;
        &temp as *const i32
        // `temp` is dropped here; `dangling` now points at freed
        // stack space. We deliberately do NOT dereference it.
    };
    println!("dangling pointer captured (not dereferenced): {dangling:p}");
}
