#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        arr: [i64; n],
    }

    let mut bit = BIT::<i64>::new(n);
    for i in 0..n {
        bit.add(i, arr[i]);
    }

    let mut ans = vec![];
    for _ in 0..q {
        input! { cmd: usize }
        if cmd == 1 {
            input! { x: Usize1 }
            let a = bit.sum(x, x + 1);
            let b = bit.sum(x + 1, x + 2);
            bit.add(x, -a + b);
            bit.add(x + 1, -b + a);
        } else {
            input! { l : Usize1, r: Usize1 }
            ans.push(bit.sum(l, r + 1));
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
