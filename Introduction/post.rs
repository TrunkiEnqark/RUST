use std::io;
// use std::io::prelude::*

fn main() {
    let mut val = String::new();
    io::stdin().read_line(&mut val).expect("failed to read");

    let mut nxt = val.split_whitespace();

    let mut next_num = || -> usize {
        nxt.next().expect("fail").parse().expect("fail")
    };

    let ia = next_num();
    let ib = next_num();

    println!("{}", ia + ib);
}