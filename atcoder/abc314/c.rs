#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let s = reads();
    let c = readv::<usize>();
    let c = mapv(&c, |&x| x - 1);

    let mut groups = vec![vec![]; m];
    for i in 0..n {
        groups[c[i]].push(i);
    }

    let mut ans = vec![' '; n];
    for i in 0..m {
        for j in 0..groups[i].len() {
            let p = groups[i][(j + 1) % groups[i].len()];
            ans[p] = s[groups[i][j]];
        }
    }

    println!("{}", join(&ans, ""));
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
