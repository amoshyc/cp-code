#![allow(unused)]

use std::collections::VecDeque;

fn solve() -> String {
    let inp = readv::<i64>();
    let (n, k) = (inp[0] as usize, inp[1]);
    let arr = readv::<i64>();

    let mut ok = |m: usize| -> bool {
        let mut que = VecDeque::from_iter(arr.iter().cloned());

        let mut att = 0;
        let mut cnt = 0;
        let mut is_head = true;
        for _ in 0..m {
            if que.len() == 1 {
                att += que[0];
                break;
            }

            let h = que.pop_front().unwrap();
            let t = que.pop_back().unwrap();

            if is_head {
                if h <= t {
                    que.push_back(t - h + 1);
                    att += 2 * h - 1;
                    is_head = false;
                } else {
                    que.push_front(h - t);
                    att += 2 * t;
                }
            } else {
                if t <= h {
                    que.push_front(h - t + 1);
                    att += 2 * t - 1;
                    is_head = true;
                } else {
                    que.push_back(t - h);
                    att += 2 * h;
                }
            }
        }
        att <= k
    };

    // 1 1 1 0 0 0
    let mut lb = 0;
    let mut ub = n;
    if ok(ub) {
        return ub.to_string();
    }
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            lb = m;
        } else {
            ub = m;
        }
    }

    lb.to_string()
}

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![];
    for _ in 0..tc {
        ans.push(solve());
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
