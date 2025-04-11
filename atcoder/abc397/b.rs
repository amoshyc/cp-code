#![allow(unused)]

fn main() {
    let s = reads();
    let mut ans = 0;
    let mut t = vec![];
    let mut i = 0;

    while i < s.len() {
        if t.len() % 2 == 0 {
            if s[i] != 'i' {
                ans += 1;
                t.push('i');
            } else {
                t.push(s[i]);
                i += 1;
            }
        } else {
            if s[i] != 'o' {
                ans += 1;
                t.push('o');
            } else {
                t.push(s[i]);
                i += 1;
            }
        }
    }

    if t.len() % 2 == 1 {
        ans += 1;
        t.push('o');
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
