#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(reads());
    }

    let mut que = std::collections::VecDeque::new();
    let mut set = std::collections::HashSet::new();
    let mut vis = vec![vec![false; m]; n];
    let sr = 1 as usize;
    let sc = 1 as usize;
    que.push_back((sr, sc));
    set.insert((sr, sc));
    vis[sr][sc] = true;
    while let Some((r, c)) = que.pop_front() {
        for &(dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)].iter() {
            let mut path = vec![];
            let mut nr = r.wrapping_add(dr);
            let mut nc = c.wrapping_add(dc);
            while nr < n && nc < m && arr[nr][nc] == '.' {
                path.push((nr, nc));
                nr = nr.wrapping_add(dr);
                nc = nc.wrapping_add(dc);
            }

            if path.len() > 0 {
                for &(a, b) in path.iter() {
                    vis[a][b] = true;
                }
                let last = path[path.len() - 1];
                if !set.contains(&last) {
                    set.insert(last);
                    que.push_back(last);
                }
            }
        }
    }

    let mut cnt = 0;
    for r in 0..n {
        for c in 0..m {
            if vis[r][c] {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
