#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Composite(Structure, Structure);

#[derive(Debug)]
struct Comp<'a, 'b>(&'a mut Composite, &'b Composite);

fn ret(r: &[i32]) -> i32 { return 2; }

#[derive(Debug)]
struct Az<'a>(&'a[i32]);

fn tr<'a>() {
    let n = 32;
    let k = 34;
}

fn stuff() {
    {
        let z = Structure(3);
        println!("A structure class: {:?}", z);

        let mut w = Composite(Structure(2), Structure(3));
        let n = Composite(Structure(2), Structure(3));
        println!("A composite class: {:?}", w);

        {
            let k = Comp(&mut w, &n);
            println!("A Comp class: {:?}", k);
        }

        println!("{:?}", w);
        println!("A composite class: {:?}", w);
    }

    {
        let n: [i32; 6] = [1, 2, 3, 4, 5, 6];
        println!("{:?}", n);
        // Immutable reference from n
        println!("{:?}", &n[2..6]);
        ret(&n[4..5]);

        let r: [i32; 4] = [1, 2, 3, 4];
        let n = Az(&r);

        let k = ret(&r);
        println!("{:?}", k);
    }

    {
        let q: [i32; 10] = [1,2,3,4,4,5,6,7,7,8];
    }
}

fn halloc() -> Box<Structure> {
    Box::new(Structure(2))
}


fn main() {
    let x: Box<Structure> = Box::new(Structure(2));
    let y: Box<&Box<Structure>> = Box::new(&x);
    println!("Here it is {:?}", ***y);
    println!("Here it is {:?}", x);

    let n = 2;
    let k = &n;
    println!("Here it is {:?}", n);
    let l: Box<Structure> = halloc();
}
