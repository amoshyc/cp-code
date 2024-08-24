#![allow(unused)]

const M1: u64 = 10u64.pow(9) + 7;
const M2: u64 = 10u64.pow(9) + 9;

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let arr_a = readv::<usize>();
    let arr_b = readv::<usize>();

    // random projection
    let mut rnd = XorShift64 { seed: 123 };
    let mut proj = vec![];
    for i in 0..=n {
        let r1 = rnd.new() % M1;
        let r2 = rnd.new() % M2;
        proj.push((r1, r2));
    }
    let arr_a = mapv(&arr_a, |&x| proj[x]);
    let arr_b = mapv(&arr_b, |&x| proj[x]);

    // query
    let pref_a = build(&arr_a);
    let pref_b = build(&arr_b);
    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<usize>();
        let (la, ra) = (ask[0] - 1, ask[1] - 1);
        let (lb, rb) = (ask[2] - 1, ask[3] - 1);
        let va = query(&pref_a, la, ra + 1);
        let vb = query(&pref_b, lb, rb + 1);
        if ra - la == rb - lb && va == vb {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
    }
    println!("{}", join(&ans, "\n"));
}

fn build(arr: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut pref = vec![arr[0]];
    for i in 1..arr.len() {
        let r1 = (pref[i - 1].0 + arr[i].0) % M1;
        let r2 = (pref[i - 1].1 + arr[i].1) % M2;
        pref.push((r1, r2));
    }
    pref
}

fn query(pref: &Vec<(u64, u64)>, l: usize, r: usize) -> (u64, u64) {
    if l == r {
        return (0, 0);
    }
    let mut v = pref[r - 1];
    if l > 0 {
        v.0 = (v.0 + M1 - pref[l - 1].0) % M1;
        v.1 = (v.1 + M2 - pref[l - 1].1) % M2;
    }
    v
}

struct XorShift64 {
    seed: u64, // should be nonzero
}

impl XorShift64 {
    fn new(&mut self) -> u64 {
        assert!(self.seed != 0);
        let mut x = self.seed;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.seed = x;
        x
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
