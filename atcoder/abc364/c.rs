#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, x, y) = (inp[0] as usize, inp[1], inp[2]);
    let mut arr_a = readv::<i64>();
    let mut arr_b = readv::<i64>();

    arr_a.sort();
    arr_b.sort();
    arr_a.reverse();
    arr_b.reverse();

    let pref_a = build(&arr_a);
    let pref_b = build(&arr_b);

    let cnt1 = (0..n).find(|i| pref_a[*i] > x).unwrap_or(n - 1) + 1;
    let cnt2 = (0..n).find(|i| pref_b[*i] > y).unwrap_or(n - 1) + 1;
    println!("{}", cnt1.min(cnt2));
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
