#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64,
        arr: [i64; n],
    }

    // a[i] + b[i] = arr[i]
    // a[i] * x + b[i] * y = const

    // const = (arr[i] - b[i]) * x + b[i] * y
    // const = arr[i] * x + b[i] * (y - x)
    // const = arr[i] * x (mod y - x)

    // (m, r) = crt(...)
    // const = k * m + r
    // maximize const
    let mut coef = vec![];
    for i in 0..n {
        let r = arr[i] * x % (y - x);
        let m = y - x;
        coef.push((m, r));
    }
    let res = crt(&coef);
    if res.is_none() {
        println!("-1");
        return;
    }
    let (m, r) = crt(&coef).unwrap();
    // println!("{:?}", (m, r));

    // b[i] <= arr[i]
    // b[i] * (y - x) = const - arr[i] * x
    // const - arr[i] * x <= arr[i] * (y - x)
    // const <= arr[i] * y
    // k * m + r <= arr[i] * y
    // k <= floor_div(arr[i] * y - r, m)
    let ks = arr.iter().map(|&a| (a * y - r) / m).collect::<Vec<_>>();
    let k = ks.iter().min().unwrap();

    // const = arr[i] * x + b[i] * (y - x)
    // b[i] = (const - arr[i] * x) / (y - x)
    let c = k * m + r;
    let mut ans = 0;
    for i in 0..n {
        if c < arr[i] * x {
            println!("-1");
            return;
        }
        ans += (c - arr[i] * x) / (y - x);
    }

    println!("{}", ans);
}

fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (1, 0, a) // (x0, y0, g)
    } else {
        let (x1, y1, g) = extgcd(b, a.rem_euclid(b));
        (y1, x1 - y1 * (a / b), g) // (x0, y0, g)
    }
}

fn minv(a: i64, m: i64) -> i64 {
    let (x0, _, _) = extgcd(a, m);
    x0.rem_euclid(m)
}

// ax = b (mod m) has solution
//  x = (a/g)^(-1) * (b/g) (mod m/g)
fn linear_congruence(a: i64, b: i64, m: i64) -> Option<(i64, i64)> {
    let (inv, _, g) = extgcd(a, m);
    if b % g != 0 {
        None
    } else {
        Some(((inv * (b / g)).rem_euclid(m / g), g))
    }
}

// x = r1 (mod m1)
// x = r2 (mod m2) has solution
// x = m1 q1 + r1 (mod lcm(m1, m2)) where
// q1 = linear_congruence(m1, r2 - r1, m2)
fn crt(coef: &Vec<(i64, i64)>) -> Option<(i64, i64)> {
    let mut m1 = coef[0].0;
    let mut r1 = coef[0].1;
    for i in 1..coef.len() {
        let (m2, r2) = coef[i];
        if let Some((q1, g)) = linear_congruence(m1, r2 - r1, m2) {
            let lcm = m1 / g * m2;
            r1 = (m1 * q1 + r1).rem_euclid(lcm);
            m1 = lcm;
        } else {
            return None;
        }
    }
    Some((m1, r1))
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
