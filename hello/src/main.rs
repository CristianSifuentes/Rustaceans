use std::collections::{HashMap, HashSet, btree_map::IntoValues};


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
    my_int = 10;
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
    println!("{MY_CONST}");

    // Control flow
    if my_int == 10 && my_bool{
        println!("10");
    } else if my_int == 11 {
                println!("11");

    }else {
        println!("no 10");
    }

    // Vec
    // When you need to store a list of items of the same type that can grow
    // or shrink in size at runtime
    let mut my_list: Vec<&str> = vec!["Angular", "React", "Astro"];
    my_list.push("Python");
    my_list.push("NET");
    my_list.push("Go");
    my_list.push("SQL");
    println!("{:?}",my_list);
    println!("{}", my_list[0]);
    

    //HashSet, When you need to maintain a collection of onique items with no duplicate values, 
    // require O(1) constant-time lookup performance, or need to perform mathematical set operations
    let mut my_hash: HashSet<&str> = vec!["Angular", "React", "Astro"].into_iter().collect();
    my_hash.insert("Go");
    println!("{:?}",my_hash);

    

    // Maps
    // When you need to associate unique keys with specific values for rapid, 
    // out-of-order data retrieval    
    let mut my_map: HashMap<&str, i32> = vec![
        ("Cris", 36),
        ("Other", 56),
        ("Other3", 16)
        ].into_iter().collect();
        my_map.insert("insert", 76);


    println!("{:?}", my_map);

    //Bucles
    //& is used when need to reuse using pointer
    for value in &my_list {
        println!("list {value}")
    }

    for value in my_hash {
        println!("hash {}", value);
    }

    for (key, value     )    in my_map {
        println!("key {} value {}", key, value)
    }
    let mut my_counter: usize = 0;
    while my_counter < my_list.len() {
        
        print!("{}", my_list[my_counter]);
        my_counter+=1;
    }


         
}
