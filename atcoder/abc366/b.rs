#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut s = vec![];
    let mut m = 0;
    for _ in 0..n {
        let x = reads();
        m = m.max(x.len());
        s.push(x);
    }

    let mut res = vec![vec!['*'; n]; m];
    for r in 0..n {
        for c in 0..s[r].len() {
            res[c][n - 1 - r] = s[r][c];
        }
    }

    for i in 0..m {
        while res[i].len() > 0 && res[i].last().unwrap() == &'*' {
            res[i].pop();
        }
        println!("{}", join(&res[i], ""));
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
