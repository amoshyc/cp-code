#![allow(unused)]

struct Bit<T> {
    data: Vec<T>,
}

impl<T> Bit<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::default::Default,
{
    fn new(n: usize) -> Self {
        Self {
            data: vec![T::default(); n + 1],
        }
    }

    fn add(&mut self, idx: usize, val: T) {
        let n = self.data.len() - 1;
        assert!(idx < n);
        let mut i = idx + 1;
        while i <= n {
            self.data[i] = self.data[i] + val;
            i += i & (!i + 1);
        }
    }

    fn prefix(&self, idx: usize) -> T {
        let n = self.data.len() - 1;
        assert!(idx < n);
        let mut res = T::default();
        let mut i = idx + 1;
        while i > 0 {
            res = res + self.data[i];
            i -= i & (!i + 1);
        }
        res
    }

    // l..r
    fn sum(&self, l: usize, r: usize) -> T {
        let n = self.data.len() - 1;
        assert!(l < r && r <= n);
        let val = self.prefix(r - 1);
        if l == 0 {
            val
        } else {
            val - self.prefix(l - 1)
        }
    }
}

fn main() {
    let n = read::<usize>();
    let xs = readv::<u32>();
    let ys = readv::<u32>();

    let mut sx = xs.clone();
    sx.sort();
    sx.dedup();
    let xs = xs
        .iter()
        .map(|&x| sx.binary_search(&x).unwrap())
        .collect::<Vec<_>>();

    let mut sy = ys.clone();
    sy.sort();
    sy.dedup();
    let ys = ys
        .iter()
        .map(|&y| sy.binary_search(&y).unwrap())
        .collect::<Vec<_>>();

    let mut bit = Bit::<usize>::new(n);

    let mut xy = xs.into_iter().zip(ys.into_iter()).collect::<Vec<_>>();
    xy.sort_by_key(|&(x, y)| (x, std::cmp::Reverse(y)));

    let mut ans = 0;

    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && xy[j] == xy[i] {
            j += 1;
        }
        
        // xs[i..j] are same points
        let cnt = j - i; 
        let (x, y) = xy[i];
        bit.add(y, cnt);
        ans += bit.sum(y, n) * cnt;

        i = j;
    }

    println!("{}", ans);
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
