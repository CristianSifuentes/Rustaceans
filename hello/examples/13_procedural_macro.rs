// Concept: using a procedural macro from the caller's side.
//
// See `hello_macros/src/lib.rs` for the macro's implementation. From
// here, `#[derive(Describe)]` looks like any other derive - the fact
// that it's implemented by a separate compiler-plugin crate running
// custom code at compile time is entirely invisible at the call site,
// which is exactly the point: proc macros extend the language's
// surface syntax without needing any special support from rustc
// itself beyond the derive attribute mechanism.

use hello_macros::Describe;

trait Describe {
    fn describe(&self) -> String;
}

#[derive(Describe)]
struct Package {
    name: &'static str,
    version: &'static str,
    downloads: u64,
}

#[derive(Describe)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

fn main() {
    let package = Package {
        name: "hello_macros",
        version: "0.1.0",
        downloads: 42,
    };
    // `describe()` did not exist anywhere in source we wrote by hand
    // - it was generated entirely by `derive_describe` at compile
    // time, from the field list of `Package` alone.
    println!("{}", package.describe());

    let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };
    println!("{}", point.describe());
}
