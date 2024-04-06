// https://oj.vnoi.info/problem/liq
use std::io;
use std::cmp::max;

fn main() {
    let mut lines = io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().expect("fail");
    let array_raw = lines.next().unwrap().unwrap();
    let arr: Vec<i32> = array_raw.split_whitespace().map(|element| element.parse().unwrap()).collect();
    let mut dp = vec![1 as i32; n];
    
    for j in 1..n {
        for i in 0..j {
            if arr[j] > arr[i] {
                dp[j] = max(dp[j], dp[i] + 1);
            }
        }
    }
    match dp.iter().max() {
        Some(max_value) => print!("{}", max_value),
        None => print!("failed"),
    }
}