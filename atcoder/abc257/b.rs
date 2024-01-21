#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k, q) = (inp[0], inp[1], inp[2]);
    let mut a = readv::<usize>();
    let mut s = std::collections::HashSet::new();
    s.extend(a.iter());
    let l = readv::<usize>();
    
    a.sort();
    for &x in l.iter() {
        if a[x - 1] == n || s.contains(&(a[x - 1] + 1)) {
            continue;
        } else {
            s.remove(&(a[x - 1]));
            s.insert(a[x - 1] + 1);
            a[x - 1] = a[x - 1] + 1;
        }
    }

    println!("{}", join(&a, " "));
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
