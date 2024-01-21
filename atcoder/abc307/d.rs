#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let s = reads();

    let mut pos = vec![];
    let mut stk = vec![];
    for (i, &c) in s.iter().enumerate() {
        if c == '(' {
            stk.push((i, c));
            pos.push(i);
        } else if c == ')' {
            if let Some(j) = pos.pop() {
                while let Some((i, c)) = stk.pop() {
                    if i == j {
                        break;
                    }
                }
            } else {
                stk.push((i, c));
            }
        } else {
            stk.push((i, c));
        }
    }

    let ans: Vec<char> = stk.iter().map(|&(i, c)| c).collect();
    println!("{}", join(&ans, ""));
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
