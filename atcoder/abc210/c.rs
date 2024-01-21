#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    let mut cnt = std::collections::HashMap::new();
    for i in 0..k {
        *cnt.entry(arr[i]).or_insert(0) += 1;
    }

    let mut ans = cnt.len();
    for r in k..n {
        let l = r - k;
        *cnt.entry(arr[r]).or_insert(0) += 1;
        *cnt.entry(arr[l]).or_insert(0) -= 1;
        if cnt[&arr[l]] == 0 {
            cnt.remove(&arr[l]);
        }
        ans = ans.max(cnt.len());
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
