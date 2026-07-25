// Concept: Associated types vs. generic type parameters on traits.
//
// Both let a trait talk about a type it doesn't know in advance, but
// they answer different questions:
//   - A generic parameter (`trait Container<T>`) is an *input*: a
//     single type can implement `Container<A>` AND `Container<B>` at
//     the same time, because each instantiation is a distinct trait.
//   - An associated type (`trait Iterator { type Item; }`) is an
//     *output*: a given concrete type may implement the trait only
//     once, and that one implementation fixes the associated type
//     for good. This is what lets `Iterator::next` be called without
//     the caller specifying which `Item` type they meant - there's
//     only ever one answer for a given `Self`.
//
// Rule of thumb: if the type is determined by (or determines) the
// implementation - "this graph's nodes are always `RoomId`" - use an
// associated type. If a single type legitimately needs multiple
// implementations differing only in that type - "this container can
// hold `i32`s and, separately, `String`s" - use a generic parameter.

// --- Associated type: exactly one Node type per Graph implementation ---
trait Graph {
    type Node;
    fn neighbors(&self, node: &Self::Node) -> Vec<Self::Node>;
}

struct RoomMap;

#[derive(Debug, Clone, PartialEq)]
struct RoomId(u32);

impl Graph for RoomMap {
    // `RoomMap` can only ever be a `Graph` over `RoomId` - trying to
    // add a second `impl Graph for RoomMap` with a different `Node`
    // would be a duplicate-implementation compile error.
    type Node = RoomId;

    fn neighbors(&self, node: &RoomId) -> Vec<RoomId> {
        vec![RoomId(node.0 + 1), RoomId(node.0.saturating_sub(1))]
    }
}

// A function written against the associated type doesn't need a
// second generic parameter for "which Node type" - it's implied by
// which `Graph` was chosen.
fn describe_neighbors<G: Graph>(graph: &G, node: &G::Node) -> Vec<G::Node> {
    graph.neighbors(node)
}

// --- Generic parameter: one type can implement the trait many times ---
trait Converter<T> {
    fn convert(&self) -> T;
}

struct Measurement {
    millimeters: f64,
}

// `Measurement` implements `Converter<f64>` (meters) and
// `Converter<String>` (a display string) simultaneously - two
// distinct trait instantiations coexisting on the same type, which
// an associated type could never express.
impl Converter<f64> for Measurement {
    fn convert(&self) -> f64 {
        self.millimeters / 1000.0
    }
}

impl Converter<String> for Measurement {
    fn convert(&self) -> String {
        format!("{}mm", self.millimeters)
    }
}

fn main() {
    let map = RoomMap;
    let start = RoomId(5);
    println!("neighbors of {start:?}: {:?}", describe_neighbors(&map, &start));

    let measurement = Measurement { millimeters: 1500.0 };
    // The turbofish selects *which* `Converter<T>` implementation to
    // use, since both `f64` and `String` are valid here.
    let meters: f64 = Converter::<f64>::convert(&measurement);
    let label: String = Converter::<String>::convert(&measurement);
    println!("{meters} meters, label {label:?}");
}
