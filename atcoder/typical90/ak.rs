#![allow(unused)]

// [Problem]
// There are N types of dishes that use spices, numbered from 1 to N.
// The value of dish i (1 ≤ i ≤ N) is Vi​, and it consumes spices when made. The amount of spices consumed can be adjusted within the range of Li​ [mg] to Ri​ [mg].
// Determine whether the following is achievable. If possible, output the maximum possible sum of the values of the dishes made.
// By selecting some dishes from the N types and making one of each, exactly consume W [mg] of spices.
// However, it is not possible to consume spices by any means other than those mentioned above.

// [Solution]
// knapsack + segtree.
// dp[i, j] = maximum total value if chosen from dishes[0..=i] while using exactly j [mg] spices.
// Inspect all possible usage of dish i:
//    dish i is not chosen: dp[i, j] = dp[i - 1, j]
//    dish i is chosen: dp[i, j] = max(dp[i - 1, j - k] + value[i] for k in l[i]..=r[i])
//                               = max(dp[i - 1, j - k] for k in l[i]..=r[i]) + value[i]
// By storing dp in a segtree, each transition can be done in O(lg(W)).

fn main() {
    let inp = readv::<usize>();
    let (w, n) = (inp[0], inp[1]);

    let mut l = vec![];
    let mut r = vec![];
    let mut v = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        l.push(inp[0]);
        r.push(inp[1]);
        v.push(inp[2] as i64);
    }

    let mut dp = SegTree::<Node>::new(w + 1);
    dp.set(0, 0, 0, 0, dp.nn);
    for i in 0..n {
        let mut new_dp = vec![i64::MIN; w + 1];
        for j in 0..=w {
            let mut rhs = dp.get(j, j + 1, 0, 0, dp.nn);
            if j >= l[i] {
                let lb = if j >= r[i] { j - r[i] } else { 0 };
                let ub = if j >= l[i] { j - l[i] } else { 0 };
                rhs = rhs.max(dp.get(lb, ub + 1, 0, 0, dp.nn) + v[i]);
            }
            new_dp[j] = rhs;
        }
        dp = SegTree::<Node>::from_vec(&new_dp);
    }

    let ans = dp.get(w, w + 1, 0, 0, dp.nn);
    if ans < 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

struct Node;
impl SegTrait for Node {
    type S = i64;
    fn default() -> Self::S {
        i64::MIN
    }
    fn op(a: Self::S, b: Self::S) -> Self::S {
        a.max(b)
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
