#[derive(Debug)]
pub struct Ox{
    pub x: i32,
    pub y: i32
}

pub fn tx_main(z: &Ox){
    println!("Got {:?}", z);
}
