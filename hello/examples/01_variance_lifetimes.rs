// Concept: Lifetime subtyping & variance.
//
// A type `F<'a>` is *covariant* over 'a if `F<'long>` is a subtype of
// `F<'short>` whenever 'long: 'short (i.e. you can always use a
// longer-lived value where a shorter-lived one is expected).
//
// `&'a T` is covariant over both 'a and T: a `&'long T` can be used
// anywhere a `&'short T` is expected. `&'a mut T`, however, is
// *invariant* over T: the compiler must forbid substituting a
// short-lived reference into a slot that expects a long-lived one,
// because that would let you stash a dangling reference and read it
// back out after the short lifetime ends.

// Covariance in action: a &'static str can stand in for any &'a str.
fn takes_short_lived<'a>(s: &'a str) -> &'a str {
    s
}

fn covariance_demo() {
    let static_str: &'static str = "lives forever";
    // 'static outlives every 'a, so this coercion is always sound.
    let shortened: &str = takes_short_lived(static_str);
    println!("covariance: {shortened}");
}

// Invariance in action: &mut T does NOT let you shorten the lifetime
// hidden inside T, because the caller could later write a long-lived
// value through the reference and the borrow checker would lose track
// of how long that value actually needs to live.
fn assign_through<'b>(dest: &mut &'b str, value: &'b str) {
    *dest = value;
}

fn invariance_demo() {
    let long_lived = String::from("outer");
    let mut target: &str = &long_lived;
    {
        let short_lived = String::from("inner");
        // `target` has type `&'long str`; `assign_through` requires
        // both arguments to share exactly one lifetime, so the borrow
        // checker forces 'b down to the shorter scope here — it cannot
        // silently treat &'short str as &'long str behind the &mut.
        assign_through(&mut target, &short_lived);
        println!("invariance (still in scope): {target}");
    }
    // Using `target` beyond this point would be a compile error if it
    // still referred to `short_lived`'s data - the invariance of
    // `&mut &'b str` over 'b is exactly what makes that dangling use
    // impossible to express in the first place.
}

fn main() {
    covariance_demo();
    invariance_demo();
}
