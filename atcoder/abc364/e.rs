#![allow(unused)]

fn main() {
    // dp[i, j, k] = maximum number among dishes 0..=i while total sweetness is i and saltiness is k
    // dp[i, j, k] = dp[i - 1, j - A[i], k - B[i]] + 1
    // probably will TLE

    // dp[i, j, k] = minimum total saltiness among dishes 0..=i while number of eaten dishes is j and total sweetness is k
    // dp[i, j, k]:
    //     if dish i is chosen: dp[i, j, k] = dp[i - 1, j - 1, k - A[i]] + B[j]
    //     if dish i is not chosen: dp[i, j, k] = dp[i - 1, j, k]

    let inp = readv::<usize>();
    let (n, x, y) = (inp[0], inp[1], inp[2]);
    let mut arr_a = vec![];
    let mut arr_b = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        arr_a.push(inp[0]);
        arr_b.push(inp[1]);
    }

    let inf = 100_000;
    let mut dp = vec![vec![vec![inf; x + 1]; n + 1]; n];

    // dish 0 is chosen
    if arr_a[0] <= x {
        dp[0][1][arr_a[0]] = arr_b[0];
    }
    // dish 0 is not chosen
    dp[0][0][0] = 0;

    for i in 1..n {
        for j in 0..=(i + 1) {
            for k in 0..=x {
                // dish i is chosen
                if j >= 1 && k >= arr_a[i] {
                    dp[i][j][k] = dp[i][j][k].min(dp[i - 1][j - 1][k - arr_a[i]] + arr_b[i]);
                }
                // dish i is not chosen
                dp[i][j][k] = dp[i][j][k].min(dp[i - 1][j][k]);
            }
        }
    }

    let mut ans = 0;
    for j in 0..=n {
        for k in 0..=x {
            if dp[n - 1][j][k] <= y {
                ans = ans.max(j);
            }
        }
    }

    if ans < n {
        ans += 1;
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
    read::<String>().chars().collect()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
