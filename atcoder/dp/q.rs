#![allow(unused)]

fn main() {
    // dp[i] = maximum possible sum of the LIS ending at i
    // dp[i] = max(dp[j] if h[j] <= h[i] for j in 0..i) + a[i]

    let n = read::<usize>();
    let h = readv::<usize>();
    let a = readv::<i64>();

    let mut dp = vec![0; n];
    let mut seg = SegTree::<Node>::new(n);

    dp[0] = a[0];
    seg.set(h[0], dp[0], 0, 0, seg.nn);

    for i in 1..n {
        dp[i] = seg.get(0, h[i] + 1, 0, 0, seg.nn) + a[i];
        if dp[i] > seg.get(h[i], h[i] + 1, 0, 0, seg.nn) {
            seg.set(h[i], dp[i], 0, 0, seg.nn);
        }    
    }

    let ans = *dp.iter().max().unwrap();
    println!("{}", ans);
}

struct Node;
impl SegTrait for Node {
    type S = i64;
    fn default() -> Self::S {
        0
    }
    fn op(a: Self::S, b: Self::S) -> Self::S {
        a.max(b)
    }
}

trait SegTrait {
    type S: Clone;
    fn default() -> Self::S;
    fn op(a: Self::S, b: Self::S) -> Self::S;
}

struct SegTree<T: SegTrait> {
    nn: usize,
    data: Vec<T::S>,
}

impl<T: SegTrait> SegTree<T> {
    fn new(n: usize) -> Self {
        let nn = n.next_power_of_two();
        let data = vec![T::default(); 2 * nn];
        Self { nn, data }
    }

    fn from_vec(arr: &Vec<T::S>) -> Self {
        let nn = arr.len().next_power_of_two();
        let mut data = vec![T::default(); 2 * nn];
        let s = nn - 1;
        let t = s + arr.len();
        data[s..t].clone_from_slice(arr);
        for u in (0..s).rev() {
            data[u] = T::op(data[2 * u + 1].clone(), data[2 * u + 2].clone());
        }
        Self { nn, data }
    }

    fn get(&mut self, a: usize, b: usize, u: usize, l: usize, r: usize) -> T::S {
        // l..r has no intersection with a..b
        if l >= b || r <= a {
            return T::default();
        }
        // l..r is inside a..b
        if l >= a && r <= b {
            return self.data[u].clone();
        }
        // partially intersect
        let m = (l + r) / 2;
        T::op(
            self.get(a, b, 2 * u + 1, l, m),
            self.get(a, b, 2 * u + 2, m, r),
        )
    }

    fn set(&mut self, i: usize, x: T::S, u: usize, l: usize, r: usize) {
        // l..r has no intersection with i..i+1
        if l >= i + 1 || r <= i {
            return;
        }
        // l..r is inside i..i+1
        if l >= i && r <= i + 1 {
            self.data[u] = x;
            return;
        }
        // partially intersect
        let (m, lch, rch) = ((l + r) / 2, 2 * u + 1, 2 * u + 2);
        self.set(i, x.clone(), lch, l, m);
        self.set(i, x.clone(), rch, m, r);
        self.data[u] = T::op(self.data[lch].clone(), self.data[rch].clone());
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
