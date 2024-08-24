#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (mut a, mut b, mut k) = (inp[0] as usize, inp[1] as usize, inp[2]);
    let n = a + b;

    // comb(i, j) = comb(i - 1, j) + comb(i - 1, j - 1)
    let mut comb = vec![vec![0i64; n + 1]; n + 1];
    comb[0][0] = 1;
    for i in 1..=n {
        comb[i][0] = 1;
        for j in 1..=i {
            comb[i][j] = comb[i - 1][j] + comb[i - 1][j - 1];
        }
    }

    let mut ans = vec![' '; a + b];
    let mut a = a;
    let mut b = b;
    for i in 0..ans.len() {
        if a > 0 && b > 0 {
            let x = comb[a + b - 1][b];
            if x >= k {
                ans[i] = 'a';
                a -= 1;
            } else {
                k -= x;
                ans[i] = 'b';
                b -= 1;
            }
        } else {
            ans[i] = if a > 0 { 'a' } else { 'b' };
        }
    }

    println!("{}", join(&ans, ""));
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
