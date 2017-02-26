fn main(){
    // Integer Addition
    println!("1 + 2 = {}",2i32 + 3i32);
    println!("1 + 2 = {}",0b11 + 3i32);

    // Bitwise operations
    println!("0011 and 0010 = {}", 0b0011u32 & 0b_0011_u32); // Expect 3
    println!("1 << 5 = {}", 1 << 5 );
    println!("0b1 << 0b_101_i32 = {:b}", 0b1 << 0b_101_i32 );
    println!("0b1 in binary is {:b}", 0b1);
    println!("0x80 in binary {:b}", 0x80 );
    println!("0x80 >> 2 is {:b}", 0x80 >> 2);
}
