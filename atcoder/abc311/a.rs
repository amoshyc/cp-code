#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let s = reads();

    let mut ok_a = false;
    let mut ok_b = false;
    let mut ok_c = false;
    for i in 0..n {
        if s[i] == 'A' {
            ok_a = true;
        }
        if s[i] == 'B' {
            ok_b = true;
        }
        if s[i] == 'C' {
            ok_c = true;
        }
        if ok_a && ok_b && ok_c {
            println!("{}", i + 1);
            return;
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
