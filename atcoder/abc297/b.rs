#![allow(unused)]

fn main() {
    let s = reads();

    let mut b = vec![];
    let mut r = vec![];
    let mut k = 0;
    for (i, &c) in s.iter().enumerate() {
        if c == 'B' {
            b.push(i);
        }
        if c == 'K' {
            k = i;
        }
        if c == 'R' {
            r.push(i);
        }
    }

    let ok1 = (b[0] % 2) ^ (b[1] % 2) == 1;
    let ok2 = r[0] < k && k < r[1];
    if ok1 && ok2 {
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
