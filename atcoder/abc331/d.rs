#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);

    let mut arr = vec![];
    for _ in 0..n {
        let inp = reads();
        let inp: Vec<i64> = mapv(&inp, |&x| if x == 'B' { 1 } else { 0 });
        arr.push(inp);
    }

    let mut pref = arr.clone();
    for r in 0..n {
        for c in 0..n {
            if r >= 1 {
                pref[r][c] += pref[r - 1][c];
            }
            if c >= 1 {
                pref[r][c] += pref[r][c - 1];
            }
            if r >= 1 && c >= 1 {
                pref[r][c] -= pref[r - 1][c - 1];
            }
        }
    }

    let f = |r: usize, c: usize| -> i64 {
        let rr = ((r + 1) / n) as i64;
        let cc = ((c + 1) / n) as i64;
        let mut cnt = 0;
        cnt += pref[n - 1][n - 1] * rr * cc;
        if r % n != n - 1 {
            cnt += pref[r % n][n - 1] * cc;
        }
        if c % n != n - 1 {
            cnt += pref[n - 1][c % n] * rr;
        }
        if r % n != n - 1 && c % n != n - 1 {
            cnt += pref[r % n][c % n];
        }
        cnt
    };

    let mut ans = vec![0; q];
    for i in 0..q {
        let inp = readv::<usize>();
        let (a, b) = (inp[0], inp[1]);
        let (c, d) = (inp[2], inp[3]);

        ans[i] = f(c, d);
        if a >= 1 {
            ans[i] -= f(a - 1, d);
        }
        if b >= 1 {
            ans[i] -= f(c, b - 1);
        }
        if a >= 1 && b >= 1 {
            ans[i] += f(a - 1, b - 1);
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
