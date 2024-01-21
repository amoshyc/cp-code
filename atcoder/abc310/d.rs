#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, t, m) = (inp[0], inp[1], inp[2]);

    let mut ban = vec![];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (a, b) = (inp[0] - 1, inp[1] - 1);
        ban.push((a, b));
    }

    let mut possible = vec![true; (1 << n)];
    for s in 0..(1 << n) {
        for &(u, v) in ban.iter() {
            if (s >> u) & 1 == 1 && (s >> v) & 1 == 1 {
                possible[s] = false;
            }
        }
    }

    // dp[i][s] = number of ways to pick i teams from people set s
    // By enumerating the members of a team ss, dp[i][s] = sum(dp[t - 1][s - ss])
    let mut dp = vec![vec![0i64; 1 << n]; t + 1];
    dp[0][0] = 1;
    for i in 1..=t {
        for s in 1..(1 << n) {
            let mut ss = s;
            while ss > 0 {
                if possible[ss] {
                    dp[i][s] += dp[i - 1][s - ss];
                }
                ss = (ss - 1) & s;
            }
        }
    }

    // divided by t!
    let mut ans = dp[t][(1 << n) - 1];
    for i in 1..=t {
        ans = ans / (i as i64);
    }

    println!("{}", ans);
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

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
