#![allow(unused)]

fn main() {
    let inp = readv::<String>();
    let n = inp[0].parse::<usize>().unwrap();
    let t = inp[1].chars().collect::<Vec<char>>();
    let m = t.len();
    let mut rt = t.clone();
    rt.reverse();

    let mut a = vec![0; n];
    let mut b = vec![0; n];
    for i in 0..n {
        let s = reads();
        let mut rs = s.clone();
        rs.reverse();
        a[i] = max_match(&s, &t);
        b[i] = max_match(&rs, &rt);
    }

    let mut cnt = vec![0; m + 1];
    for &x in b.iter() {
        cnt[x] += 1;
    }

    let mut suff = vec![0; m + 1];
    suff[m] = cnt[m];
    for i in (0..m).rev() {
        suff[i] = suff[i + 1] + cnt[i];
    }

    let mut ans: i64 = 0;
    for i in 0..n {
        // a[i] + b[j] >= m
        // b[j] >= m - a[i]
        if m >= a[i] {
            ans += suff[m - a[i]];
        }
    }

    println!("{}", ans);
}

fn max_match(s: &Vec<char>, t: &Vec<char>) -> usize {
    let mut t_idx = 0;
    for i in 0..s.len() {
        if t_idx < t.len() && s[i] == t[t_idx] {
            t_idx += 1;
        }
    }
    t_idx
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
