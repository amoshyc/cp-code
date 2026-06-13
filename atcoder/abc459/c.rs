#![allow(unused)]

use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        asks: [(usize, usize); q],
    }

    let mut val = vec![0; n];
    let mut set = BTreeSet::new();
    let mut cnt = BIT::<i64>::new(600_000 + 1);

    for i in 0..n {
        set.insert((0, i));
    }
    cnt.add(0, n as i64);

    let mut ans = vec![];
    for &(cmd, x) in &asks {
        if cmd == 1 {
            let i = x - 1;
            set.remove(&(val[i], i));
            cnt.add(val[i], -1);
            val[i] += 1;
            cnt.add(val[i], 1);
            set.insert((val[i], i));
        } else {
            let (base, _) = set.first().unwrap();
            let q = x + base;
            ans.push(cnt.sum(q, 600_000 + 1));
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

    // l..r, 0-based
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
