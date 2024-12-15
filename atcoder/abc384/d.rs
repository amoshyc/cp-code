#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, s) = (inp[0] as usize, inp[1]);
    let arr = readv::<i64>();

    let mut pref = vec![arr[0]; n];
    for i in 1..n {
        pref[i] = pref[i - 1] + arr[i];
    }
    let mut suff = vec![arr[n - 1]; n];
    for i in (0..(n - 1)).rev() {
        suff[i] = suff[i + 1] + arr[i];
    }

    let sum = pref[n - 1];

    for i in 0..n {
        pref[i] %= sum;
        suff[i] %= sum;
    }
    pref.sort();

    // suff[i] + k * sum + pref[j] = s
    // k * sum = s - suff[i] - pref[j]
    // (s - suff[i] - pref[j]) mod sum = 0
    for i in 0..n {
        let val = (s % sum + sum - suff[i]) % sum;
        if let Ok(x) = pref.binary_search(&val) {
            println!("Yes");
            return;
        }
    }
    println!("No");
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
