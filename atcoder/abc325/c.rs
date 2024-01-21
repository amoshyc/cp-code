#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let mut vis = vec![vec![false; w]; h];
    let mut ans = 0;
    for root_r in 0..h {
        for root_c in 0..w {
            if vis[root_r][root_c] || arr[root_r][root_c] == '.' {
                continue;
            }

            ans += 1;
            let mut que = std::collections::VecDeque::new();
            vis[root_r][root_c] = true;
            que.push_back((root_r, root_c));

            while let Some((r, c)) = que.pop_front() {
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        let nr = r.checked_add_signed(dr).unwrap_or(h);
                        let nc = c.checked_add_signed(dc).unwrap_or(w);
                        if nr < h && nc < w && arr[nr][nc] == '#' && !vis[nr][nc] {
                            vis[nr][nc] = true;
                            que.push_back((nr, nc));
                        }
                    }
                }
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
