use std::io;

let mut s: String = String::from("Hello World!");

fn change(new_string: String) {
    s = new_string;
}
fn main() {
    // let mut s: String = String::from("Hello world");
    println!("{}", s);
    change(String::from("Bye world"));
    println!("{}", s);
}