#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let xs = readv::<u32>();
    let ys = readv::<u32>();
 
    let (xs, _) = compress(&xs);
    let (ys, _) = compress(&ys);
    let mut xy = xs.into_iter().zip(ys.into_iter()).collect::<Vec<_>>();
    xy.sort_by_key(|&(x, y)| (x, std::cmp::Reverse(y)));
 
    let mut bit = BIT::new(n);
    let mut ans = 0;
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && xy[j] == xy[i] {
            j += 1;
        }
 
        // xy[i..j] are same points
        let cnt = j - i;
        let (x, y) = xy[i];
        bit.add(y, cnt);
        ans += bit.sum(y, n) * cnt;
 
        i = j;
    }
 
    println!("{}", ans);
}

fn compress<T: Clone + Ord>(arr: &[T]) -> (Vec<usize>, Vec<T>) {
    let mut s = arr.to_vec();
    s.sort();
    s.dedup();
    let res = arr
        .iter()
        .map(|x| s.binary_search(x).unwrap())
        .collect::<Vec<_>>();
    (res, s)
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
