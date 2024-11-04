#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, m) = (inp[0] as usize, inp[1]);
    let arr = readv::<i64>();

    let mut pref = vec![0; n];
    pref[0] = arr[0] % m;
    for i in 1..n {
        pref[i] = (pref[i - 1] + arr[i]) % m;
    }

    let mut ans: i64 = 0;
    let mut bit_cnt = BIT::<i64>::new(m as usize);
    let mut bit_sum = BIT::<i64>::new(m as usize);
    for r in 0..n {
        let cnt_lt = bit_cnt.sum(0, pref[r] as usize);
        let sum_lt = bit_sum.sum(0, pref[r] as usize);
        let val_lt = cnt_lt * pref[r] - sum_lt;

        let cnt_gt = bit_cnt.sum(pref[r] as usize + 1, m as usize);
        let sum_gt = bit_sum.sum(pref[r] as usize + 1, m as usize);
        let val_gt = cnt_gt * pref[r] + cnt_gt * m - sum_gt;

        ans += pref[r] as i64;
        ans += val_lt;
        ans += val_gt;

        bit_cnt.add(pref[r] as usize, 1);
        bit_sum.add(pref[r] as usize, pref[r]);
    }

    println!("{}", ans);
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
