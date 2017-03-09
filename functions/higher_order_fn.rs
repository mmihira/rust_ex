#[derive(Debug)]
struct Ox{
    x:  i32,
    y:  i32
}



fn main(){
    let v = vec![1, 2, 4, 6, 8, 9];
    println!("v is = {:?}", v);

    let z = v.iter().map(|&n| n * 2);
    for i in z {
        println!("i is {:?}", i);
    }

    let k = vec![
        Ox { x: 1, y: 2},
        Ox { x: 2, y: 9},
        Ox { x: 3, y: 8},
        Ox { x: 4, y: 8}
    ];

    println!("k is {:?}", k);
    let w = k.iter().map(|u: &Ox| -> i32 { u.x + 1 });
    let l = k.iter().map(|u: &Ox| -> Ox {
        let add = |x: i32| x + 3;
        Ox { x: add(u.x), y: add(u.y)}
    });

    println!("w is {:?}", w);
    for z in w { println!("z is {:?}", z); }
    for z in l { println!("z is {:?}", z); }
}

