#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, h) = (inp[0], inp[1], inp[2] as i64);
    let mut kind = vec![0; n];
    let mut attk = vec![0; n];
    for i in 0..n {
        let inp = readv::<usize>();
        kind[i] = inp[1] - 1;
        attk[i] = inp[0] as i64;
    }

    let mut cnt = vec![0; n];
    let mut sum = 0;
    let mut larger = std::collections::BTreeSet::new();
    let mut smaller = std::collections::BTreeSet::new();
    let mut need = vec![!0; n];

    for i in 0..n {
        let old_key = (cnt[kind[i]], kind[i]);
        if smaller.remove(&old_key) {
            sum -= cnt[kind[i]];
        } else {
            larger.remove(&old_key);
        }

        cnt[kind[i]] += attk[i];
        let new_key = (cnt[kind[i]], kind[i]);
        smaller.insert(new_key);
        sum += cnt[kind[i]];

        // min(larger) should >= max(smaller)
        while larger.len() > 0 && smaller.len() > 0 {
            let (c1, k1) = *larger.iter().next().unwrap();
            let (c2, k2) = *smaller.iter().last().unwrap();
            if c1 < c2 {
                larger.remove(&(c1, k1));
                larger.insert((c2, k2));
                smaller.remove(&(c2, k2));
                smaller.insert((c1, k1));
                sum += c1;
                sum -= c2;
            } else {
                break;
            }
        }

        // make sum >= h
        while larger.len() > 0 && sum < h {
            let (c, k) = larger.pop_first().unwrap();
            smaller.insert((c, k));
            sum += c;
        }

        // make sum <= h
        while smaller.len() > 0 && sum >= h {
            let (c, k) = smaller.pop_last().unwrap();
            larger.insert((c, k));
            sum -= c;
        }

        need[i] = larger.len();
    }

    let mut indices = (0..n).collect::<Vec<_>>();
    let mut ans = vec![];
    for k in 0..=m {
        let p = indices.partition_point(|&i| need[i] <= k);
        ans.push(p);
    }

    println!("{}", join(&ans, " "));
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
