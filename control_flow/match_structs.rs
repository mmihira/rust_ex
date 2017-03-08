#[derive(Debug)]
struct Ox {
    tp: (u32, u32),
    fx: u64,
}

fn m(x: &Ox){
    match *x {
        Ox{ fx: 2, .. } => println!("only care about you"),
        Ox{ tp: x, fx: 13 } => println!("actually 13"),
        Ox{ tp: x, .. } => println!("this is x now {:?}", x)
    }
}

fn main(){
    let x = Ox { tp: (1, 1), fx: 14u64 };
    let y = Ox { tp: (1, 1), fx: 13u64 };
    let z = Ox { tp: (1, 1), fx: 2u64 };
    println!("x is {:?}", x);

    m(&x);
    m(&y);
    m(&z);
}
