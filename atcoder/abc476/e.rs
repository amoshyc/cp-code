#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        perm: [usize; n],
        ask: [(Usize1, Usize1); m],
    }

    let arr = (0..n).map(|i| (perm[i], perm[i], i, i)).collect();
    let mut seg = SegTree::<Node>::from_vec(&arr);
    let ans = (0..n)
        .map(|i| seg.get(i, i + 1, 0, 0, seg.nn).0)
        .collect::<Vec<_>>();

    for &(l, r) in &ask {
        let (min_val, max_val, min_idx, max_idx) = seg.get(l, r + 1, 0, 0, seg.nn);
        // println!("{:?} {:?} {:?}", (l, r), (min_val, max_val), (min_idx, max_idx));
        seg.set(min_idx, (max_val, max_val, min_idx, min_idx), 0, 0, seg.nn);
        seg.set(max_idx, (min_val, min_val, max_idx, max_idx), 0, 0, seg.nn);
    }

    let ans = (0..n)
        .map(|i| seg.get(i, i + 1, 0, 0, seg.nn).0)
        .collect::<Vec<_>>();
    println!("{}", join(&ans, " "));
}

struct Node;
impl SegTrait for Node {
    type S = (usize, usize, usize, usize); // min_val, max_val, min_idx, max_idx
    fn default() -> Self::S {
        (!0, 0, !0, !0)
    }
    fn op(a: Self::S, b: Self::S) -> Self::S {
        let (min_a, max_a, idx1_a, idx2_a) = a;
        let (min_b, max_b, idx1_b, idx2_b) = b;

        let (min_val, min_idx) = if min_a < min_b {
            (min_a, idx1_a)
        } else {
            (min_b, idx1_b)
        };
        let (max_val, max_idx) = if max_a > max_b {
            (max_a, idx2_a)
        } else {
            (max_b, idx2_b)
        };

        (min_val, max_val, min_idx, max_idx)
    }
}

trait SegTrait {
    type S: Clone + std::fmt::Debug;
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
        let n = arr.len();
        let nn = n.next_power_of_two();
        let mut data = vec![T::default(); 2 * nn];
        data[(nn - 1)..(nn - 1 + n)].clone_from_slice(arr);
        for u in (0..(nn - 1)).rev() {
            data[u] = T::op(data[2 * u + 1].clone(), data[2 * u + 2].clone());
        }
        Self { nn, data }
    }

    fn get(&mut self, a: usize, b: usize, u: usize, l: usize, r: usize) -> T::S {
        if l >= b || r <= a {
            return T::default();
        }
        if l >= a && r <= b {
            return self.data[u].clone();
        }
        let m = (l + r) / 2;
        T::op(
            self.get(a, b, 2 * u + 1, l, m),
            self.get(a, b, 2 * u + 2, m, r),
        )
    }

    fn set(&mut self, i: usize, x: T::S, u: usize, l: usize, r: usize) {
        if l >= i + 1 || r <= i {
            return;
        }
        if l >= i && r <= i + 1 {
            self.data[u] = x;
            return;
        }
        let (m, lch, rch) = ((l + r) / 2, 2 * u + 1, 2 * u + 2);
        self.set(i, x.clone(), lch, l, m);
        self.set(i, x.clone(), rch, m, r);
        self.data[u] = T::op(self.data[lch].clone(), self.data[rch].clone());
    }

    // 0 0 0 1 1 1
    //       ^
    fn first_of<P: Fn(T::S, T::S, T::S) -> bool>(
        &self,
        ok: &P,
        pref: T::S,
        suff: T::S,
        u: usize,
        l: usize,
        r: usize,
    ) -> Option<usize> {
        if !ok(
            self.data[u].clone(),
            T::op(pref.clone(), self.data[u].clone()),
            T::op(self.data[u].clone(), suff.clone()),
        ) {
            return None;
        }
        if r - l == 1 {
            return Some(l);
        }
        let (m, lch, rch) = ((l + r) / 2, 2 * u + 1, 2 * u + 2);
        let new_suff = T::op(self.data[rch].clone(), suff.clone());
        if let Some(i) = self.first_of(ok, pref.clone(), new_suff, lch, l, m) {
            return Some(i);
        }
        let new_pref = T::op(pref.clone(), self.data[lch].clone());
        if let Some(i) = self.first_of(ok, new_pref, suff.clone(), rch, m, r) {
            return Some(i);
        }
        None
    }

    fn show(&self, u: usize, dep: usize) {
        if u >= 2 * self.nn - 1 {
            return;
        }
        println!("{}{:?}", " ".repeat(dep * 2), self.data[u]);
        self.show(2 * u + 1, dep + 1);
        self.show(2 * u + 2, dep + 1);
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
