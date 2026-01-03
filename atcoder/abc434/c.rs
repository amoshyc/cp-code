#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        tc: usize,
    }

    for _ in 0..tc {
        input! {
            n: usize,
            h: i64,
            arr: [(i64, i64, i64); n],
        }

        let mut t = arr[n - 1].0;
        let mut l = arr[n - 1].1;
        let mut r = arr[n - 1].2;
        let mut ok = true;

        for i in (0..(n - 1)).rev() {
            let (pt, pl, pr) = arr[i];

            // [pt, pl, pr] <- [t, l, r]
            // nl = l - (t - pt)
            // nr = r + (t - pt)
            // (l', r') = intersection of [nl, nr] and [pl, pr]
            let dt = t - pt;
            l = pl.max(l - dt).max(1);
            r = pr.min(r + dt);
            t = pt;
            if r < l {
                ok = false;
                break;
            }
        }

        l = (l - arr[0].0).max(1);
        r = r + arr[0].0;

        ok &= (l <= h) && (h <= r);
        if ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
