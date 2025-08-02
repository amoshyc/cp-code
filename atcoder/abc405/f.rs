#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    // Reference: [eijirou](https://atcoder.jp/contests/abc405/submissions/65679297)

    let mut events = vec![vec![]; 2 * n];
    for _ in 0..m {
        let ab = readv::<usize>();
        let (a, b) = (ab[0] - 1, ab[1] - 1);
        events[a].push(('d', a, 1)); // +1 at each endpoint
        events[b].push(('d', b, 1)); // +1 at each endpoint
        events[b].push(('d', a, -2)); // Cancel the segment
    }

    let q = read::<usize>();
    for i in 0..q {
        let cd = readv::<usize>();
        let (c, d) = (cd[0] - 1, cd[1] - 1);
        events[d].push(('q', c, i as isize));
    }

    let mut bit = BIT::<isize>::new(2 * n);
    let mut ans = vec![0; q];
    for i in 0..2 * n {
        for &event in &events[i] {
            if event.0 == 'd' {
                // data
                let (_, i, x) = event;
                bit.add(i, x);
            } else {
                // query
                let (_, c, qid) = event;
                ans[qid as usize] = bit.sum(c, i);
            }
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
    std::io::stdin().read_line(&mut s);
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
