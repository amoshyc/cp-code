#![allow(unused)]

fn main() {
    let s = reads();
    let mut stack = vec![];
    for c in s {
        if ['(', '[', '<'].contains(&c) {
            stack.push(c);
        } else {
            if let Some(x) = stack.pop() {
                if [('(', ')'), ('[', ']'), ('<', '>')].contains(&(x, c)) {
                    continue;
                }
            }
            println!("No");
            return;
        }
    }
    if stack.len() == 0 {
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
