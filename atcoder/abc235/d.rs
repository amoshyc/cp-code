#![allow(unused)]

fn main() {
    let max = 1111111;

    let inp = readv::<usize>();
    let (a, n) = (inp[0], inp[1]);

    let inf = max;
    let mut dis = vec![inf; max];
    let mut que = std::collections::VecDeque::new();
    dis[1] = 0;
    que.push_back(1);
    while let Some(u) = que.pop_front() {
        if u == n {
            break;
        }

        let v1 = u * a;
        if v1 < max && dis[u] + 1 < dis[v1] {
            dis[v1] = dis[u] + 1;
            que.push_back(v1);
        }

        let mut x = u;
        let mut digits = vec![];
        while x > 0 {
            digits.push(x % 10);
            x /= 10;
        }
        digits.reverse();
        digits.rotate_right(1);
        let mut v2 = 0;
        for d in digits {
            v2 = v2 * 10 + d;
        }
        if u >= 10 && u % 10 != 0 && v2 < max && dis[u] + 1 < dis[v2] {
            dis[v2] = dis[u] + 1;
            que.push_back(v2);
        }
    }

    if dis[n] == inf {
        println!("-1");
    } else {
        println!("{}", dis[n]);
    }
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
