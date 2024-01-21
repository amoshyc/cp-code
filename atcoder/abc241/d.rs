#![allow(unused)]


fn main() {
    let q = read::<usize>();
    let mut s = std::collections::BTreeSet::new();
    let mut ans = vec![];

    for qid in 0..q {
        let inp = readv::<i64>();
        if inp[0] == 1 {
            s.insert((inp[1], qid));
        } else if inp[0] == 2 {
            let (x, k) = (inp[1], inp[2] as usize);
            if let Some((y, _)) = s.range(..(x, q)).nth_back(k - 1) {
                ans.push(*y);
            } else {
                ans.push(-1);
            }
        } else {
            let (x, k) = (inp[1], inp[2] as usize);
            if let Some((y, _)) = s.range((x, 0)..).nth(k - 1) {
                ans.push(*y);
            } else {
                ans.push(-1);
            }
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

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
