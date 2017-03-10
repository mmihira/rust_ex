
fn main(){
    let integer = 1u32;
    let a_bool = false;
    let unit = ();

    let copied_int = integer;

    println!("An integer: {:?}", copied_int);
    println!("A boolean value: {:?}", a_bool);
    println!("The unit value: {:?}", unit);

    let _unused = 10u64;
}
