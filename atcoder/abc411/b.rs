#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut pos = vec![0; n];
    for i in 1..n {
        pos[i] = pos[i - 1] + arr[i - 1];
    }

    for i in 0..n {
        for j in (i + 1)..n {
            print!("{} ", pos[i].abs_diff(pos[j]));
        }
        println!();
    }
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
