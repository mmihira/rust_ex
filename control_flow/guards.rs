#[derive(Debug)]
struct Ox {
    x: i32,
    y: i32,
    z: i32,
    f: String,
    t: (i32, i32)
}

impl Ox {
    fn add(&self) -> i32 {
        self.x + self.y + self.z
    }

    fn addw(&self, n: i32) -> i32 {
        self.add() + n
    }
}

fn m(x: &Ox){
    match *x {
        Ox { x: z, .. } if z > 10 => println!("got z > 10, with : {:?}", *x),
        _ => println!("something else")
    }
}

fn main() {
    let z = Ox { x: 1, y: 2, z: 3, f: "Hello".to_owned(), t: (0, 0) };
    let k = Ox { x: 100, y: 2, z: 100, f: "Hello".to_owned(), t: (0, 0) };
    println!("z.add = {:?}", z.add());
    println!("z.addw = {:?}", z.addw(5));
    println!("");

    println!("Hello what is going on ");

    m(&z);
    m(&k);
}
