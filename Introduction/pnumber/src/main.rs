use std::io;
use std::f32;

fn eratosthenes(lim: usize) -> Vec<bool> {
    let mut is_prime: Vec<bool> = vec![true; lim + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let n = (lim as f32).sqrt() as usize + 1;
    // print!("{}", n);
    for i in 2..n {
        if is_prime[i] == true {
            let mut j = i * i;
            while j <= lim {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    is_prime
}

fn main() {
    let mut lines = io::stdin().lines();
    let first_line = lines.next().unwrap().unwrap();
    let (a, b) = first_line.split_once(' ').unwrap();
    let a: usize = a.parse().unwrap();
    let b: usize = b.parse().unwrap();
    let is_prime: Vec<bool> = eratosthenes(200000);
    for i in a..b + 1 {
        if is_prime[i] == true {
            println!("{} ", i);
        }
    }
}
