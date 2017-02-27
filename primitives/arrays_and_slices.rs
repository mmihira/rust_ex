use std::mem;

// Slices have the syntax &[<type>] and can borrow
// from an array. They have a pointer to an element
// in the array and also have a length they span.

// The & indicates a borrow
fn look_at_slize(slice: &[i32]){
    println!("Beginning array analysis");
    println!("first element of the slice {}", slice[0]);
    println!("second element of the slice {}", slice[1]);
    second_look(slice);
}

// borrowing the borrow
fn second_look(slice: &[i32]){
    println!("second look");
    println!("the third element of the slice {}", slice[2]);
}

fn main(){
    // Arrays and slices
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let bs: [i32; 2] = [1, 2];

    println!("5 length array: {}", xs.len());
    println!("2 length array: {}", bs.len());

    // Arrays are stack allocated
    // 32/8 = 4 bytes * 5 = 20 bytes
    println!("size of 5 length array: {}", mem::size_of_val(&xs));

    look_at_slize(&[1, 2, 3]);

    let xxs: [i32; 18] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18];
    println!("-------------------------");
    println!("-------------------------");
    look_at_slize(&xxs[5 .. 10]);
}
