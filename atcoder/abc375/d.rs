#![allow(unused)]

fn main() {
    let s = reads();
    let n = s.len();
    let mut f = vec![vec![0 as i64; n]; 26];
    for i in 0..n {
        f[s[i] as usize - 'A' as usize][i] = 1;
    }

    let mut ans = 0;
    for c in 0..26 {
        let pref = build(&f[c]);
        for j in 1..(n - 1) {
            let cnt_l = query(&pref, 0, j);
            let cnt_r = query(&pref, j + 1, n);
            ans += cnt_l * cnt_r;
        }
    }
    println!("{}", ans);
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
