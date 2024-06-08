#![allow(unused)]

fn main() {
    let n = read::<usize>();

    let q = readv::<i64>();
    let a = readv::<i64>();
    let b = readv::<i64>();

    let inf = 10i64.pow(9);

    let mut ans = 0;
    let max = *q.iter().min().unwrap();
    for cnt_a in 0..=max {
        let mut cnt_b = inf;
        let mut ok = true;
        for i in 0..n {
            if q[i] < a[i] * cnt_a {
                ok = false;
                break;
            }
            if b[i] != 0 {
                cnt_b = cnt_b.min((q[i] - a[i] * cnt_a) / b[i]);
            }
        }
        if ok && cnt_b != inf {
            ans = ans.max(cnt_a + cnt_b);
        }
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
