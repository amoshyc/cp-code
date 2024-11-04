#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w, k) = (inp[0], inp[1], inp[2]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let mut sum = 0;
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == '.' {
                let mut vis = vec![vec![false; w]; h];
                let mut cnt = 0;
                vis[r][c] = true;

                dfs(r, c, k, &mut vis, &mut cnt, &arr);
                sum += cnt;
            }
        }
    }

    println!("{}", sum);
}

fn dfs(
    r: usize,
    c: usize,
    k: usize,
    vis: &mut Vec<Vec<bool>>,
    ans: &mut usize,
    arr: &Vec<Vec<char>>,
) {
    if k == 0 {
        *ans += 1;
        return;
    }

    let h = arr.len();
    let w = arr[0].len();
    for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let nr = r.checked_add_signed(dr).unwrap_or(!0);
        let nc = c.checked_add_signed(dc).unwrap_or(!0);
        if nr < h && nc < w && arr[nr][nc] == '.' && !vis[nr][nc] {
            vis[nr][nc] = true;
            dfs(nr, nc, k - 1, vis, ans, arr);
            vis[nr][nc] = false;
        }
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
