use std::io;
use std::cmp::max;

fn main() {
    let mut lines = io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().expect("failed");
    let mut exc: Vec<(i32, i32, i32)> = vec![(0, 0, 0); n];
    for i in 0..n {
        let tup: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace()
                                                        .map(|element| element.parse().unwrap())
                                                        .collect();
        exc[i] = (tup[0], tup[1], tup[2]);
    }
    let mut dp: Vec<(i32, i32, i32)> = vec![(0, 0, 0); n];
    for i in 0..n {
        if i == 0 {
            dp[0] = exc[0];
        } else {
            dp[i].0 = max(dp[i - 1].1, dp[i - 1].2) + exc[i].0;
            dp[i].1 = max(dp[i - 1].0, dp[i - 1].2) + exc[i].1;
            dp[i].2 = max(dp[i - 1].0, dp[i - 1].1) + exc[i].2;
        }
    }
    print!("{}", max(dp[n - 1].0, max(dp[n - 1].1, dp[n - 1].2)));
}