// Concept: Dynamically Sized Types (DSTs) & `?Sized`.
//
// Most types have a size known at compile time, and Rust silently
// adds an implicit `T: Sized` bound to every generic parameter. A
// handful of types don't fit that mold - `str` (not `&str`) and slice
// types like `[u8]` have no fixed size, and trait objects like
// `dyn Trait` erase the concrete type entirely. These can only ever
// be handled through a pointer (`&str`, `&[u8]`, `&dyn Trait`, `Box<dyn
// Trait>`...), because the pointer itself carries the extra metadata
// (a length, or a vtable pointer) that makes an unsized value usable.
//
// Writing a generic function that should also accept these unsized
// types requires opting out of the implicit `Sized` bound with
// `T: ?Sized` ("T may or may not be sized").

use std::fmt::Display;

// Without `?Sized`, `T` would implicitly require `T: Sized`, and this
// function could never be called with `&str` or `&dyn Display`
// because `str` and `dyn Display` are themselves unsized - only
// references/pointers to them are.
fn describe<T: Display + ?Sized>(value: &T) {
    println!("describe: {value}");
}

// A struct holding a pointer to an unsized field must also opt out of
// `Sized` for that field's type parameter. Only the *last* field of a
// struct is allowed to be unsized, and only through a pointer/wrapper.
struct Wrapper<T: ?Sized> {
    label: &'static str,
    inner: T,
}

fn main() {
    // `str` and `[i32]` are DSTs: they cannot live on the stack by
    // value, only behind a reference that also stores their length.
    let owned = String::from("dynamically sized");
    let slice: &str = &owned; // fat pointer: (data ptr, length)
    describe(slice);

    let numbers = [1, 2, 3, 4];
    let unsized_slice: &[i32] = &numbers;
    println!("slice len via fat pointer: {}", unsized_slice.len());

    // `dyn Display` is unsized too - `describe` accepts it only
    // because of the `?Sized` bound.
    let boxed: Box<dyn Display> = Box::new(42u32);
    describe(&*boxed);

    // A `Wrapper<dyn Display>` can only exist behind a pointer: the
    // fat pointer stores a vtable pointer alongside the data pointer.
    // The unsizing coercion (Sized field -> `dyn Display` field) is
    // applied at the moment of construction, right here in `Box::new`.
    let boxed_wrapper: Box<Wrapper<dyn Display>> = Box::new(Wrapper {
        label: "boxed unsized field",
        inner: 42u32,
    });
    println!("{}: {}", boxed_wrapper.label, &boxed_wrapper.inner);
}
