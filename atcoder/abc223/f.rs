#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let mut s = reads();

    let mut p = vec![0 as i32; n];
    p[0] = if s[0] == '(' { 1 } else { -1 };
    for i in 1..n {
        p[i] = p[i - 1] + if s[i] == '(' { 1 } else { -1 };
    }

    let mut seg = SegTree::<Node>::from_vec(&p);

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (cmd, l, r) = (inp[0], inp[1] - 1, inp[2] - 1);

        if cmd == 1 {
            if s[l] == '(' && s[r] == ')' {
                seg.set(l, r, -2, 0, 0, seg.nn);
                s.swap(l, r);
            } else if s[l] == ')' && s[r] == '(' {
                seg.set(l, r, 2, 0, 0, seg.nn);
                s.swap(l, r);
            }
        } else {
            let ok1 = s[l] == '(' && s[r] == ')';
            let ok2 = seg.get(r, r + 1, 0, 0, seg.nn)
                == if l > 0 {
                    seg.get(l - 1, l, 0, 0, seg.nn)
                } else {
                    0
                };
            let ok3 = seg.get(l, r + 1, 0, 0, seg.nn) == seg.get(l, l + 1, 0, 0, seg.nn) - 1;

            if ok1 && ok2 && ok3 {
                ans.push("Yes");
            } else {
                ans.push("No");
            }
        }
    }
    println!("{}", join(&ans, "\n"));
}

struct Node;
impl SegTrait for Node {
    type S = i32;
    type F = i32;
    fn default_data() -> Self::S {
        0x3f3f3f3f
    }
    fn default_lazy() -> Self::F {
        0
    }
    fn op(a: Self::S, b: Self::S) -> Self::S {
        a.min(b)
    }
    fn apply_lazy(lazy: Self::F, data: Self::S, u: usize, l: usize, r: usize) -> Self::S {
        (lazy as Self::S) + data
    }
    fn merge_lazy(parent: Self::F, child: Self::F) -> Self::F {
        parent + child
    }
}

trait SegTrait {
    type S: Clone;
    type F: Clone + PartialEq;
    fn default_data() -> Self::S;
    fn default_lazy() -> Self::F;
    fn op(a: Self::S, b: Self::S) -> Self::S;
    fn apply_lazy(lazy: Self::F, data: Self::S, u: usize, l: usize, r: usize) -> Self::S;
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
            self.data[lch] = T::apply_lazy(self.lazy[u].clone(), self.data[lch].clone(), lch, l, m);
            self.data[rch] = T::apply_lazy(self.lazy[u].clone(), self.data[rch].clone(), rch, m, r);
            self.lazy[lch] = T::merge_lazy(self.lazy[u].clone(), self.lazy[lch].clone());
            self.lazy[rch] = T::merge_lazy(self.lazy[u].clone(), self.lazy[rch].clone());
            self.lazy[u] = T::default_lazy();
        }
    }

    fn get(&mut self, a: usize, b: usize, u: usize, l: usize, r: usize) -> T::S {
        // l..r has no intersection with a..b
        if l >= b || r <= a {
            return T::default_data();
        }
        // l..r is inside a..b
        if l >= a && r <= b {
            return self.data[u].clone();
        }
        // partially intersect
        let m = (l + r) / 2;
        self.push(u, l, r);
        T::op(
            self.get(a, b, 2 * u + 1, l, m),
            self.get(a, b, 2 * u + 2, m, r),
        )
    }

    fn set(&mut self, a: usize, b: usize, x: T::F, u: usize, l: usize, r: usize) {
        if l >= b || r <= a {
            return;
        }
        // l..r is inside a..b
        if l >= a && r <= b {
            self.data[u] = T::apply_lazy(x.clone(), self.data[u].clone(), u, l, r);
            self.lazy[u] = T::merge_lazy(x.clone(), self.lazy[u].clone());
            return;
        }
        // partially intersect
        let (m, lch, rch) = ((l + r) / 2, 2 * u + 1, 2 * u + 2);
        self.push(u, l, r);
        self.set(a, b, x.clone(), lch, l, m);
        self.set(a, b, x.clone(), rch, m, r);
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
