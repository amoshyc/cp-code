#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut groups = vec![];
    let mut ans = vec![];
    dfs(0, &mut groups, n, &mut ans);
    ans.sort();
    ans.dedup();
    println!("{}", join(&ans, "\n"));
}

// set partition
fn dfs(i: usize, groups: &mut Vec<Vec<usize>>, n: usize, ans: &mut Vec<String>) {
    if i == n {
        let mut s = vec![' '; n];
        let mut groups = groups.clone();
        groups.sort_by_key(|g| *g.iter().min().unwrap());
        for (i, group) in groups.iter().enumerate() {
            for j in group {
                s[*j] = char::from_u32('a' as u32 + i as u32).unwrap();
            }
        }
        ans.push(join(&s, ""));
        return;
    }

    for j in 0..groups.len() {
        groups[j].push(i);
        dfs(i + 1, groups, n, ans);
        groups[j].pop();
    }

    groups.push(vec![i]);
    dfs(i + 1, groups, n, ans);
    groups.pop();
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
