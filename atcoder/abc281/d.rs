#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k, d) = (inp[0], inp[1], inp[2]);
    let arr = readv::<i64>();

    // dp[x][y][z] = max subset sum to choose y items from A[..=x] and the sum % d == z
    // dp[x][y][z] = max(dp[x - 1][y][z], dp[x - 1][y - 1][(z - A[x]) % d] + A[x])
    let mut dp = vec![vec![vec![std::i64::MIN; d]; k + 1]; n];
    dp[0][0][0] = 0;
    dp[0][1][arr[0] as usize % d] = arr[0];
    for x in 1..n {
        for y in 0..=k {
            for z in 0..d {
                let item1 = dp[x - 1][y][z];
                let item2 = if y == 0 {
                    std::i64::MIN
                } else {
                    let prev_z = (z as i64 - arr[x]).rem_euclid(d as i64);
                    dp[x - 1][y - 1][prev_z as usize] + arr[x]               
                };
                dp[x][y][z] = std::cmp::max(item1, item2);
            }
        }
    }

    if dp[n - 1][k][0] < 0 {
        println!("-1");
    } else {
        println!("{}", dp[n - 1][k][0]);
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
