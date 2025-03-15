#![allow(unused)]

fn main() {
    // v: A B C D
    // p: 0 0 1 0
    //      ↓
    // q: 3 1 2 0
    // -----------
    //    D B C A

    // For p[i] to transform to q[i], it increases when it encouters a value <= it while scanning from left to right.
    // This is difficult to handle, therefore we process p in reverse.

    // Note that the final positions of each value (i.e., q) is a permutation of 0 ~ (n - 1)
    // We maintain the available positions in a container C while inspecting p from right to left.
    // After processing p[i], we will remove q[i] from C.

    // q[i] is simply C[p[i]]

    // available positions C: {0, 1, 2, 3}
    // v: A B C D
    // p: 0 0 1 0
    // q:       ?
    //          ^
    //         C[0]
    // -----------
    //    D _ _ _

    // available positions C: {1, 2, 3}
    // v: A B C D
    // p: 0 0 1 2
    // q:     ? 0
    //        ^
    //       C[1]
    // -----------
    //    D _ C _

    // available positions C: {1, 3}
    // v: A B C D
    // p: 0 0 1 2
    // q:   ? 2 0
    //      ^
    //     C[0]
    // -----------
    //    D B C _

    // available positions C: {3}
    // v: A B C D
    // p: 0 0 1 2
    // q: ? 1 2 0
    //    ^
    //   C[0]
    // -----------
    //    D B C A

    // Such container can be implemented via SortedSet if using python:
    // ref: https://atcoder.jp/contests/abc392/submissions/62533022

    // For rust/cpp we can use SegTree/BIT with binary search:
    // For each position, maintain if it is available(1) or not(0).
    // To find the k-th smallest (1-based) in the container, we binary search on the positions.
    // Find the position who has prefix sum = k

    let n = read::<usize>();
    let p = mapv(&readv::<usize>(), |&x| x - 1);

    let mut ans = vec![0; n];
    let mut seg = SegTree::<Node>::from_vec(&vec![1; n]);
    for i in (0..n).rev() {
        let j = seg
            .first_of(&|data, pref, suff| pref >= p[i] + 1, 0, 0, 0, 0, seg.nn)
            .unwrap();
        ans[j] = i + 1;
        seg.set(j, 0, 0, 0, seg.nn);
    }

    println!("{}", join(&ans, " "));
}

struct Node;
impl SegTrait for Node {
    type S = usize;
    fn default() -> Self::S {
        0
    }
    fn op(a: Self::S, b: Self::S) -> Self::S {
        a + b
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
