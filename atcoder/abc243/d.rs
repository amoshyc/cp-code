#![allow(unused)]


fn main() {
    let inp = readv::<u64>();
    let (n, x) = (inp[0] as usize, inp[1]);

    let mut stack = vec![];
    let mut y = x;
    while y > 1 {
        if y % 2 == 0 {
            stack.push('L');
        } else {
            stack.push('R');
        }
        y = y / 2;
    }
    stack.reverse();
    
    let s = reads();
    for i in 0..n {
        if s[i] == 'U' {
            if stack.len() > 0 {
                stack.pop();
            }
        } else {
            stack.push(s[i]);
        }
    }

    let mut y = 1 as u64;
    for d in stack {
        if d == 'L' {
            y = y * 2 + 0;
        } else {
            y = y * 2 + 1;
        }
    }

    println!("{}", y);
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
