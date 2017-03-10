mod tx;
mod ty;

fn main(){
    let h = tx::Ox { x:1, y:2 };
    tx::tx_main(&h);

    let mut z = ty::Oy { k: [8; 10] };
    z.p();
}

