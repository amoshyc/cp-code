#![allow(unused)]

fn main() {
    // dp[i, j, k] = minimum number of people in 0..i to switch team to make
    //               team A has total strength j and team B has k.
    // person i switch to A/B/C

    let n = read::<usize>();
    let mut team = vec![];
    let mut str = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        team.push(inp[0] - 1);
        str.push(inp[1]);
    }
    let w = str.iter().sum::<usize>();
    if w % 3 != 0 {
        println!("-1");
        return;
    }
    let w = w / 3;

    let inf = 1_000_000;
    let mut dp = vec![vec![vec![inf; w + 1]; w + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        for j in 0..=w {
            for k in 0..=w {
                for t in 0..3 {
                    // person i to team t
                    let mut new_j = j;
                    let mut new_k = k;
                    if t == 0 {
                        new_j += str[i];
                    } else if t == 1 {
                        new_k += str[i];
                    }
                    let val = dp[i][j][k] + if team[i] == t { 0 } else { 1 };
                    if new_j <= w && new_k <= w {
                        dp[i + 1][new_j][new_k] = dp[i + 1][new_j][new_k].min(val);
                    }
                }
            }
        }
    }

    if dp[n][w][w] == inf {
        println!("-1");
    } else {
        println!("{}", dp[n][w][w]);
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
