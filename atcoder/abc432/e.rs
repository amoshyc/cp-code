#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut arr: [i64; n],
        ask: [(usize, i64, i64); q],
    }

    let mut bit_cnt = BIT::<i64>::new(500_001);
    let mut bit_val = BIT::<i64>::new(500_001);
    for i in 0..n {
        bit_cnt.add(arr[i] as usize, 1);
        bit_val.add(arr[i] as usize, arr[i]);
    }

    let mut ans = vec![];

    for (cmd, x, y) in ask {
        if cmd == 1 {
            let x = x as usize - 1;
            bit_cnt.add(arr[x] as usize, -1);
            bit_val.add(arr[x] as usize, -arr[x]);
            arr[x] = y;
            bit_cnt.add(arr[x] as usize, 1);
            bit_val.add(arr[x] as usize, arr[x]);
        } else {
            let (l, r) = (x, y);
            if l <= r {
                let inside = bit_val.sum(l as usize, r as usize + 1);
                let outside =
                    bit_cnt.sum(0, l as usize) * l + bit_cnt.sum(r as usize + 1, 500_001) * r;
                ans.push(inside + outside);
            } else {
                ans.push(n as i64 * l);
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
