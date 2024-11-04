#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let mut arr = readv::<i64>();

    let mut ans = vec![];
    for _ in 0..q {
        let k = read::<i64>();

        // find a first value m that m - sum(1 for x in A if x <= m) = k
        let ok = |m: i64| -> bool { m >= k + arr.partition_point(|x| *x <= m) as i64 };
        let mut lb = 0;
        let mut ub = 2 * 10i64.pow(18);
        while ub - lb > 1 {
            let m = (lb + ub) / 2;
            if ok(m) {
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
