#![allow(unused)]

use std::convert::TryFrom;
use std::collections::HashSet;


fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        arr.push(inp);
    }

    let (enter, leave) = (0, 1);

    let mut stack = vec![];
    let mut vis = HashSet::new();
    let mut ans = 0;

    stack.push((leave, 0, 0));
    stack.push((enter, 0, 0));
    vis.insert(arr[0][0]);

    while let Some((act, r, c)) = stack.pop() {
        if act == leave {
            vis.remove(&arr[r][c]);
            if r == n - 1 && c == m - 1 {
                ans += 1;
            }
        } else {
            vis.insert(arr[r][c]);
            for &(dr, dc) in [(1, 0), (0, 1)].iter() {
                let nr = usize::try_from((r as i32) + dr).unwrap_or(n);
                let nc = usize::try_from((c as i32) + dc).unwrap_or(m);
                if nr >= n || nc >= m || vis.contains(&arr[nr][nc]) {
                    continue;
                }
                stack.push((leave, nr, nc));
                stack.push((enter, nr, nc));
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
