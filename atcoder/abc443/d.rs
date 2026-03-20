#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        tc: usize,
    }

    let mut res = vec![];
    for _ in 0..tc {
        input! {
            n: usize,
            arr: [usize; n],
        }

        let mut events = vec![vec![]; n + 2];
        for i in 0..n {
            events[arr[i]].push(i);
        }

        let mut pos = vec![!0; n];
        for x in 1..=n {
            for i in events[x].clone() {
                if pos[i] == !0 {
                    pos[i] = x;

                    if i >= 1 && arr[i - 1] > x {
                        events[x + 1].push(i - 1);
                    }
                    if i + 1 < n && arr[i + 1] > x {
                        events[x + 1].push(i + 1);
                    }
                }
            }
        }

        let cnt = (0..n).map(|i| (arr[i] - pos[i]) as i64).sum::<i64>();
        res.push(cnt);
    }

    println!("{}", join(&res, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
