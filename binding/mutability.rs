fn main(){
    let _immutable_binding = 1;
    let mut mutable_binding = 2;

    println!("before mutation {}", mutable_binding);
    mutable_binding = 3;

    println!("after mutation {}", mutable_binding);

}
