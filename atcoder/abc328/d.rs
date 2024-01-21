#![allow(unused)]

fn main() {
    let s = reads();
    let mut stack = vec![];
    for i in 0..s.len() {
        stack.push(s[i]);

        let l = stack.len();
        if l >= 3 && stack[l - 1] == 'C' && stack[l - 2] == 'B' && stack[l - 3] == 'A' {
            stack.pop();
            stack.pop();
            stack.pop();
        }
    }
    println!("{}", join(&stack, ""));
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
