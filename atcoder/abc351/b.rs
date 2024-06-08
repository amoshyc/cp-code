#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr1 = vec![];
    for _ in 0..n {
        arr1.push(reads());
    }
    let mut arr2 = vec![];
    for _ in 0..n {
        arr2.push(reads());
    }

    for r in 0..n {
        for c in 0..n {
            if arr1[r][c] != arr2[r][c] {
                println!("{} {}", r + 1, c + 1);
                return;
            }
        }
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
