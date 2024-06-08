#![allow(unused)]

// [Problem]
// Please find the number of integers between 2 and N (inclusive) that have at least K distinct prime factors.

// [Solution]
// Variant of Sieve of Eratosthenes
// cnt[i] = number of prime factors of i

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);

    let mut cnt = vec![0; n + 1];
    for i in 2..=n {
        if cnt[i] == 0 {
            for j in (i..=n).step_by(i) {
                cnt[j] += 1;
            }
        }
    }

    let mut ans = 0;
    for i in 2..=n {
        if cnt[i] >= k {
            ans += 1;
        }
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
