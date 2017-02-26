fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation
    let big_number = 100123i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    println!("default_float {}", default_float);
    println!("default_integer {}", default_integer);
    println!("default_float {}", a_float);
    println!("an integer {}", an_integer);
    println!("a big_number {}", big_number);
    println!("logical value {}", logical);
    println!("What the hell{}", logical);
}
