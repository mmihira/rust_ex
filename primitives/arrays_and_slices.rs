use std::mem;

fn main(){
    // Arrays and slices
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let bs: [i32; 2] = [1, 2];

    println!("5 length array: {}", xs.len());
    println!("2 length array: {}", bs.len());

    // Arrays are stack allocated
    // 32/8 = 4 bytes * 5 = 20 bytes
    println!("size of 5 length array: {}", mem::size_of_val(&xs));
}
