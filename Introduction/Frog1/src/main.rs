use std::io;
// use std::num;
use std::cmp::min;

fn main() {
    let mut lines = io::stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().expect("failed");
    let height: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace()
                                                 .map(|element| element.parse().unwrap())
                                                 .collect();
    let mut dp = vec![i32::MAX; n];

    dp[0] = 0;
    for i in 0..n {
        for j in i + 1..i + 3 {
            if j < n {
                dp[j] = min(dp[j], dp[i] + (height[i] - height[j]).abs());
            }
        }
    }

    println!("{}", dp[n - 1]);
}
