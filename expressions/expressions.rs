fn main(){
    let mut x = 5;
    let x:u32 = {
        let z = 1u32;
        z + 5u32
    };
    println!("x is {}", x);
}
