#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    // f[i] = 1 if arr[i] is the rightmost position of arr[i] else 0.
    // Inspect the queries from left to right (via right boundry) and
    // maintain the f of previous queries in a BIT.
    // Then for query (l, r), the answer is sum(f[l..=r]) = BIT.sum(l..=r).

    let mut asks = vec![vec![]; n];
    for qid in 0..q {
        let ask = readv::<usize>();
        let l = ask[0] - 1;
        let r = ask[1] - 1;
        asks[r].push((l, qid));
    }

    let mut ans = vec![0; q];
    let mut pos = vec![!0; n + 1]; // pos[i] = rightmost position of i
    let mut bit = BIT::new(n); // stores the f
    for r in 0..n {
        // update f
        if pos[arr[r]] != !0 {
            bit.add(pos[arr[r]], -1);
        }
        pos[arr[r]] = r;
        bit.add(pos[arr[r]], 1);

        // find answer
        for (l, qid) in asks[r].iter() {
            ans[*qid] = bit.sum(*l, r + 1);
        }
    }

    println!("{}", join(&ans, "\n"));
}

struct BIT<T> {
    dat: Vec<T>,
}

impl<T: Clone + Default + std::ops::AddAssign + std::ops::Sub<Output = T>> BIT<T> {
    fn new(n: usize) -> Self {
        Self {
            dat: vec![T::default(); n + 1],
        }
    }

    // 0-based
    fn add(&mut self, mut i: usize, x: T) {
        i += 1; // convert to 1-based
        while i < self.dat.len() {
            self.dat[i] += x.clone();
            i += i & (!i + 1); // i & (-i)
        }
    }

    // 0..=i, 0-based
    fn pref(&self, mut i: usize) -> T {
        let mut res = T::default();
        i += 1; // convert to 1-based
        while i > 0 {
            res += self.dat[i].clone();
            i -= i & (!i + 1);
        }
        res
    }

    // l..i, 0-based
    fn sum(&self, mut l: usize, mut r: usize) -> T {
        if r == 0 {
            T::default()
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
