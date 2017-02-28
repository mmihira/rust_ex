fn main(){

    let n: u32 = 100;
    let x: bool =
        if n < 10 {
            true
        } else if n > 200 {
            true
        } else {
            false
        };

    println!("Expect x to be {}", x);

}
