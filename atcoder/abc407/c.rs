#![allow(unused)]

fn main() {
    let s = reads();
    let n = s.len();
    let arr = mapv(&s, |&c| c as usize - '0' as usize);

    let mut suff = 0;
    let mut ans = n;
    for i in (0..n).rev() {
        // x + suff = arr[i] (mod 10)
        let x = (arr[i] + 10 * n - suff) % 10;
        ans += x;
        suff += x;
    }

    println!("{}", ans);
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
