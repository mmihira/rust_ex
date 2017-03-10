#[derive(Debug)]
pub struct Oy {
    pub k: [i32; 10]
}

impl Oy {
    fn tru(&mut self){
        self.k = [2; 10];
    }

    pub fn p(&mut self){
        self.tru(); // private function
        println!("I am = {:?}", self);
    }
}
