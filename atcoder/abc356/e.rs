#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = readv::<usize>();

    let m = *arr.iter().max().unwrap();
    let mut freq = vec![0; m + 1];
    for &x in arr.iter() {
        freq[x] += 1;
    }

    arr.sort();
    arr.dedup();

    let mut bit = BIT::new(arr.len());
    let mut ans = 0;
    for i in (0..arr.len()).rev() {
        let mut j = i + 1;
        while j < arr.len() {
            let v = arr[j] / arr[i];
            let p = arr[j..].partition_point(|x| x / arr[i] <= v) + j;
            // dbg!(arr[i], j, p);
            ans += freq[arr[i]] * v * bit.sum(j, p);
            j = p;
        }
        ans += freq[arr[i]] * (freq[arr[i]] - 1) / 2;

        bit.add(i, freq[arr[i]]);
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
