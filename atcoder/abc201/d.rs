#![allow(unused)]

fn main() {
    // f(r, c, 0/1) = the evaluation from Takahashi's perspective
    //                 if the piece is at (r, c) and it's Takahashi/Aoki's turn.
    // f(r, c, 0) = max(f(r + 1, c, 1) + A[r + 1, c], f(r, c + 1, 1) + A[r, c + 1])
    // f(r, c, 1) = min(f(r + 1, c, 0) - A[r + 1, c], f(r, c + 1, 0) - A[r, c + 1])

    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = Vec::with_capacity(h);
    for _ in 0..h {
        let row = reads();
        let row = mapv(&row, |c| if *c == '+' { 1 } else { -1 });
        arr.push(row);
    }

    let mut dp = vec![vec![vec![0; 2]; w]; h];
    for r in (0..h).rev() {
        for c in (0..w).rev() {
            // t = 0
            let mut cands = vec![];
            if r + 1 < h {
                cands.push(dp[r + 1][c][1] + arr[r + 1][c]);
            }
            if c + 1 < w {
                cands.push(dp[r][c + 1][1] + arr[r][c + 1]);
            }
            if cands.len() == 0 {
                dp[r][c][0] = 0;
            } else {
                dp[r][c][0] = *cands.iter().max().unwrap();
            }
            // t = 1
            let mut cands = vec![];
            if r + 1 < h {
                cands.push(dp[r + 1][c][0] - arr[r + 1][c]);
            }
            if c + 1 < w {
                cands.push(dp[r][c + 1][0] - arr[r][c + 1]);
            }
            if cands.len() == 0 {
                dp[r][c][1] = 0;
            } else {
                dp[r][c][1] = *cands.iter().min().unwrap();
            }
        }
    }

    if dp[0][0][0] > 0 {
        println!("Takahashi");
    } else if dp[0][0][0] < 0 {
        println!("Aoki");
    } else {
        println!("Draw");
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
