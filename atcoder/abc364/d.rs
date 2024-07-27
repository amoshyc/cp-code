#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let mut arr_a = readv::<i64>();
    arr_a.sort();

    // number of points inside [x - d, x + d] >= k
    let ok = |x: i64, d: i64, k: usize| -> bool {
        let ub = arr_a.partition_point(|a| *a <= x + d);
        let lb = arr_a.partition_point(|a| *a < x - d);
        let cnt = ub - lb;
        cnt >= k
    };

    // for d in 0..10 {
    //     println!("{}: {}", d, ok(-2, d, 3));
    // }

    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<i64>();
        let (x, k) = (ask[0], ask[1] as usize);

        // 0 0 0 1 1 1
        let mut lb = 0;
        let mut ub = 10i64.pow(10);
        if ok(x, lb, k) {
            ans.push(lb);
            continue;
        }
        while ub - lb > 1 {
            let m = (lb + ub) / 2;
            if ok(x, m, k) {
                ub = m;
            } else {
                lb = m;
            }
        }

        ans.push(ub);
    }

    println!("{}", join(&ans, "\n"));
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
