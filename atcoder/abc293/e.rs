#![allow(unused)]

fn main() {
    // S(k - 1) = A^0 + A^1 + ... + A^(k-2)
    // S(k) = A^0 + A^1 + ... + A^(k-2) + A^(k-1)
    //      = A * S(k - 1) + 1

    // [ S(k) ] = [ A   1 ] [ S(k-1) ]
    // [  1   ] = [ 0   1 ] [   1    ]

    let inp = readv::<u64>();
    let (a, x, m) = (inp[0], inp[1], inp[2]);

    let mat = vec![vec![a, 1], vec![0, 1]];
    let mat = matpow(&mat, x - 1, m);
    let ans = (mat[0][0] + mat[0][1]) % m;
    println!("{}", ans);
}

type Mat = Vec<Vec<u64>>;

fn matmul(a: &Mat, b: &Mat, m: u64) -> Mat {
    let (h, w) = (a.len(), b[0].len());
    let mut res = vec![vec![0; w]; h];
    for r in 0..h {
        for c in 0..w {
            for k in 0..b.len() {
                res[r][c] += (a[r][k] * b[k][c]) % m;
                res[r][c] %= m;
            }
        }
    }
    res
}

fn matpow(mat: &Mat, mut exp: u64, m: u64) -> Mat {
    let n = mat.len();
    let mut base = mat.clone();
    let mut ans = vec![vec![0; n]; n];
    for i in 0..n {
        ans[i][i] = 1;
    }
    while exp > 0 {
        if exp & 1 == 1 {
            ans = matmul(&ans, &base, m);
        }
        base = matmul(&base, &base, m);
        exp >>= 1;
    }
    ans
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
