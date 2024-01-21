#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut s = vec![];
    for _ in 0..n {
        s.push(reads());
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut t: Vec<char> = vec![];
            t.extend(s[i].iter());
            t.extend(s[j].iter());
            let mut r = t.clone();
            r.reverse();
            if t == r {
                println!("Yes");
                return;
            }
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
