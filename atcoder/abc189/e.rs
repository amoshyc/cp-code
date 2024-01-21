#![allow(unused)]

fn main() {
    // R(u) = [[c -s], [s, c]]
    // R(pi / 2) = [[0, -1], [1, 0]]
    // R(-pi / 2) = [[0, 1], [-1, 0]]

    // op 1
    // x' = -y, y' = x
    // [x']   [ 0  1  0 ]  [x]
    // [y'] = [ -1 0  0 ]  [y]
    // [1 ]   [ 0  0  1 ]  [1]

    // op 2
    // x' = y, y' = -x
    // [x']   [ 0  -1  0 ]  [x]
    // [y'] = [ 1  0   0 ]  [y]
    // [1 ]   [ 0  0   1 ]  [1]

    // op 3
    // x' = p - (x - p) = 2p - x, y' = y
    // [x']   [ -1 0 2p ]  [x]
    // [y'] = [ 0  1  0 ]  [y]
    // [1 ]   [ 0  0  1 ]  [1]

    // op 4
    // y' = p - (y - p) = 2p - y, x' = x
    // [x']   [ 1  0  0 ]  [x]
    // [y'] = [ 0 -1 2p ]  [y]
    // [1 ]   [ 0  0  1 ]  [1]

    let n = read::<usize>();
    let mut pts = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        let (x, y) = (inp[0], inp[1]);
        pts.push(vec![vec![x], vec![y], vec![1]]);
    }

    let m = read::<usize>();
    let mut mat = vec![];
    for _ in 0..m {
        let inp = readv::<i64>();
        if inp[0] == 1 {
            mat.push(vec![vec![0, 1, 0], vec![-1, 0, 0], vec![0, 0, 1]]);
        } else if inp[0] == 2 {
            mat.push(vec![vec![0, -1, 0], vec![1, 0, 0], vec![0, 0, 1]]);
        } else if inp[0] == 3 {
            let p = inp[1];
            mat.push(vec![vec![-1, 0, 2 * p], vec![0, 1, 0], vec![0, 0, 1]]);
        } else {
            let p = inp[1];
            mat.push(vec![vec![1, 0, 0], vec![0, -1, 2 * p], vec![0, 0, 1]]);
        }
    }

    let mut pref = vec![];
    pref.push(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
    for i in 0..m {
        // Note the order
        pref.push(matmul(&mat[i], &pref[pref.len() - 1]));
    }

    let mut ans = vec![];
    let q = read::<usize>();
    for _ in 0..q {
        let inp = readv::<usize>();
        let (a, b) = (inp[0], inp[1]);
        let res = matmul(&pref[a], &pts[b - 1]);
        ans.push(format!("{} {}", res[0][0], res[1][0]));
    }

    println!("{}", join(&ans, "\n"));
}

fn matmul(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let (n, m) = (a.len(), b[0].len());
    let mut res = vec![vec![0; m]; n];
    for r in 0..n {
        for c in 0..m {
            for k in 0..a[0].len() {
                res[r][c] += a[r][k] * b[k][c];
            }
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn mapv<T, F>(arr: &Vec<T>, f: fn(&T) -> F) -> Vec<F> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
