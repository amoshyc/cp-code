#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, k) = (inp[0], inp[1], inp[2]);
    const MOD: u64 = 998_244_353;

    let build = |arr: &Vec<u64>| -> Vec<u64> {
        let mut pref = vec![arr[0]];
        for i in 1..arr.len() {
            pref.push((pref[i - 1] + arr[i]) % MOD);
        }
        pref
    };

    let query = |arr: &Vec<u64>, l: usize, r: usize| -> u64 {
        if l == r {
            return 0;
        }
        let mut val = arr[r - 1];
        if l > 0 {
            val = val + MOD - arr[l - 1];
            val = val % MOD;
        }
        val
    };

    // dp[i, j] = number of valid A[..=i] while A[i] = j
    // dp[i, j] = sum(dp[i - 1, x] for all |x - j| >= k)
    // x - j <= -k => 1 <= x <= j - k
    // x - j >= k => m >= x >= j + k
    let mut dp = vec![vec![0 as u64; m + 1]; n];
    for j in 1..=m {
        dp[0][j] = 1;
    }
    for i in 1..n {
        let pref = build(&dp[i - 1]);
        for j in 1..=m {
            if k == 0 {
                dp[i][j] = query(&pref, 1, m + 1);
            } else {   
                if j >= k + 1 {
                    dp[i][j] += query(&pref, 1, j - k + 1);
                    dp[i][j] %= MOD;
                }
                if j + k <= m {
                    dp[i][j] += query(&pref, j + k, m + 1);
                    dp[i][j] %= MOD;
                }
            }
        }
    }

    let mut ans = 0;
    for j in 1..=m {
        ans = ans + dp[n - 1][j];
        ans = ans % MOD;
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
