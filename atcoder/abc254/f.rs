#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let arr_a = readv::<i64>();
    let arr_b = readv::<i64>();

    let mut diff_a = vec![0; n];
    let mut diff_b = vec![0; n];
    for i in 1..n {
        diff_a[i] = (arr_a[i] - arr_a[i - 1]).abs();
        diff_b[i] = (arr_b[i] - arr_b[i - 1]).abs();
    }

    let st_a = SparseTable::new(&diff_a, |a, b| gcd(a, b));
    let st_b = SparseTable::new(&diff_b, |a, b| gcd(a, b));

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (h1, h2, w1, w2) = (inp[0] - 1, inp[1] - 1, inp[2] - 1, inp[3] - 1);
        let mut val = arr_a[h1] + arr_b[w1];
        if h1 != h2 {
            val = gcd(val, st_a.query(h1 + 1, h2 + 1));
        }
        if w1 != w2 {
            val = gcd(val, st_b.query(w1 + 1, w2 + 1));
        }
        ans.push(val);
    }

    println!("{}", join(&ans, "\n"));
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct SparseTable<T> {
    dp: Vec<Vec<T>>,
    op: fn(T, T) -> T,
}

impl<T: Copy> SparseTable<T> {
    fn new(arr: &Vec<T>, op: fn(T, T) -> T) -> Self {
        let nn = (arr.len() as f64).log2() as usize + 1;
        let mut dp = vec![arr.clone(); nn];
        for i in 1..nn {
            for u in 0..=(arr.len() - (1 << i)) {
                dp[i][u] = (op)(dp[i - 1][u], dp[i - 1][u + (1 << (i - 1))]);
            }
        }
        Self { dp, op }
    }

    // a..b
    fn query(&self, a: usize, b: usize) -> T {
        assert!(a != b);
        let k = ((b - a) as f64).log2() as usize;
        (self.op)(self.dp[k][a], self.dp[k][b - (1 << k)])
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
