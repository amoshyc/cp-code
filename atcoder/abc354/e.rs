#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = vec![vec![0; n]; 2];
    for i in 0..n {
        let inp = readv::<usize>();
        arr[0][i] = inp[0];
        arr[1][i] = inp[1];
    }

    let mut dp = vec![vec![!0; (1 << n)]; 2];
    if dfs(0, (1 << n) - 1, &arr, &mut dp) == 0 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

fn dfs(who: usize, mask: usize, arr: &Vec<Vec<usize>>, dp: &mut Vec<Vec<usize>>) -> usize {
    if dp[who][mask] != !0 {
        return dp[who][mask];
    }

    let n = arr[0].len();
    let mut pairs = vec![];
    for i in 0..n {
        for j in (i + 1)..n {
            if (mask >> i) & 1 == 1 && (mask >> j) & 1 == 1 {
                if arr[0][i] == arr[0][j] || arr[1][i] == arr[1][j] {
                    pairs.push((i, j));
                }
            }
        }
    }

    if pairs.len() == 0 {
        return 1 - who;
    }

    let mut any = false;
    for (i, j) in pairs {
        if dfs(1 - who, mask ^ (1 << i) ^ (1 << j), arr, dp) == who {
            any = true;
        }
    }

    let res = if any { who } else { 1 - who };
    dp[who][mask] = res;
    return res;
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
