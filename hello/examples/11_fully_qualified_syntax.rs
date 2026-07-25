// Concept: Fully qualified syntax.
//
// Method call syntax (`value.method()`) resolves ambiguity by
// preferring inherent methods, then searching traits in scope - but
// once two *traits* implement a method with the identical name on
// the identical type, the compiler cannot pick one for you and
// `value.method()` becomes a compile error. Fully qualified syntax,
// `<Type as Trait>::method(&value)`, sidesteps inference entirely by
// naming both the type and the trait explicitly.

trait Fly {
    fn launch(&self) -> &'static str;
}

trait Swim {
    fn launch(&self) -> &'static str;
}

struct Duck;

impl Duck {
    // An inherent method of the same name takes priority over both
    // trait methods in plain `duck.launch()` calls.
    fn launch(&self) -> &'static str {
        "Duck::launch (inherent): waddles off"
    }
}

impl Fly for Duck {
    fn launch(&self) -> &'static str {
        "<Duck as Fly>::launch: takes to the air"
    }
}

impl Swim for Duck {
    fn launch(&self) -> &'static str {
        "<Duck as Swim>::launch: dives underwater"
    }
}

// Associated functions (no `self`) are even more ambiguous, since
// there's no receiver value for the compiler to lean on at all - the
// fully qualified form is effectively mandatory here.
trait Named {
    fn name() -> String;
}

struct Robot;

impl Robot {
    fn name() -> String {
        "Robot::name (inherent)".to_string()
    }
}

impl Named for Robot {
    fn name() -> String {
        "<Robot as Named>::name".to_string()
    }
}

fn main() {
    let duck = Duck;

    // Plain method syntax resolves to the inherent impl - trait
    // methods of the same name are shadowed, not ambiguous, as long
    // as an inherent method exists.
    println!("{}", duck.launch());

    // Fully qualified syntax picks a specific trait impl explicitly,
    // bypassing the inherent method entirely.
    println!("{}", <Duck as Fly>::launch(&duck));
    println!("{}", <Duck as Swim>::launch(&duck));

    // Associated functions: `Robot::name()` alone would resolve to
    // the inherent function; naming the trait is required to reach
    // the trait implementation instead.
    println!("{}", Robot::name());
    println!("{}", <Robot as Named>::name());
}
