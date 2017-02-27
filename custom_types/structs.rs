
// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, i32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point
}

#[allow(dead_code)]
struct Carry {
    array: [i32; 4],
    point: Point
}

fn print_point(point: &Point){
    println!("x = {}, y = {}", point.x, point.y);
}

fn print_carry(carry: &Carry){
    println!("\nPrinting carry");
    print_point(&carry.point);
}

fn rect_area(rect: &Rectangle) -> f32 {
    // Nested destructuring
    let &Rectangle { p1: Point { x: x1, y: y1 }, p2: Point { x: x2, y: y2 } } = rect;
    let length = y2 - y1;
    let width = x2 - x1;
    return length * width;
}

fn main(){
    // Instantiate a point
    let point: Point = Point { x: 0.3, y: 0.4 };
    print_point(&point);
    let carry: Carry = Carry { array: [1, 2, 3, 4], point: Point { x: 0.2, y: 0.5 } };
    print_carry(&carry);

    // Destructure a struct
    #[allow(dead_code)]
    let Carry { array: xs, point: p } = carry;
    print_point(&p);

    let rect: Rectangle = Rectangle { p1: Point { x: 0.3, y:0.4 }, p2: Point { x: 0.9, y: 1.0 } };

    println!("Area of rectangel is {}", rect_area(&rect));
}
