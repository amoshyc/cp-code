// use itertools::*;
use proconio::input;

fn check(h: i32, m: i32) -> bool {
    let (a, b) = (h / 10, h % 10);
    let (c, d) = (m / 10, m % 10);
    let hour = a * 10 + c;
    let min = b * 10 + d;
    (0..24).contains(&hour) && (0..60).contains(&min)
}

fn main() {
    input! {
        mut h: i32,
        mut m: i32,
    }

    loop {
        if check(h, m) {
            break;
        }

        m += 1;
        if m == 60 {
            // (h, w) = (h + 1, 0);
            h += 1;
            m = 0;
        }
        if h == 24 {
            // (h, w) = (0, 0);
            h = 0;
            m = 0;
        }
    }

    println!("{} {}", h, m);
}
