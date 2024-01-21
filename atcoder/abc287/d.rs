#![allow(unused)]

fn is_matched(a: char, b: char) -> bool {
    a == b || a == '?' || b == '?'
}

fn main() {
    let s = reads();
    let t = reads();
    let n = s.len();
    let m = t.len();

    // pref[i] = s[..i] match t[..i]
    let mut pref = vec![false; n];
    pref[0] = is_matched(s[0], t[0]);
    for i in 1..m {
        pref[i] = pref[i - 1] && is_matched(s[i], t[i]);
    }

    // suff[i] = s[i..] match t[i..]
    let mut suff = vec![false; n];
    suff[n - 1] = is_matched(s[n - 1], t[m - 1]);
    for i in (2..=m) {
        suff[n - i] = suff[n - i + 1] && is_matched(s[n - i], t[m - i]);
    }

    let mut ans = vec![];
    for x in 0..=(n - (n - m)) {
        let (s, e) = (x, x + (n - m) - 1); // s..=e
        let mut ok = true;
        if s != 0 && !pref[s - 1] {
            ok = false;
        }
        if e != n - 1 && !suff[e + 1] {
            ok = false;
        }
        ans.push(if ok { "Yes" } else { "No" });
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
