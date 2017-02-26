fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn main(){
    println!("reverse this {:?}", reverse((20,false)));

    let long_tuple = (1u8, 1i32, 10i8);
    println!("tuple index 3 = {:b}", long_tuple.2);
    println!("all tuple = {:?}", long_tuple);

    let nested_tuple = ((1,1), "say whay");
    println!("all tuple = {:?}", nested_tuple);

    // Destructure
    let tup = (10u8, 11u8, 12u8);
    let (a, b, c) = tup;
    println!("{}, {}, {}", a, b, c);
}
