#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let n = inp[0] as usize;
    let x = inp[1];
    let mut arr = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        arr.push((inp[0], inp[1], inp[2], inp[3]));
    }

    // Is it possible to make all process having >= m capacity
    let ok = |m: i64| -> bool {
        let mut cost = 0;
        for i in 0..n {
            // Assume that the smaller number <= 100
            let (a, p, b, q) = arr[i];
            let mut min = 10i64.pow(15);
            for cnt_s in 0..=100 {
                let cnt_t = ceil_div(m - cnt_s * a, b);
                if cnt_s * a <= m && cnt_s * a + cnt_t * b >= m {
                    min = min.min(cnt_s * p + cnt_t * q);
                }
            }
            for cnt_t in 0..=100 {
                let cnt_s = ceil_div(m - cnt_t * b, a);
                if cnt_t * b <= m && cnt_s * a + cnt_t * b >= m {
                    min = min.min(cnt_s * p + cnt_t * q);
                }
            }
            cost += min;
        }
        cost <= x
    };

    // 1 1 1 0 0 0
    let mut lb = 0;
    let mut ub = 10i64.pow(12);
    if !ok(lb) {
        println!("0");
        return;
    }
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            lb = m;
        } else {
            ub = m;
        }
    }
    println!("{}", lb);
}

// Add 1 to floor(a/b) if needed
fn ceil_div(a: i64, b: i64) -> i64 {
    a.div_euclid(b) + if a.rem_euclid(b) != 0 { 1 } else { 0 }
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
