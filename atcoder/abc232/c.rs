#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut edges1 = vec![];
    let mut edges2 = vec![];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        edges1.push((u, v));
    }
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        edges2.push((u, v));
    }

    let mut p = (0..n).collect::<Vec<usize>>();
    loop {
        let mut s1 = std::collections::HashSet::new();
        let mut s2 = std::collections::HashSet::new();
        for &(u, v) in edges1.iter() {
            s1.insert((u.min(v), u.max(v)));
        }
        for &(u, v) in edges2.iter() {
            s2.insert((p[u].min(p[v]), p[u].max(p[v])));
        }
        if s1 == s2 {
            println!("Yes");
            return;
        }

        if next_permutation(&mut p).is_none() {
            break;
        }
    }

    println!("No");
}

fn next_permutation<T: Ord>(arr: &mut [T]) -> Option<()> {
    let k = arr.windows(2).rposition(|w| w[0] < w[1])?;
    let j = arr.iter().rposition(|a| a > &arr[k]).unwrap();
    arr.swap(k, j);
    arr[(k + 1)..].reverse();
    Some(())
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
