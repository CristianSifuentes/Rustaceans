// Concept: Higher-Rank Trait Bounds (HRTBs).
//
// A plain generic bound like `F: Fn(&'a str)` fixes 'a to a single,
// specific lifetime chosen by the *caller* at the call site. That is
// too restrictive for a function that wants to hand the closure a
// borrow it creates internally (e.g. a slice of a locally-owned
// String) - no single external 'a could ever be valid for that.
//
// `for<'a> Fn(&'a str)` says instead: "this closure must work for
// EVERY possible lifetime 'a", which lets the callee pick a fresh,
// short-lived borrow on each call. This is exactly the shape that
// `Fn`, `FnMut`, and `FnOnce` bounds involving references are given
// implicitly most of the time - HRTBs are usually invisible, but
// spelling them out with `for<'a>` is required once the bound also
// needs to mention 'a in more than one position, or you want a trait
// object like `dyn for<'a> Fn(&'a str) -> &'a str`.

// `f` must accept a borrow of *any* lifetime, because `process_lines`
// creates each `&str` fresh from a locally-owned `String`.
fn process_lines<F>(text: &str, f: F)
where
    F: for<'a> Fn(&'a str) -> usize,
{
    let mut total = 0;
    for line in text.lines() {
        // `owned` only lives for this loop iteration - a bound of
        // `Fn(&'fixed str)` for one external 'fixed could never be
        // satisfied here, since no lifetime outside the loop is long
        // enough. The `for<'a>` bound lets the compiler instantiate a
        // fresh, local 'a on every call instead.
        let owned = line.trim().to_uppercase();
        total += f(&owned);
    }
    println!("total scored: {total}");
}

// A trait object version: `dyn for<'a> Fn(&'a str) -> bool` is a
// boxed closure that must also work across arbitrarily many distinct
// borrows, one per invocation.
fn make_validator() -> Box<dyn for<'a> Fn(&'a str) -> bool> {
    Box::new(|s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric()))
}

fn main() {
    process_lines("rust\nis\nfun", |s| s.len());

    let validator = make_validator();
    for word in ["rust123", "", "not valid!"] {
        println!("{word:?} valid? {}", validator(word));
    }
}
