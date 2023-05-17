// use itertools::*;
use proconio::input;
use std::collections::*;

type S = HashSet<i32>;
type F = HashMap<i32, S>;

fn main() {
    input! {
        _: i32,
        q: i32,
        tab: [(u8, i32, i32); q]
    }

    let mut follows = F::new();
    for (cmd, a, b) in tab {
        if cmd == 1 {
            follows.entry(a).or_insert(S::new()).insert(b);
        } else if cmd == 2 {
            follows.entry(a).or_insert(S::new()).remove(&b);
        } else {
            let ok1 = follows.get(&a).unwrap_or(&S::new()).contains(&b);
            let ok2 = follows.get(&b).unwrap_or(&S::new()).contains(&a);
            let ans = if ok1 && ok2 { "Yes" } else { "No" };
            println!("{}", ans);
        }
    }
}
