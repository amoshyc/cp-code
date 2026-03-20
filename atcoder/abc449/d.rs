#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        l: i64,
        r: i64,
        d: i64,
        u: i64,
    }

    let mut ans = 0;

    // (0, 0)
    if l <= 0 && 0 <= r && d <= 0 && 0 <= u {
        ans += 1;
    }

    for p in (2..=1_000_000).step_by(2) {
        // top: (-p, p)..=(p, p)
        if d <= p && p <= u {
            let a = (-p).max(l);
            let b = p.min(r);
            if a <= b {
                ans += b - a + 1;
            }
        }

        // bottom: (-p, -p)..=(p, -p)
        if d <= -p && -p <= u {
            let a = (-p).max(l);
            let b = p.min(r);
            if a <= b {
                ans += b - a + 1;
            }
        }

        // left: (-p, -p + 1)..=(-p, p - 1)
        if l <= -p && -p <= r {
            let a = (-p + 1).max(d);
            let b = (p - 1).min(u);
            if a <= b {
                ans += b - a + 1;
            }
        }

        // right: (p, -p + 1)..=(p, p - 1)
        if l <= p && p <= r {
            let a = (-p + 1).max(d);
            let b = (p - 1).min(u);
            if a <= b {
                ans += b - a + 1;
            }
        }
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
