#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k, q) = (inp[0], inp[1] as i64, inp[2] as i64);

    let mut cnt = vec![0; n];
    for _ in 0..q {
        let x = read::<usize>() - 1;
        cnt[x] += 1;
    }

    let mut ans = vec![];
    for i in 0..n {
        let s = k - (q - cnt[i]);
        if s <= 0 {
            ans.push("No");
        } else {
            ans.push("Yes");
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
