#![allow(unused)]

const MAX: i64 = 2 * 10i64.pow(6) + 1;

fn main() {
    let inp = readv::<i64>();
    let (n, d) = (inp[0] as usize, inp[1]);
    let mut xs = vec![];
    let mut ys = vec![];
    for i in 0..n {
        let xy = readv::<i64>();
        xs.push(xy[0]);
        ys.push(xy[1]);
    }
    xs.sort();
    ys.sort();
    let pref_x = build(&xs);
    let pref_y = build(&ys);

    // Inspect each x and find the number of valid y
    let mut ans = 0;
    for x in (-MAX)..=MAX {
        let bound = d - total_l1_dist_to_sorted(x, &xs, &pref_x);
        let value = solve_1d(&ys, &pref_y, bound);
        ans += value;
    }

    println!("{}", ans);
}

// Finding the total l1 distance to points in O(lg(n)) by  binary search and prefix sum
fn total_l1_dist_to_sorted(x: i64, sorted: &Vec<i64>, pref: &Vec<i64>) -> i64 {
    let n = sorted.len();
    let p = sorted.partition_point(|xi| *xi <= x);
    let d1 = (p as i64) * x - query(&pref, 0, p);
    let d2 = query(&pref, p, n) - ((n - p) as i64) * x;
    d1 + d2
}

// Given the points pts in number line, find the number of integer x such that
//   sum([abs(x - pts[i]) for i in 0..n]) <= bound
// The total distance function is a U-shaped (concave) function with minimum at median(pts)
// We want to find range the function has value <= bound.
// It can be solved by binary searching the left border and the right border.
fn solve_1d(sorted: &Vec<i64>, pref: &Vec<i64>, bound: i64) -> i64 {
    let n = sorted.len();
    let median = sorted[n / 2];

    let ok = |x: i64| total_l1_dist_to_sorted(x, sorted, pref) <= bound;
    if !ok(median) {
        return 0;
    }

    // left border: 0 0 0 1 1 1
    let mut lb = -MAX;
    let mut ub = median;
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            ub = m;
        } else {
            lb = m;
        }
    }
    let border_l = ub;

    // right border: 1 1 1 0 0 0
    let mut lb = median;
    let mut ub = MAX;
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            lb = m;
        } else {
            ub = m;
        }
    }
    let border_r = lb;

    border_r - border_l + 1
}

fn build<T>(arr: &[T]) -> Vec<T>
where
    T: Copy + std::ops::Add<Output = T>,
{
    let mut pref = vec![];
    pref.push(arr[0]);
    for i in 1..arr.len() {
        pref.push(pref[i - 1] + arr[i]);
    }
    pref
}

// i..j
fn query<T>(pref: &[T], i: usize, j: usize) -> T
where
    T: Default + Copy + std::ops::Sub<Output = T>,
{
    if i == j {
        return T::default();
    }
    let mut res = pref[j - 1];
    if i > 0 {
        res = res - pref[i - 1];
    }
    res
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
