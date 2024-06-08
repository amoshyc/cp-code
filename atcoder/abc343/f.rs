#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let arr = readv::<i32>();

    let arr = mapv(&arr, |&x| ((x, 1), (0, 0)));
    let mut seg = SegTree::<Node>::from_vec(&arr);

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        if inp[0] == 1 {
            let (p, x) = (inp[1] - 1, inp[2] as i32);
            seg.set(p, ((x, 1), (0, 0)), 0, 0, seg.nn);
        } else {
            let (l, r) = (inp[1] - 1, inp[2]);
            let ((max1, cnt1), (max2, cnt2)) = seg.get(l, r, 0, 0, seg.nn);
            ans.push(cnt2);
        }
    }
    println!("{}", join(&ans, "\n"));
}

struct Node;
impl SegTrait for Node {
    type S = ((i32, i32), (i32, i32)); // (max1, cnt1), (max2, cnt2)
    fn default() -> Self::S {
        ((0, 0), (0, 0))
    }
    fn op(a: Self::S, b: Self::S) -> Self::S {
        let arr1 = vec![a.0, a.1, (0, 0)];
        let arr2 = vec![b.0, b.1, (0, 0)];
        let mut i = 0;
        let mut j = 0;
        let mut res = vec![];
        while i < 3 && j < 3 && res.len() < 2 {
            if arr1[i].0 == arr2[j].0 {
                res.push((arr1[i].0, arr1[i].1 + arr2[j].1));
                i += 1;
                j += 1;
            } else if arr1[i].0 > arr2[j].0 {
                res.push(arr1[i]);
                i += 1;
            } else {
                res.push(arr2[j]);
                j += 1;
            }
        }
        if res.len() == 1 {
            (res[0], (0, 0))
        } else {
            (res[0], res[1])
        }
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
