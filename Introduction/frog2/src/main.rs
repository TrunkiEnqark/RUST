use std::io;
use std::cmp::min;

fn main() {
    let mut lines = io::stdin().lines();
    let first_line = lines.next().unwrap().unwrap();
    let (n, k) = first_line.split_once(' ').unwrap();
    let n: usize = n.parse().unwrap();
    let k: usize = k.parse().unwrap();
    let height: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace().map(|element| element.parse().unwrap()).collect();
    let mut dp: Vec<i32> = vec!(i32::MAX; n);
    dp[0] = 0;
    for i in 0..n {
        for j in i + 1..i + k + 1 {
            if j < n {
                dp[j] = min(dp[j], dp[i] + (height[i] - height[j]).abs());
            }
        }
    }
    print!("{:?}", dp[n - 1]);
}
