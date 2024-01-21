fn main() {
    let inp = readv::<f64>();
    let (a, b) = (inp[0], inp[1]);

    let f = |n: f64| -> f64 {
        b * n + a * ((1.0 + n).powf(-0.5))
    };

    let mut lb = 0.0 as f64;
    let mut ub = (10.0 as f64).powf(19.0);
    for _ in 0..200 {
        let l = (2.0 * lb + 1.0 * ub) / 3.0;
        let r = (1.0 * lb + 2.0 * ub) / 3.0;
        if f(l) <= f(r) {
            ub = r;
        } else {
            lb = l;
        }
    }

    let ans1 = f(lb.ceil());
    let ans2 = f(lb.floor());
    println!("{:.7}", ans1.min(ans2));
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
