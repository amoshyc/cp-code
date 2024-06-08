#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut events = vec![];
    for i in 0..n {
        let inp = readv::<usize>();
        let (s, t) = (inp[0], inp[1]);
        let (s, t) = (s.min(t), s.max(t));
        events.push((s, 1, '+', i));
        events.push((t + 1, 0, '-', i));
    }
    events.sort();

    let mut stack = vec![];
    for (p, _, t, i) in events {
        if t == '+' {
            stack.push(i);
        } else {
            if stack.len() == 0 || stack[stack.len() - 1] != i {
                println!("Yes");
                return;
            } else {
                stack.pop();
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
