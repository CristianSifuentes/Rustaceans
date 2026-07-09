
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
    
    
}
