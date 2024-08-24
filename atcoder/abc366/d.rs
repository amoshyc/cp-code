#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = vec![vec![vec![0; n]; n]; n];
    for x in 0..n {
        for y in 0..n {
            arr[x][y] = readv::<i64>();
        }
    }

    let mut pref = vec![];
    for x in 0..n {
        pref.push(build_2d(&arr[x]));
    }

    let q = read::<usize>();
    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<usize>();
        let (x1, x2) = (ask[0] - 1, ask[1] - 1);
        let (y1, y2) = (ask[2] - 1, ask[3] - 1);
        let (z1, z2) = (ask[4] - 1, ask[5] - 1);
        let mut res = 0;
        for x in x1..=x2 {
            res += query_2d(&pref[x], y1, y2, z1, z2);
        }
        ans.push(res);
    }

    println!("{}", join(&ans, "\n"));
}

fn build_2d(arr: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let (nr, nc) = (arr.len(), arr[0].len());
    let mut pref = vec![vec![0; nc]; nr];
    let transition = [((-1, 0), 1), ((0, -1), 1), ((-1, -1), -1)];
    for r in 0..nr {
        for c in 0..nc {
            pref[r][c] = arr[r][c];
            for &((dr, dc), s) in transition.iter() {
                let pr = r.checked_add_signed(dr).unwrap_or(nr);
                let py = c.checked_add_signed(dc).unwrap_or(nc);
                if pr < nr && py < nc {
                    pref[r][c] += s * pref[pr][py];
                }
            }
        }
    }
    pref
}

// arr[r1..=r2, c1..=c2]
fn query_2d(pref: &Vec<Vec<i64>>, r1: usize, r2: usize, c1: usize, c2: usize) -> i64 {
    let (nr, nc) = (pref.len(), pref[0].len());
    let r1 = r1.checked_add_signed(-1).unwrap_or(nr);
    let c1 = c1.checked_add_signed(-1).unwrap_or(nc);
    let transition = [((r2, c2), 1), ((r1, c2), -1), ((r2, c1), -1), ((r1, c1), 1)];
    let mut res = 0;
    for ((r, c), s) in transition {
        if r < nr && c < nc {
            res += s * pref[r][c];
        }
    }
    res
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
