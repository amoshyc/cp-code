fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let arr = readv::<u64>();
    
    let mut seg = SegTree::new(n);
    seg.init(&arr);

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<u64>();
        if inp[0] == 0 {
            let (p, x) = (inp[1] as usize, inp[2]);
            seg.apply(p, p + 1, x, 0, 0, seg.nn);
        } else {
            let (l, r) = (inp[1] as usize, inp[2] as usize);
            ans.push(seg.prod(l, r, 0, 0, seg.nn));
        }
    }
    
    println!("{}", join(&ans, "\n"));
}

type S = u64;
type F = u64;

fn default_data() -> S {
    0
}
fn default_lazy() -> F {
    0
}
fn op(a: S, b: S) -> S {
    a + b
}
fn apply_lazy(lazy: F, data: S, _u: usize, _l: usize, _r: usize) -> S {
    (lazy as S) + data
}
fn merge_lazy(parent: F, child: F) -> F {
    parent + child
}

struct SegTree {
    nn: usize,
    data: Vec<S>,
    lazy: Vec<F>,
}

impl SegTree {
    fn new(n: usize) -> SegTree {
        let nn = n.next_power_of_two();
        SegTree {
            nn,
            data: vec![default_data(); 2 * nn],
            lazy: vec![default_lazy(); 2 * nn],
        }
    }

    fn init(&mut self, arr: &[S]) {
        let s = self.nn - 1;
        let t = s + arr.len();
        self.data[s..t].clone_from_slice(arr);
        for u in (0..(self.nn - 1)).rev() {
            self.data[u] = op(self.data[2 * u + 1], self.data[2 * u + 2]);
        }
    }

    fn push(&mut self, u: usize, l: usize, r: usize) {
        if self.lazy[u] == default_lazy() {
            return;
        }
        let (m, lch, rch) = ((l + r) / 2, 2 * u + 1, 2 * u + 2);
        self.data[lch] = apply_lazy(self.lazy[u], self.data[lch], lch, l, m);
        self.data[rch] = apply_lazy(self.lazy[u], self.data[rch], rch, m, r);
        self.lazy[lch] = merge_lazy(self.lazy[u], self.lazy[lch]);
        self.lazy[rch] = merge_lazy(self.lazy[u], self.lazy[rch]);
        self.lazy[u] = default_lazy();
    }

    fn prod(&mut self, a: usize, b: usize, u: usize, l: usize, r: usize) -> S {
        // l..r has no intersection with a..b
        if l >= b || r <= a {
            return default_data();
        }
        // l..r is inside a..b
        if l >= a && r <= b {
            return self.data[u];
        }
        // partially intersect
        let m = (l + r) / 2;
        self.push(u, l, r);
        op(
            self.prod(a, b, 2 * u + 1, l, m),
            self.prod(a, b, 2 * u + 2, m, r),
        )
    }

    fn apply(&mut self, a: usize, b: usize, x: F, u: usize, l: usize, r: usize) {
        // l..r has no intersection with a..b
        if l >= b || r <= a {
            return;
        }
        // l..r is inside a..b
        if l >= a && r <= b {
            self.data[u] = apply_lazy(x, self.data[u], u, l, r);
            self.lazy[u] = merge_lazy(x, self.lazy[u]);
            return;
        }
        // partially intersect
        let (m, lch, rch) = ((l + r) / 2, 2 * u + 1, 2 * u + 2);
        self.push(u, l, r);
        self.apply(a, b, x, lch, l, m);
        self.apply(a, b, x, rch, m, r);
        self.data[u] = op(self.data[lch], self.data[rch]);
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

// fn reads() -> Vec<char> {
//     read::<String>().chars().collect::<Vec<char>>()
// }

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
