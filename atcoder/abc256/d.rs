#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut segs = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        segs.push((inp[0], inp[1]));
    }

    segs.sort();

    let mut ans = vec![];
    let mut s = segs[0].0;
    let mut t = segs[0].1;
    let mut i = 0;
    let mut j = 1;
    while i < n {
        while j < n && segs[j].0 <= t {
            t = t.max(segs[j].1);
            j += 1;
        }
        ans.push(format!("{} {}", s, t));
        i = j;
        if i < n {
            s = segs[i].0;
            t = segs[i].1;
        }
    }
    println!("{}", join(&ans, "\n"));
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
