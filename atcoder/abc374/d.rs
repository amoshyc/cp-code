#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let n = inp[0] as usize;
    let s = inp[1] as f64;
    let t = inp[2] as f64;
    let mut seg = vec![];
    for _ in 0..n {
        let inp = readv::<f64>();
        seg.push(((inp[0], inp[1]), (inp[2], inp[3])));
    }

    let mut ans: f64 = 1e18;
    for perm in perm_iter(n) {
        for m in 0..(1 << n) {
            let mut cost = 0.0;
            let mut last = (0.0, 0.0);
            for i in 0..n {
                let (mut u, mut v) = seg[perm[i]];
                if (m >> i) & 1 == 1 {
                    (u, v) = (v, u);
                }
                // last -> u
                let d = ((u.0 - last.0).powi(2) + (u.1 - last.1).powi(2)).sqrt();
                cost += d / s;
                // u -> v
                let d = ((u.0 - v.0).powi(2) + (u.1 - v.1).powi(2)).sqrt();
                cost += d / t;
                last = v;
            }
            ans = ans.min(cost);
        }
    }
    println!("{:.7}", ans);
}

fn next_perm<T: Ord>(arr: &mut [T]) -> Option<()> {
    let k = arr.windows(2).rposition(|w| w[0] < w[1])?;
    let j = arr.iter().rposition(|a| a > &arr[k]).unwrap();
    arr.swap(k, j);
    arr[(k + 1)..].reverse();
    Some(())
}

fn perm_iter(n: usize) -> impl std::iter::Iterator<Item = Vec<usize>> {
    let mut perm: Vec<usize> = (0..n).collect();
    let iter1 = std::iter::once(perm.clone());
    let iter2 = std::iter::from_fn(move || next_perm(&mut perm).and_then(|_| Some(perm.clone())));
    iter1.chain(iter2)
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
