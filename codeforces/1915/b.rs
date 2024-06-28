#![allow(unused)]

fn solve() -> String {
    let mut cnt = vec![3; 3];
    for _ in 0..3 {
        let inp = reads();
        for c in inp {
            if let Some(i) = "ABC".chars().position(|x| x == c) {
                cnt[i] -= 1;
            }
        }
    }

    for i in 0..3 {
        if cnt[i] > 0 {
            return "ABC"[i..(i + 1)].to_string();
        }
    }

    "".to_string()
}

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![];
    for _ in 0..tc {
        ans.push(solve());
    }
    println!("{}", ans.join("\n"));
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
