#![allow(unused)]

fn main() {
    let (h, w) = read2::<usize, usize>();
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    // 01 bfs
    let inf = h * w + 1;
    let mut dis = vec![vec![inf; w]; h];
    let mut que = std::collections::VecDeque::new();

    que.push_back((0_usize, 0_usize));
    dis[0][0] = 0;

    while let Some((r, c)) = que.pop_front() {
        // w = 0 edges
        for &(dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)].iter() {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr < h && nc < w && arr[nr][nc] == '.' && dis[r][c] < dis[nr][nc] {
                que.push_front((nr, nc));
                dis[nr][nc] = dis[r][c];
            }
        }

        // w = 1 edges
        let r1 = if r >= 2 { r - 2 } else { 0 };
        let r2 = if r + 2 < h { r + 2 } else { h - 1 };
        let c1 = if c >= 2 { c - 2 } else { 0 };
        let c2 = if c + 2 < w { c + 2 } else { w - 1 };

        for a in r1..=r2 {
            for b in c1..=c2 {
                if (a, b) == (r, c) {
                    continue;
                }

                let ok1 = a.max(r) - a.min(r) + b.max(c) - b.min(c) <= 3;
                let ok2 = dis[r][c] + 1 < dis[a][b];

                if ok1 && ok2 {
                    que.push_back((a, b));
                    dis[a][b] = dis[r][c] + 1;
                }
            }
        }

        // for r in 0..h {
        //     println!("{}", join(&dis[r], " "));
        // }
        // println!();
    }

    println!("{}", dis[h - 1][w - 1]);
}


fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read2<T: std::str::FromStr, F: std::str::FromStr>() -> (T, F) {
    let mut inp = read::<String>();
    let mut token = inp.split_ascii_whitespace();
    let a: T = token.next().unwrap().parse().ok().unwrap();
    let b: F = token.next().unwrap().parse().ok().unwrap();
    (a, b)
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
