#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let s = reads();
    let s = mapv(&s, |&c| match c {
        'R' => 0,
        'P' => 1,
        'S' => 2,
        _ => panic!(),
    });

    let win = |a: usize, b: usize| -> bool {
        if a == 0 {
            return b == 2;
        } else if a == 1 {
            return b == 0;
        } else {
            return b == 1;
        }
    };

    // dp[i, 0/1/2] = maximum number of games Takahashi have won among game 0..=i
    //                while Takahashi i-th move is R/P/S
    let inf = 10i64.pow(10);
    let mut dp = vec![vec![0; 3]; n];
    for j in 0..3 {
        if win(j, s[0]) {
            dp[0][j] = 1;
        } else if win(s[0], j) {
            dp[0][j] = -inf;
        } else {
            dp[0][j] = 0;
        }
    }
    for i in 1..n {
        for j in 0..3 {
            if win(j, s[i]) {
                // Takahashi wins
                dp[i][j] = dp[i - 1][(j + 1) % 3].max(dp[i - 1][(j + 2) % 3]) + 1;
            } else if win(s[i], j) {
                // Takahashi loses
                dp[i][j] = -inf;
            } else {
                // draw
                dp[i][j] = dp[i - 1][(j + 1) % 3].max(dp[i - 1][(j + 2) % 3]);
            }
        }
    }
    println!("{}", dp[n - 1].iter().max().unwrap());
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
