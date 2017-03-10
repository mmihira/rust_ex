#[allow(dead_code)]
use std::mem;

mod geom {
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct Point {
        pub x: f64,
        pub y: f64
    }

    #[derive(Debug)]
    pub struct Rectangle {
        pub p1: Point,
        pub p2: Point
    }

    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    impl Point {
        pub fn address_of(&self){
            println!("address is {:?}", self as *const Point);
        }
    }

    impl Drop for Point {
        fn drop(&mut self) {
            println!("Dropping a point residing at {:?}", self as *const Point);
        }
    }
}


fn main() {
    let p1 = geom::Point { x: 1.0, y: 2.0 };
    let origin = geom::origin();

    let r1: geom::Rectangle = geom::Rectangle {
        p1: p1,
        p2: origin
    };

    println!("The rectangle is {:?}", r1);
    println!("r1 occupies {:?} bytes on the stack", mem::size_of_val(&r1));
    println!("p1 occupies {:?} bytes on the stack", mem::size_of_val(&r1.p1));
    println!("\n--------------\n");

    let p3_boxed = Box::new(geom::Point { x: 20.0, y: 13.0 });
    println!("The p3_boxed is {:?}", *p3_boxed);
    println!("p3_boxed pointer occupies {:?} bytes on the heap", mem::size_of_val(&p3_boxed));
    println!("p3_boxed occupies {:?} bytes on the heap", mem::size_of_val(&*p3_boxed));
    println!("\n--------------\n");

    let r1_boxed = Box::new(geom::Rectangle {
        p1: geom::Point { x: 2.0, y: 3.0 },
        p2: geom::origin()
    });

    println!("The r1_boxed is {:?}", *r1_boxed);
    println!("r1_boxed pointer occupies {:?} bytes on the heap", mem::size_of_val(&r1_boxed));
    println!("r1_boxed occupies {:?} bytes on the heap", mem::size_of_val(&*r1_boxed));

    println!("\n--------------\n");
    println!("where in the memory are the points");
    r1.p1.address_of();
    r1.p2.address_of();
    p3_boxed.address_of();
    r1_boxed.p1.address_of();
    r1_boxed.p2.address_of();
}


