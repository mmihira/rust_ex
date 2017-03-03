fn k(i: &mut (i32, i32)){
    println!("got {:?}", i);
    (*i).0 = 4;
}

// referene to a mutable value a
fn m(a: &mut (i32, i32)){
  match *a {
      (_, 2) => println!("got 2"),
      (_, 3) => k(a),
      _ => println!("else")
  }
}

fn main(){
  // allow a to be mutable
  let mut a: (i32, i32) = (1, 2);
  m(&mut a);
  a = (1, 3);
  m(&mut a);
  println!("{:?}", a);
}

