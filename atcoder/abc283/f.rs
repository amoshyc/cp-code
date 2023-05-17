#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    let n = read::<usize>();
    let arr = readv::<i32>();

    let inf = 1_000_000_000;
    let mut xys = arr
        .iter()
        .enumerate()
        .map(|(i, &x)| (i as i32, x as i32))
        .collect::<Vec<_>>();
    let mut dis = vec![vec![inf; 4]; n];

    let op_min = |a: i32, b: i32| std::cmp::min(a, b);
    let op_max = |a: i32, b: i32| std::cmp::max(a, b);

    // ↗
    {
        let mut seg = SegTree::<i32>::new(n + 1, inf, op_min);
        xys.sort_by_key(|&(x, y)| (Reverse(x), Reverse(y)));
        for &(x, y) in xys.iter() {
            let prev = seg.prod(y as usize, n + 1, 0, 0, seg.nn);
            if prev != inf {
                dis[x as usize][0] = prev - (x + y);
            }
            seg.apply(y as usize, x + y, 0, 0, seg.nn);
        }
    }

    // ↖
    {
        let mut seg = SegTree::<i32>::new(n + 1, -inf, op_max);
        xys.sort_by_key(|&(x, y)| (x, Reverse(y)));
        for &(x, y) in xys.iter() {
            let prev = seg.prod(y as usize, n + 1, 0, 0, seg.nn);
            if prev != -inf {
                dis[x as usize][1] = (x - y) - prev;
            }
            seg.apply(y as usize, x - y, 0, 0, seg.nn);
        }
    }
    
    // ↙
    {
        let mut seg = SegTree::<i32>::new(n + 1, -inf, op_max);
        xys.sort_by_key(|&(x, y)| (x, y));
        for &(x, y) in xys.iter() {
            let prev = seg.prod(0, y as usize, 0, 0, seg.nn);
            if prev != -inf {
                dis[x as usize][2] = (x + y) - prev;
            }
            seg.apply(y as usize, x + y, 0, 0, seg.nn);
        }
    }
    
    // ↘
    {
        let mut seg = SegTree::<i32>::new(n + 1, inf, op_min);
        xys.sort_by_key(|&(x, y)| (Reverse(x), y));
        for &(x, y) in xys.iter() {
            let prev = seg.prod(0, y as usize, 0, 0, seg.nn);
            if prev != inf {
                dis[x as usize][3] = prev - (x - y);
            }
            seg.apply(y as usize, x - y, 0, 0, seg.nn);
        }
    }

    let mut ans = vec![inf; n];
    for i in 0..n {
        ans[i] = *dis[i].iter().min().unwrap();
    }

    println!("{}", join(&ans, " "));
}

struct SegTree<S> {
    nn: usize,
    data: Vec<S>,
    default_data: S,
    op: fn(S, S) -> S,
}

impl<S: Copy + PartialEq> SegTree<S> {
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

    fn prod(&mut self, a: usize, b: usize, u: usize, l: usize, r: usize) -> S {
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
            self.prod(a, b, 2 * u + 1, l, m),
            self.prod(a, b, 2 * u + 2, m, r),
        )
    }

    fn apply(&mut self, i: usize, x: S, u: usize, l: usize, r: usize) {
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
        self.apply(i, x, lch, l, m);
        self.apply(i, x, rch, m, r);
        self.data[u] = (self.op)(self.data[lch], self.data[rch]);
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
