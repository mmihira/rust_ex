fn main(){
    // This is a raw pointer to an i32
    let mut raef: *const i32 = &2;
    println!("{:?}", raef as *const i32);
    unsafe {
        println!("{:?}", *raef );
    }
}


