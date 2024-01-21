#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    let n = read::<usize>();
    let mut pts = vec![];
    let mut ys = vec![];
    for _ in 0..n {
        let mut inp = readv::<i64>();
        inp.sort();
        let (x, y, z) = (inp[0], inp[1], inp[2]);
        pts.push((x, y, z));
        ys.push(y);
    }
    ys.sort();
    ys.dedup();

    pts.sort_by_key(|&(x, y, z)| (x, Reverse(y), z));
    let mut seg = SegTree::new(ys.len(), 10i64.pow(10), |a, b| a.min(b));
    for &(x, y, z) in pts.iter() {
        let y = ys.binary_search(&y).unwrap();
        if seg.get(0, y, 0, 0, seg.nn) < z {
            println!("Yes");
            return;
        }
        if z < seg.get(y, y + 1, 0, 0, seg.nn) {
            seg.set(y, z, 0, 0, seg.nn);
        }
    }

    println!("No");
}


struct SegTree<S> {
    nn: usize,
    data: Vec<S>,
    default_data: S,
    op: fn(S, S) -> S,
}

impl<S: Copy> SegTree<S> {
    fn new(n: usize, default_data: S, op: fn(S, S) -> S) -> SegTree<S> {
        let nn = n.next_power_of_two();
        let data = vec![default_data; 2 * nn];
        Self {
            nn,
            data,
            default_data,
            op,
        }
    }

    fn init(&mut self, arr: &[S]) {
        let s = self.nn - 1;
        let t = s + arr.len();
        self.data[s..t].clone_from_slice(arr);
        for u in (0..(self.nn - 1)).rev() {
            self.data[u] = (self.op)(self.data[2 * u + 1], self.data[2 * u + 2]);
        }
    }

    fn get(&self, a: usize, b: usize, u: usize, l: usize, r: usize) -> S {
        // l..r has no intersection with a..b
        if l >= b || r <= a {
            return self.default_data;
        }
        // l..r is inside a..b
        if l >= a && r <= b {
            return self.data[u];
        }
        // partially intersect
        let m = (l + r) / 2;
        (self.op)(
            self.get(a, b, 2 * u + 1, l, m),
            self.get(a, b, 2 * u + 2, m, r),
        )
    }

    fn set(&mut self, i: usize, x: S, u: usize, l: usize, r: usize) {
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
        self.set(i, x, lch, l, m);
        self.set(i, x, rch, m, r);
        self.data[u] = (self.op)(self.data[lch], self.data[rch]);
    }
}

impl<S: Copy + std::fmt::Debug> std::fmt::Display for SegTree<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let arr = (0..self.nn)
            .map(|i| self.get(i, i + 1, 0, 0, self.nn))
            .collect::<Vec<_>>();
        write!(f, "{:?}", arr)
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
