#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<usize>();
    let arr = mapv(&arr, |&x| x - 1);
    // println!("{:?}", arr);

    let mut pos = vec![(0, 0); n];
    for i in 0..n {
        pos[arr[i]] = (i, i);
    }
    let mut seg = SegTree::<Node>::from_vec(&pos);

    let mut ans = usize::MAX;
    for i in 0..n {
        let a = arr[i];
        let b = a + k - 1;
        if b >= n {
            continue;
        }
        let (l, r) = seg.get(a, b + 1, 0, 0, seg.nn);
        // println!("a={}, b={}: l={}, r={}", a, b, l, r);
        ans = ans.min(r - l);
    }

    println!("{}", ans);
}

struct Node;
impl SegTrait for Node {
    type S = (usize, usize);
    fn default() -> Self::S {
        (usize::MAX, 0)
    }
    fn op(a: Self::S, b: Self::S) -> Self::S {
        (a.0.min(b.0), a.1.max(b.1))
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
    read::<String>().chars().collect::<_>()
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
