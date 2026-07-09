
// This is main point entry
fn main() {
    // First expression
    /* This is comment */
    println!("Hello, world!");

    let mut my_variable: &str = "This is a string";
    println!("Variable: {}" , my_variable);
    // my_variable = "change";
    // //cannot assign twice to immutable variable `my_variable`

    my_variable = "Change";
    println!("{my_variable}");

    // Rust is a strong type lenguage
    
    /*
    tring is an owned, growable, heap-allocated buffer of UTF-8 encoded bytes, while &str (string slice) is an immutable, borrowed reference to a sequence of UTF-8 bytes stored somewhere else
     */
    let my_variable2: String = String::from("This is other string");
    println!("{my_variable2}");
    
}
