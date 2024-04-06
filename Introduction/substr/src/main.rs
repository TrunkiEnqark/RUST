use std::io;

fn main() {
    let mut lines = io::stdin().lines();
    let a: String = lines.next().unwrap().unwrap();
    let b: String = lines.next().unwrap().unwrap();

    

    for i in 0..a.len() - b.len() + 1 {
        if a[i..i + b.len()] == b {
            print!("{} ", i + 1);
        }
    }
}
