extern crate rand;
use std::collections::HashMap;
use rand::distributions::{IndependentSample, Range};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Free,
    Set,
    Dead
}

#[derive(Debug)]
pub enum Material {
    Sand,
    Water,
    Stone,
    Background
}

#[derive(Debug)]
pub struct T { x: u32 }

fn main(){
    let mut k: HashMap<u32, Box<T>> = HashMap::new();
    k.insert(1, Box::new(T{x:3}));
    k.get_mut(&1u32).unwrap().x = 2;
    println!("{:?}", k.get(&1u32).unwrap().x);

    let mut z = Box::new(T{x:2});
    z.x = 30;

    let between = Range::new(-1f64, 1.);
    let mut rng = rand::thread_rng();

    println!("{:?}", between.ind_sample(&mut rng));
}
