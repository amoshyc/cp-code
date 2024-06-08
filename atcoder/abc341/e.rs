#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let s = reads();

    let mut seq = vec![(0, 0); n];
    for i in 0..n {
        if s[i] == '1' {
            if i % 2 == 0 {
                seq[i].0 = 1;
            } else {
                seq[i].1 = 1;
            }
        }
    }

    let mut seg = SegTree::<Node>::from_vec(&seq);
    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<usize>();
        let (c, l, r) = (ask[0], ask[1] - 1, ask[2]);
        if c == 1 {
            seg.modify(l, r, 1, 0, 0, seg.nn);
        } else {
            let num_even = count_multiple(2, l, r);
            let num_odd = r - l - num_even;
            
            let (cnt1_at_even, cnt1_at_odd) = seg.query(l, r, 0, 0, seg.nn);
            let cnt0_at_even = num_even - cnt1_at_even;
            let cnt0_at_odd = num_odd - cnt1_at_odd;

            let case1 = cnt1_at_even + cnt0_at_odd == r - l;
            let case2 = cnt0_at_even + cnt1_at_odd == r - l;
            if case1 || case2 {
                ans.push("Yes");
            } else {
                ans.push("No");
            }
        }
    }

    println!("{}", join(&ans, "\n"));
}

// Count number of multiple of x in [l, r)
fn count_multiple(x: usize, l: usize, r: usize) -> usize {
    1 + (r - 1) / x - (l + 1) / x
}

struct Node;
impl SegTrait for Node {
    // number of 1 in even positions, number of 1 in odd positions
    type S = (usize, usize);
    // flip?
    type F = usize;
    fn default_data() -> Self::S {
        (0, 0)
    }
    fn default_lazy() -> Self::F {
        0
    }
    fn op(a: Self::S, b: Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }
    fn apply_lazy(lazy: Self::F, data: Self::S, l: usize, r: usize) -> Self::S {
        if lazy == 1 {
            let num_even = count_multiple(2, l, r);
            let num_odd = r - l - num_even;
            (num_even - data.0, num_odd - data.1)
        } else {
            data
        }
    }
    fn merge_lazy(parent: Self::F, child: Self::F) -> Self::F {
        parent ^ child
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
