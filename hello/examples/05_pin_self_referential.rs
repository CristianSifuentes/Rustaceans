// Concept: `Pin<P>` and self-referential structs.
//
// Rust's move semantics normally let any value be freely relocated in
// memory (a `mem::swap`, a `Vec` reallocating, a value being returned
// out of a function). That is unsound for a *self-referential* type -
// one that holds a pointer into its own fields - because moving the
// struct invalidates that internal pointer without anyone updating it.
//
// `Pin<P>` is a wrapper around a pointer that promises the pointee
// will never move again (for types that don't implement `Unpin`).
// This is exactly the guarantee the compiler-generated state machines
// behind `async fn` need, since an `.await` point can suspend the
// function with a live reference from one local variable into
// another local variable of the *same* generator.
//
// This example builds a minimal self-referential struct by hand and
// pins it, to make the mechanism concrete without the sugar of `async`.

use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

struct SelfReferential {
    value: String,
    // Points back into `value`. Once populated, `SelfReferential`
    // must never move, or `value_ptr` becomes a dangling pointer.
    value_ptr: NonNull<String>,
    // Opts this type out of `Unpin`, so a plain `Pin<&mut Self>`
    // cannot be moved out of via safe code (e.g. `mem::take`,
    // `mem::swap`) - only `unsafe` code that upholds the pin
    // contract is allowed to touch the pointee afterwards.
    _pinned: PhantomPinned,
}

impl SelfReferential {
    // Returns a *pinned box*: heap allocation gives the value a
    // stable address for its whole lifetime, and `Box::into_pin`
    // then asserts the pin contract on top of that.
    fn new(text: &str) -> Pin<Box<Self>> {
        let unpinned = Box::new(SelfReferential {
            value: String::from(text),
            value_ptr: NonNull::dangling(),
            _pinned: PhantomPinned,
        });
        let mut pinned = Box::into_pin(unpinned);

        // SAFETY: we only obtain a raw pointer here, we never move
        // the pointee out from under the `Pin`. Taking a mutable
        // reference to fill in `value_ptr` is sound because the
        // struct's address is now fixed for the rest of its life.
        let self_ptr: NonNull<String> = NonNull::from(&pinned.value);
        unsafe {
            let mut_ref: Pin<&mut Self> = pinned.as_mut();
            Pin::get_unchecked_mut(mut_ref).value_ptr = self_ptr;
        }
        pinned
    }

    fn value(self: Pin<&Self>) -> &str {
        // `Pin::get_ref` consumes the `Pin<&'a Self>` by value and
        // hands back a plain `&'a Self`, so the returned `&str` is
        // tied to the original borrow's lifetime rather than to a
        // fresh reborrow of the local `self` binding.
        &Pin::get_ref(self).value
    }

    // Dereferencing the internal self-pointer, proving it still
    // points at valid, unmoved data.
    fn value_via_self_pointer(self: Pin<&Self>) -> &str {
        // SAFETY: `value_ptr` was derived from `value` while pinned,
        // and the pin contract guarantees `self` has not moved since.
        unsafe { self.value_ptr.as_ref() }
    }
}

fn main() {
    let pinned = SelfReferential::new("pin me down");

    println!("direct field:      {}", pinned.as_ref().value());
    println!("via self-pointer:  {}", pinned.as_ref().value_via_self_pointer());

    // `pinned` is a `Pin<Box<SelfReferential>>` - it can be moved
    // itself (the Box pointer is relocated), but the *pointee* on the
    // heap never is, so `value_ptr` stays valid the whole time.
    let relocated = pinned;
    println!("after relocating the Pin<Box<..>>: {}", relocated.as_ref().value_via_self_pointer());
}
