#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let (mut sr, mut sc) = (0, 0);
    let (mut gr, mut gc) = (0, 0);
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == 'S' {
                (sr, sc) = (r, c);
            }
            if arr[r][c] == 'G' {
                (gr, gc) = (r, c);
            }
        }
    }

    arr[sr][sc] = '.';
    arr[gr][gc] = '.';
    let inf = usize::MAX;
    let drdc1 = [(0, 1), (0, -1)];
    let drdc2 = [(1, 0), (-1, 0)];

    let bfs = |f: usize| -> usize {
        let mut dis = vec![vec![inf; w]; h];
        let mut que = VecDeque::new();
        dis[sr][sc] = 0;
        que.push_back((sr, sc));

        while let Some((r, c)) = que.pop_front() {
            let drdc = if dis[r][c] % 2 == f { drdc1 } else { drdc2 };
            for (dr, dc) in drdc {
                let nr = r.checked_add_signed(dr).unwrap_or(h);
                let nc = c.checked_add_signed(dc).unwrap_or(w);
                if nr < h && nc < w && arr[nr][nc] == '.' && dis[nr][nc] == inf {
                    dis[nr][nc] = dis[r][c] + 1;
                    que.push_back((nr, nc));
                }
            }
        }

        dis[gr][gc]
    };

    let ans = bfs(0).min(bfs(1));
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
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
