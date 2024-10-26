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
    let mut i = 0;
    let mut j = 1;
    while i < n {
        let mut t = segs[i].1;
        while j < n && segs[j].0 <= t {
            t = t.max(segs[j].1);
            j += 1;
        }
        ans.push(format!("{} {}", segs[i].0, t));
        i = j;
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
