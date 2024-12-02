#![allow(unused)]

fn main() {
    let s = reads();
    let n = s.len();
    let mut cnt = vec![0; 26];
    for i in 0..n {
        cnt[s[i] as usize - 'a' as usize] += 1;
    }

    let mut ok = true;
    ok &= n % 2 == 0;
    ok &= (1..n).step_by(2).all(|i| s[i] == s[i - 1]);
    ok &= (0..26).all(|c| cnt[c] == 0 || cnt[c] == 2);

    if ok {
        println!("Yes");
    } else {
        println!("No");
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
