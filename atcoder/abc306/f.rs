#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut xs: Vec<i64> = vec![];
    let mut arr = vec![];
    for _ in 0..n {
        let row = readv::<i64>();
        xs.extend(row.iter());
        arr.push(row);
    }

    // compress
    xs.sort();
    xs.dedup();
    let mut sets = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            sets[i][j] = xs.binary_search(&arr[i][j]).unwrap();
        }
    }

    // compute
    let mut bit = BIT::new(xs.len());
    let mut ans = 0 as i64;
    ans += ((n - 1) * n / 2 * (1 + m) * m / 2) as i64;
    for i in (0..n).rev() {
        for &s in sets[i].iter() {
            ans += bit.sum(0, s);
        }
        for &s in sets[i].iter() {
            bit.add(s, 1);
        }
    }

    println!("{}", ans);
}

struct BIT {
    dat: Vec<i64>,
}

impl BIT {
    fn new(n: usize) -> Self {
        Self {
            dat: vec![0; n + 1],
        }
    }

    // 0-based
    fn add(&mut self, mut i: usize, x: i64) {
        i += 1;
        while i < self.dat.len() {
            self.dat[i] += x;
            i += i & (!i + 1); // i & (-i)
        }
    }

    // 1..=i, 1-based
    fn pref(&self, mut i: usize) -> i64 {
        let mut res = 0;
        i += 1;
        while i > 0 {
            res += self.dat[i];
            i -= i & (!i + 1);
        }
        res
    }

    // l..i, 0-based
    fn sum(&self, mut l: usize, mut r: usize) -> i64 {
        if r == 0 {
            0
        } else if l >= 1 {
            self.pref(r - 1) - self.pref(l - 1)
        } else {
            self.pref(r - 1)
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
