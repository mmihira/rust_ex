
#[allow(dead_code)]
enum Coder {
    FrontEnd,
    BackEnd,
    Evangelist,
    Ninga(i32),
    Ai { make: String, power: f64 }
}

#[allow(dead_code)]
fn inspect_coder(coder: Coder){
    match coder {
        Coder::FrontEnd => println!("Is a FrontEnd person"),
        Coder::BackEnd => println!("Is a BackEnd person"),
        Coder::Evangelist => println!("Evangelist !! what ?"),
        Coder::Ninga(level) => println!("Ninga of level {}", level),
        Coder::Ai { make: m, power: _p } => println!("Ai of make {}", m),
    }
}

fn main(){
    let person = Coder::BackEnd;
    let ninga = Coder::Ninga(23);
    let Ai = Coder::Ai { make: "What the".to_owned(), power: 78.0 };

    inspect_coder(person);
    inspect_coder(ninga);
    inspect_coder(Ai);
}


