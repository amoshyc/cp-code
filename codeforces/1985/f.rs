#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        let inp = readv::<usize>();
        let (h, n) = (inp[0] as i64, inp[1]);
        let arr_a = readv::<i64>();
        let arr_c = readv::<i64>();

        // 0 0 0 1 1 1
        let ok = |m: i64| -> bool {
            let mut sum: i64 = 0;
            for i in 0..n {
                // add = a[i] + (m - 1) // c[i] * a[i];
                let q = (m - 1) / arr_c[i];
                let add = arr_a[i].saturating_add(q.saturating_mul(arr_a[i]));
                sum = sum.saturating_add(add);
            }
            sum >= h
        };

        let mut lb = 0;
        let mut ub = 10i64.pow(16);
        while ub - lb > 1 {
            let m = (lb + ub) / 2;
            if ok(m) {
                ub = m;
            } else {
                lb = m;
            }
        }

        println!("{}", ub);
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
