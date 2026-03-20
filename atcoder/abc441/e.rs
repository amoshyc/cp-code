#![allow(unused)]

use proconio::{input, marker::Chars, marker::Usize1};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let fa: Vec<usize> = s.iter().map(|&c| (c == 'A') as usize).collect();
    let fb: Vec<usize> = s.iter().map(|&c| (c == 'B') as usize).collect();
    let pa = build(&fa);
    let pb = build(&fb);

    // Case 1: (l >= 1)
    // PA[r] - PA[l - 1] > PB[r] - PB[l - 1]
    // PA[r] - PB[r] > PA[l - 1] - PB[l - 1]
    // for each r, find the number of valid l

    // Case 2: (l = 0)
    // PA[r] > PB[r]

    let mut ans = 0;
    let mut bit = BIT::<usize>::new(2 * n + 1);
    for r in 0..n {
        // Case 1
        ans += bit.sum(0, n + pa[r] - pb[r]);

        // Case 2
        if pa[r] > pb[r] {
            ans += 1;
        }

        bit.add(n + pa[r] - pb[r], 1);
    }

    println!("{ans}");
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

fn build<T>(arr: &[T]) -> Vec<T>
where
    T: Default + Copy + std::ops::Add<Output = T>,
{
    let mut pref = vec![T::default(); arr.len()];
    pref[0] = arr[0];
    for i in 1..arr.len() {
        pref[i] = pref[i - 1] + arr[i];
    }
    pref
}

// i..j
fn query<T>(pref: &[T], i: usize, j: usize) -> T
where
    T: Default + Copy + std::ops::Sub<Output = T>,
{
    if i == j {
        T::default()
    } else if i == 0 {
        pref[j - 1]
    } else {
        pref[j - 1] - pref[i - 1]
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
