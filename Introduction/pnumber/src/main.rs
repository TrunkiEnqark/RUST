use std::io;
use std::f32;

fn eratosthenes(lim: usize) -> Vec<bool> {
    let mut isPrime: Vec<bool> = vec![true; lim + 1];
    isPrime[0] = false;
    isPrime[1] = false;
    let n = (lim as f32).sqrt() as usize + 1;
    // print!("{}", n);
    for i in 2..n {
        if isPrime[i] == true {
            let mut j = i * i;
            while j <= lim {
                isPrime[j] = false;
                j += i;
            }
        }
    }
    isPrime
}

fn main() {
    let mut lines = io::stdin().lines();
    let first_line = lines.next().unwrap().unwrap();
    let (a, b) = first_line.split_once(' ').unwrap();
    let a: usize = a.parse().unwrap();
    let b: usize = b.parse().unwrap();
    let isPrime: Vec<bool> = eratosthenes(200000);
    for i in a..b + 1 {
        if isPrime[i] == true {
            println!("{} ", i);
        }
    }
}
