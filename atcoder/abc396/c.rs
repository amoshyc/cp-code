#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr_b = readv::<i64>();
    let mut arr_w = readv::<i64>();

    arr_b.sort();
    arr_b.reverse();
    arr_w.sort();
    arr_w.reverse();

    let mut pref_b = 0;
    let mut pref_w = 0;

    let mut ans = 0;
    for i in 0..n {
        pref_b += arr_b[i];
        if i < m && arr_w[i] > 0 {
            pref_w += arr_w[i];
        }
        ans = ans.max(pref_b + pref_w);
    }

    println!("{}", ans);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
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
