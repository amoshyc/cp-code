#![allow(unused)]

fn main() {
    let mut cnt = vec![0; 26];
    let s = reads();
    for &c in s.iter() {
        cnt[c as usize - 'a' as usize] += 1;
    }

    let mut ok = true;
    for i in 1..=s.len() {
        let c = cnt.iter().filter(|&x| *x == i).count();
        ok &= (c == 2 || c == 0);
    }

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
    read::<String>().chars().collect::<_>()
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
