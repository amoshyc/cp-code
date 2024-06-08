#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut segs = vec![];
    for _ in 0..n {
        let seg = readv::<usize>();
        segs.push((seg[0], seg[1]));
    }

    let mut ans = (n as i64) * (n as i64 - 1) / 2;
    segs.sort_by_key(|&(l, r)| (r, l));
    for i in 0..n {
        let (curr_l, curr_r) = segs[i];
        let p = segs.partition_point(|&(prev_l, prev_r)| prev_r < curr_l);
        if p <= i {
            ans -= p as i64;
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
