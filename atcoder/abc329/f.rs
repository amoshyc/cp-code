#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    let mut sets = vec![std::collections::HashSet::new(); n];
    let mut idxs = vec![0; n];

    for i in 0..n {
        sets[i].insert(arr[i]);
        idxs[i] = i;
    }

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (a, b) = (inp[0] - 1, inp[1] - 1);

        if sets[idxs[a]].len() < sets[idxs[b]].len() {
            let src = sets[idxs[a]].clone();
            sets[idxs[b]].extend(src);
            sets[idxs[a]].clear();
        } else {
            let src = sets[idxs[b]].clone();
            sets[idxs[a]].extend(src);
            sets[idxs[b]].clear();
            idxs.swap(a, b);
        }

        ans.push(sets[idxs[b]].len());
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
