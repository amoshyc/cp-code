#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<u64>();
    let mut adj = vec![vec![false; n]; n];
    for r in 0..n {       
        let inp = reads();
        for c in 0..n {
            if inp[c] == 'Y' {
                adj[r][c] = true;
            }
        }
    }

    let mut dp = vec![vec![(n + 1, 0); n]; n];

    for s in 0..n {
        for t in 0..n {
            if adj[s][t] {
                dp[s][t] = (1, arr[s] + arr[t]);
            }
        }
    }

    for k in 0..n {
        for s in 0..n {
            for t in 0..n {
                // s ~> t
                let mut f = dp[s][t].0;
                let mut v = dp[s][t].1;

                // s ~> k ~> t
                let (f_sk, v_sk) = dp[s][k];
                let (f_kt, v_kt) = dp[k][t];
                let new_f = f_sk + f_kt;
                let new_v = (v_sk + v_kt).saturating_sub(arr[k]);
                if new_f < f || (new_f == f && new_v > v) {
                    f = new_f;
                    v = new_v;
                }

                dp[s][t] = (f, v);
            }
        }
    }

    let q = read::<usize>();
    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (s, t) = (inp[0] - 1, inp[1] - 1);
        let (f, v) = dp[s][t];
        if f >= n {
            ans.push("Impossible".to_string());
        } else {
            ans.push(format!("{} {}", f, v));
        }
    }
    println!("{}", join(&ans, "\n"));
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
