#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut c = vec![0; n];
    let mut a = vec![vec![]; n];
    for i in 0..n {
        c[i] = read::<usize>();
        a[i] = readv::<usize>();
    }
    let x = read::<usize>();

    let mut ans = vec![];
    let mut min = !0;
    for i in 0..n {
        if a[i].contains(&x) {
            ans.push(i);
            min = min.min(c[i]);
        }
    }

    ans = ans.iter().filter(|&i| c[*i] == min).cloned().collect();
    ans = mapv(&ans, |x| x + 1);
    println!("{}", ans.len());
    println!("{}", join(&ans, " "));
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
