#![allow(unused)]

fn solve() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(readv::<u64>());
    }

    let mut ans = 0;
    let mut vis = vec![vec![false; m]; n];
    for a in 0..n {
        for b in 0..m {
            if !vis[a][b] && arr[a][b] != 0 {
                vis[a][b] = true;
                let mut cnt = arr[a][b];
                let mut que = std::collections::VecDeque::new();
                que.push_back((a, b));
                while let Some((r, c)) = que.pop_front() {
                    for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let nr = r.checked_add_signed(dr).unwrap_or(!0);
                        let nc = c.checked_add_signed(dc).unwrap_or(!0);
                        if nr < n && nc < m && !vis[nr][nc] && arr[nr][nc] != 0 {
                            vis[nr][nc] = true;
                            que.push_back((nr, nc));
                            cnt += arr[nr][nc];
                        }
                    }
                }
                ans = ans.max(cnt);
            }
        }
    }

    println!("{}", ans);
}

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        solve();
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
