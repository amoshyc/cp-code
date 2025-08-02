#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    let mut ans = vec![0; q];
    let mut cnt = vec![0; n];
    for i in 0..q {
        let x = arr[i];
        if x >= 1 {
            cnt[x - 1] += 1;
            ans[i] = x;
        } else {
            let idx = (0..n).min_by_key(|&i| (cnt[i], i)).unwrap();
            cnt[idx] += 1;
            ans[i] = idx + 1;
        }
    }

    println!("{}", join(&ans, " "));
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
