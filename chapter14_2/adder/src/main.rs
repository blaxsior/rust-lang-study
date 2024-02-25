use add_one;

fn main() {
    println!("Hello, world!");

    let v = 10;
    println!("add one {} = {}", v, add_one::add_one(v));
}
