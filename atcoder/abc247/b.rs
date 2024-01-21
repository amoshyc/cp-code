#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut s = vec![];
    let mut t = vec![];
    for _ in 0..n {
        let inp = readv::<String>();
        s.push(inp[0].clone());
        t.push(inp[1].clone());
    }

    for i in 0..n {
        let mut ok = false;
        for x in vec![s[i].clone(), t[i].clone()] {
            if (0..n).filter(|&j| j != i).all(|j| x != s[j] && x != t[j]) {
                ok = true;
            }
        }
        if !ok {
            println!("No");
            return;
        }
    }

    println!("Yes");
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
