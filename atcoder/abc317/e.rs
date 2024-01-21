#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(reads());
    }

    let mut sr = 0;
    let mut sc = 0;
    let mut gr = 0;
    let mut gc = 0;
    for r in 0..n {
        for c in 0..m {
            if arr[r][c] == 'S' {
                sr = r;
                sc = c;
                arr[r][c] = '.';
            }
            if arr[r][c] == 'G' {
                gr = r;
                gc = c;
                arr[r][c] = '.';
            }
        }
    }

    let mut can = vec![vec![true; m]; n];
    let dirs = vec!['v', '^', '<', '>'];
    let dr = vec![1, -1, 0, 0];
    let dc = vec![0, 0, -1, 1];
    for r in 0..n {
        for c in 0..m {
            if let Some(p) = (0..4).position(|i| dirs[i] == arr[r][c]) {
                can[r][c] = false;
                for i in 1..n.max(m) {
                    let nr = r.checked_add_signed((i as isize) * dr[p]).unwrap_or(n);
                    let nc = c.checked_add_signed((i as isize) * dc[p]).unwrap_or(m);
                    if nr < n && nc < m && arr[nr][nc] == '.' {
                        can[nr][nc] = false;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let inf = 10usize.pow(9);
    let mut dis = vec![vec![inf; m]; n];
    let mut que = std::collections::VecDeque::new();
    que.push_back((sr, sc));
    dis[sr][sc] = 0;

    while let Some((r, c)) = que.pop_front() {
        for i in 0..4 {
            let nr = r.checked_add_signed(dr[i]).unwrap_or(n);
            let nc = c.checked_add_signed(dc[i]).unwrap_or(m);
            if nr < n && nc < m {
                let ok1 = arr[nr][nc] == '.';
                let ok2 = can[nr][nc];
                let ok3 = dis[r][c] + 1 < dis[nr][nc];
                if ok1 && ok2 && ok3 {
                    dis[nr][nc] = dis[r][c] + 1;
                    que.push_back((nr, nc));
                }
            }
        }
    }

    if dis[gr][gc] == inf {
        println!("-1");
    } else {
        println!("{}", dis[gr][gc]);
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
