#![allow(unused)]

fn solve() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let s = reads();
    let t = reads();

    let mut pref_s = vec![];
    let mut pref_t = vec![];
    for c in 'a'..='z' {
        let arr_s = mapv(&s, |x| if *x == c { 1 } else { 0 } as i64);
        let arr_t = mapv(&t, |x| if *x == c { 1 } else { 0 } as i64);
        pref_s.push(build(&arr_s));
        pref_t.push(build(&arr_t));
    }

    let mut ans = vec![0; q];
    for qid in 0..q {
        let ask = readv::<usize>();
        let (l, r) = (ask[0] - 1, ask[1]);
        for i in 0..26 {
            ans[qid] += query(&pref_s[i], l, r).abs_diff(query(&pref_t[i], l, r));
        }
        ans[qid] /= 2;
    }
    println!("{}", join(&ans, "\n"))
}

fn build<T>(arr: &[T]) -> Vec<T>
where
    T: Copy + std::ops::Add<Output = T>,
{
    let mut pref = vec![];
    pref.push(arr[0]);
    for i in 1..arr.len() {
        pref.push(pref[i - 1] + arr[i]);
    }
    pref
}

// i..j
fn query<T>(pref: &[T], i: usize, j: usize) -> T
where
    T: Default + Copy + std::ops::Sub<Output = T>,
{
    if i == j {
        return T::default();
    }
    let mut res = pref[j - 1];
    if i > 0 {
        res = res - pref[i - 1];
    }
    res
}

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        solve();
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
