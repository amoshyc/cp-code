#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let n = read::<usize>();
    let bags = readv::<i64>();

    let mut groups = vec![];
    let mut ans = HashSet::new();
    dfs(0, &mut groups, &bags, &mut ans);
    println!("{}", ans.len());
}

fn dfs(i: usize, groups: &mut Vec<i64>, bags: &Vec<i64>, ans: &mut HashSet<i64>) {
    let n = bags.len();
    if i == n {
        let xor = groups.iter().fold(0, |acc, x| acc ^ x);
        ans.insert(xor);
        return;
    }

    // move bags[i] to one of previous groups
    for j in 0..groups.len() {
        groups[j] += bags[i];
        dfs(i + 1, groups, bags, ans);
        groups[j] -= bags[i];
    }

    // move bags[i] to new group
    groups.push(bags[i]);
    dfs(i + 1, groups, bags, ans);
    groups.pop();
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
