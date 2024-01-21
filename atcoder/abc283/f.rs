#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    let n = read::<usize>();
    let arr = readv::<i32>();

    let solve = |xs: &[i32], ys: &[i32]| {
        let mut seg = SegTree::<Node>::new(n + 1);
        let mut dis = vec![Node::default(); n];
        let mut indices = (0..n).collect::<Vec<usize>>();
        indices.sort_by_key(|&i| (Reverse(xs[i]), Reverse(ys[i])));
        for i in indices {
            let prev = seg.get(ys[i] as usize, n + 1, 0, 0, seg.nn);
            if prev != Node::default() {
                dis[i] = prev - (xs[i] + ys[i]);
            }
            seg.set(ys[i] as usize, xs[i] + ys[i], 0, 0, seg.nn);
        }
        dis
    };

    let xs: Vec<i32> = (0..n).map(|i| i as i32).collect();
    let ys: Vec<i32> = arr.clone();
    let dis1 = solve(&xs, &ys);

    let xs: Vec<i32> = (0..n).map(|i| (n - i) as i32).collect();
    let ys: Vec<i32> = arr.clone();
    let dis2 = solve(&xs, &ys);

    let xs: Vec<i32> = (0..n).map(|i| (n - i) as i32).collect();
    let ys: Vec<i32> = arr.iter().map(|&y| n as i32 - y).collect();
    let dis3 = solve(&xs, &ys);

    let xs: Vec<i32> = (0..n).map(|i| i as i32).collect();
    let ys: Vec<i32> = arr.iter().map(|&y| n as i32 - y).collect();
    let dis4 = solve(&xs, &ys);

    let mut ans = vec![Node::default(); n];
    for i in 0..n {
        let cands = vec![dis1[i], dis2[i], dis3[i], dis4[i]];
        ans[i] = *cands.iter().min().unwrap();
    }

    println!("{}", join(&ans, " "));
}

struct Node;
impl SegTrait for Node {
    type S = i32;
    fn default() -> Self::S {
        10i32.pow(9)
    }
    fn op(a: Self::S, b: Self::S) -> Self::S {
        a.min(b)
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
