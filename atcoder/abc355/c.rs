#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, t) = (inp[0], inp[1]);
    let mut rows = vec![0; n];
    let mut cols = vec![0; n];
    let mut diag1 = 0;
    let mut diag2 = 0;

    let ask = readv::<usize>();
    for i in 0..t {
        let x = ask[i] - 1;
        let r = x / n;
        let c = x % n;

        rows[r] += 1;
        cols[c] += 1;
        if r == c {
            diag1 += 1;
        }
        if r == n - 1 - c {
            diag2 += 1;
        }

        if rows[r] == n || cols[c] == n || diag1 == n || diag2 == n {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
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
