#![allow(unused)]

// [Problem]
// We are stacking bricks at a horizontal section of length W.
// Initially, the height of thesection is 0. Here, we stack N bricks of height 1 one by one.
// The height of the top surface of the brick adhered to a surface of height h will become h+1.
// The i-th brick to be stacked is placed so that it exactly covers squares from the Li-th to the Ri-th square from the left.
// At this time, it adheres to the highest horizontal surface within the range covered by the brick.
// Please find the height of the top surface for each brick.

// [Solution]
// LazySegTree

fn main() {
    let inp = readv::<usize>();
    let (w, n) = (inp[0], inp[1]);
    let mut ans = vec![];
    let mut seg = SegTree::<Node>::new(w); // stores the height of the section
    for _ in 0..n {
        let lr = readv::<usize>();
        let (l, r) = (lr[0] - 1, lr[1]); // l..r
        let h = seg.query(l, r, 0, 0, seg.nn); // The highest height in l..r
        seg.modify(l, r, h + 1, 0, 0, seg.nn); // Add brick i on top of it
        ans.push(h + 1);
    }
    println!("{}", join(&ans, "\n"));
}

struct Node;
impl SegTrait for Node {
    type S = usize;
    type F = usize;
    fn default_data() -> Self::S {
        0
    }
    fn default_lazy() -> Self::F {
        0
    }
    fn op(a: Self::S, b: Self::S) -> Self::S {
        a.max(b)
    }
    fn apply_lazy(lazy: Self::F, data: Self::S, l: usize, r: usize) -> Self::S {
        lazy
    }
    fn merge_lazy(parent: Self::F, child: Self::F) -> Self::F {
        parent
    }
}

trait SegTrait {
    type S: Clone + PartialEq + std::fmt::Debug;
    type F: Clone + PartialEq + std::fmt::Debug;
    fn default_data() -> Self::S;
    fn default_lazy() -> Self::F;
    fn op(a: Self::S, b: Self::S) -> Self::S;
    fn apply_lazy(lazy: Self::F, data: Self::S, l: usize, r: usize) -> Self::S;
    fn merge_lazy(parent: Self::F, child: Self::F) -> Self::F;
}

struct SegTree<T: SegTrait> {
    nn: usize,
    data: Vec<T::S>,
    lazy: Vec<T::F>,
}

impl<T: SegTrait> SegTree<T> {
    fn new(n: usize) -> Self {
        let nn = n.next_power_of_two();
        let data = vec![T::default_data(); 2 * nn];
        let lazy = vec![T::default_lazy(); 2 * nn];
        Self { nn, data, lazy }
    }

    fn from_vec(arr: &Vec<T::S>) -> Self {
        let nn = arr.len().next_power_of_two();
        let mut data = vec![T::default_data(); 2 * nn];
        let lazy = vec![T::default_lazy(); 2 * nn];
        let s = nn - 1;
        let t = s + arr.len();
        data[s..t].clone_from_slice(arr);
        for u in (0..s).rev() {
            data[u] = T::op(data[2 * u + 1].clone(), data[2 * u + 2].clone());
        }
        Self { nn, data, lazy }
    }

    fn push(&mut self, u: usize, l: usize, r: usize) {
        if self.lazy[u] != T::default_lazy() {
            let (m, lch, rch) = ((l + r) / 2, 2 * u + 1, 2 * u + 2);
            self.data[lch] = T::apply_lazy(self.lazy[u].clone(), self.data[lch].clone(), l, m);
            self.data[rch] = T::apply_lazy(self.lazy[u].clone(), self.data[rch].clone(), m, r);
            self.lazy[lch] = T::merge_lazy(self.lazy[u].clone(), self.lazy[lch].clone());
            self.lazy[rch] = T::merge_lazy(self.lazy[u].clone(), self.lazy[rch].clone());
            self.lazy[u] = T::default_lazy();
        }
    }

    fn query(&mut self, a: usize, b: usize, u: usize, l: usize, r: usize) -> T::S {
        // l..r has no intersection with a..b
        if l >= b || r <= a {
            return T::default_data();
        }
        // l..r is inside a..b
        if l >= a && r <= b {
            return self.data[u].clone();
        }
        // partially intersect
        let (m, lch, rch) = ((l + r) / 2, 2 * u + 1, 2 * u + 2);
        self.push(u, l, r);
        T::op(self.query(a, b, lch, l, m), self.query(a, b, rch, m, r))
    }

    fn modify(&mut self, a: usize, b: usize, x: T::F, u: usize, l: usize, r: usize) {
        // l..r has no intersection with a..b
        if l >= b || r <= a {
            return;
        }
        // l..r is inside a..b
        if l >= a && r <= b {
            self.data[u] = T::apply_lazy(x.clone(), self.data[u].clone(), l, r);
            self.lazy[u] = T::merge_lazy(x.clone(), self.lazy[u].clone());
            return;
        }
        // partially intersect
        let (m, lch, rch) = ((l + r) / 2, 2 * u + 1, 2 * u + 2);
        self.push(u, l, r);
        self.modify(a, b, x.clone(), lch, l, m);
        self.modify(a, b, x.clone(), rch, m, r);
        self.data[u] = T::op(self.data[lch].clone(), self.data[rch].clone());
    }

    fn find_first_of<P: Copy + Fn(T::S) -> bool>(
        &mut self,
        f: P,
        a: usize,
        b: usize,
        u: usize,
        l: usize,
        r: usize,
    ) -> Option<usize> {
        if l >= b || r <= a || f(self.data[u].clone()) {
            return None;
        }
        if r - l == 1 {
            return Some(l);
        }
        let (m, lch, rch) = ((l + r) / 2, 2 * u + 1, 2 * u + 2);
        self.push(u, l, r);
        if let Some(idx) = self.find_first_of(f, a, b, lch, l, m) {
            return Some(idx);
        }
        if let Some(idx) = self.find_first_of(f, a, b, rch, m, r) {
            return Some(idx);
        }
        None
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
