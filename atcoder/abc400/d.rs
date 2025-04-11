#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }
    let inp = readv::<usize>();
    let (sr, sc) = (inp[0] - 1, inp[1] - 1);
    let (gr, gc) = (inp[2] - 1, inp[3] - 1);

    let inf = h * w + 1;
    let mut dis = vec![vec![inf; w]; h];
    let mut que = VecDeque::new();

    dis[sr][sc] = 0;
    que.push_back((sr, sc, 0));

    // 01-bfs
    while let Some((r, c, d)) = que.pop_front() {
        // (r, c) may be pushed into queue multiple times
        if d > dis[r][c] {
            continue;
        }

        // weight = 0 edges
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nr = r.checked_add_signed(dr).unwrap_or(!0);
            let nc = c.checked_add_signed(dc).unwrap_or(!0);
            if nr < h && nc < w && arr[nr][nc] == '.' && dis[r][c] < dis[nr][nc] {
                dis[nr][nc] = dis[r][c];
                que.push_front((nr, nc, dis[nr][nc]));
            }
        }

        // weight = 1 edges
        for i in 1..=2 {
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nr = r.checked_add_signed(dr * i).unwrap_or(!0);
                let nc = c.checked_add_signed(dc * i).unwrap_or(!0);
                if nr < h && nc < w && arr[nr][nc] == '#' && dis[r][c] + 1 < dis[nr][nc] {
                    dis[nr][nc] = dis[r][c] + 1;
                    que.push_back((nr, nc, dis[nr][nc]));
                }
            }
        }
    }

    println!("{}", dis[gr][gc]);
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
