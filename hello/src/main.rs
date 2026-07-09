
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


    let mut my_int: i32 = 7;
    my_int = my_int + 4;
    println!("{my_int}");
    println!("{}", my_int - 1); 


    println!("This is a value for my_int: {}", my_int);

    let my_int64: i64 = 7;
    println!("{my_int64}");

    let my_float: f64 = 6.5;
    println!("{my_float}");           
    // my_float = my_float + my_int;   
    // cannot mutate immutable variable `my_float`rust-analyzerE0384

    let my_float2: f32 = 6.5;
    println!("{my_float2}"); 

    let mut my_bool: bool = false;
    my_bool = true;
    println!("{my_bool}");
    

    // Constants
    const MY_CONST: &str = "MY";// does not infer at the moment

         
}
