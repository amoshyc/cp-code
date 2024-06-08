#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let arr = readv::<usize>();
    let arr = mapv(&arr, |&x| x - 1);

    let mut seg = SegTree::<Node>::new(n);
    let mut tour = 0;

    for w in arr.windows(2) {
        let (a, b) = (w[0], w[1]);
        let route1 = a.max(b) - a.min(b);
        let route2 = n - route1;
        if route1 <= route2 {
            // direct
            seg.modify(a.min(b), a.max(b), (route1 as i64, 1), 0, 0, seg.nn);
            tour += route1 as i64;
        } else {
            // indirect
            seg.modify(0, a.min(b), (route2 as i64, 1), 0, 0, seg.nn);
            seg.modify(a.max(b), n + 1, (route2 as i64, 1), 0, 0, seg.nn);
            tour += route2 as i64;
        }
    }

    let mut ans = 10i64.pow(12);
    for i in 0..n {
        let (total, count) = seg.query(i, i + 1, 0, 0, seg.nn);
        let value = tour + count * (n as i64) - 2 * total;
        ans = ans.min(value);
    }

    println!("{}", ans);
}

struct Node;
impl SegTrait for Node {
    type S = (i64, i64);
    type F = (i64, i64);
    fn default_data() -> Self::S {
        (0, 0)
    }
    fn default_lazy() -> Self::F {
        (0, 0)
    }
    fn op(a: Self::S, b: Self::S) -> Self::S {
        let (x, y) = a;
        let (w, z) = b;
        (x + w, y + z)
    }
    fn apply_lazy(lazy: Self::F, data: Self::S, l: usize, r: usize) -> Self::S {
        let (x, y) = data;
        let (w, z) = lazy;
        (x + w, y + z)
    }
    fn merge_lazy(parent: Self::F, child: Self::F) -> Self::F {
        let (x, y) = parent;
        let (w, z) = child;
        (x + w, y + z)
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
