fn main(){
    let mut count: i8 = -10;

    loop {
        if count > 10 { break; }
        if count % 2 == 0 { println!("is now : {}", count); }
        count += 1;
    }
}

