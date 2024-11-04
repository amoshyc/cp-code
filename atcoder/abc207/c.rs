#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut segs = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        let (t, l, r) = (inp[0], inp[1], inp[2]);
        if t == 1 {
            segs.push((2 * l, 2 * r));
        } else if t == 2 {
            segs.push((2 * l, 2 * r - 1));
        } else if t == 3 {
            segs.push((2 * l + 1, 2 * r));
        } else {
            segs.push((2 * l + 1, 2 * r - 1));
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let (l1, r1) = segs[i];
            let (l2, r2) = segs[j];
            let l = l1.max(l2);
            let r = r1.min(r2);
            if r >= l {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
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
