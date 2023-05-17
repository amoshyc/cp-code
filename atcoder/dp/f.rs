#![allow(unused)]

fn main() {
    let s = reads();
    let t = reads();
    let n = s.len();
    let m = t.len();

    let mut dp = vec![vec![0; m]; n];
    let mut pr = vec![vec!['x'; m]; n];
    
    if s[0] == t[0] {
        dp[0][0] = 1;
        pr[0][0] = '↖';
    }

    for c in 1..m {
        if s[0] == t[c] {
            dp[0][c] = 1;
            pr[0][c] = '↖';
        } else {
            dp[0][c] = dp[0][c - 1];
            pr[0][c] = '←'
        }
    }
    for r in 1..n {
        if s[r] == t[0] {
            dp[r][0] = 1;
            pr[r][0] = '↖';
        } else {
            dp[r][0] = dp[r - 1][0];
            pr[r][0] = '↑';
        }
    }
    for r in 1..n {
        for c in 1..m {
            if s[r] == t[c] {
                dp[r][c] = dp[r - 1][c - 1] + 1;
                pr[r][c] = '↖';
            } else {
                if dp[r - 1][c] > dp[r][c - 1] {
                    dp[r][c] = dp[r - 1][c];
                    pr[r][c] = '↑';
                } else {
                    dp[r][c] = dp[r][c - 1];
                    pr[r][c] = '←';
                }
            }
        }
    }
    
    let mut r = n - 1;
    let mut c = m - 1;
    let mut p = vec![];
    loop {
        match pr[r][c] {
            '↖' => {
                p.push(s[r]);
                if r == 0 || c == 0 {
                    break;
                }
                r = r - 1;
                c = c - 1;
            }
            '↑' => {
                if r == 0 {
                    break;
                }
                r = r - 1;
            }
            '←' => {
                if c == 0 {
                    break;
                }
                c = c - 1;
            }
            _ => break,
        }
    }
    p.reverse();

    println!("{}", join(&p, ""));
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
