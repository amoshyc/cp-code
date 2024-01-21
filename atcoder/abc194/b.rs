#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        a.push(inp[0]);
        b.push(inp[1]);
    }

    let mut ans = !0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                ans = ans.min(a[i] + b[i]);
            } else {
                ans = ans.min(a[i].max(b[j]));
            }
        }
    }

    println!("{}", ans);
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

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
