#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut x = vec![];
    let mut y = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        x.push(inp[0]);
        y.push(inp[1]);
    }

    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let l = ((x[i] - x[j]).pow(2) + (y[i] - y[j]).pow(2));
            ans = ans.max(l);
        }
    }

    println!("{:.7}", (ans as f64).sqrt());
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
