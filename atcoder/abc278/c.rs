fn readv<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read");
    buf.split_ascii_whitespace()
        .map(|t| String::from(t).parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

use std::collections::*;
use std::io::*;

type S = HashSet<i32>;
type F = HashMap<i32, S>;

fn main() {
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let v = readv::<i32>();
    let (_, q) = (v[0], v[1]);
    let mut cmds = vec![];
    for _ in 0..q {
        let v = readv::<i32>();
        cmds.push((v[0], v[1], v[2]));
    }

    let mut follows = F::new();
    for (cmd, a, b) in cmds {
        if cmd == 1 {
            follows.entry(a).or_insert(S::new()).insert(b);
        } else if cmd == 2 {
            follows.entry(a).or_insert(S::new()).remove(&b);
        } else {
            let ok1 = follows.get(&a).unwrap_or(&S::new()).contains(&b);
            let ok2 = follows.get(&b).unwrap_or(&S::new()).contains(&a);
            let ans = if ok1 && ok2 { "Yes" } else { "No" };
            writeln!(writer, "{}", ans).ok();
            // println!("{}", ans);
        }
    }
}
