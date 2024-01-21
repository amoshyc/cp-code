#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut pref = vec![0; n];
    let mut j = 0;
    for i in 0..n {
        while j < n && arr[j] >= 1 + (j - i) {
            pref[j] = pref[j].max(1 + j - i);
            j += 1;
        }
    }

    let mut suff = vec![0; n];
    let mut rev = arr.clone();
    rev.reverse();
    let mut j = 0;
    for i in 0..n {
        while j < n && rev[j] >= 1 + (j - i) {
            suff[j] = suff[j].max(1 + j - i);
            j += 1;
        }
    }
    suff.reverse();

    // println!("{:?}", pref);
    // println!("{:?}", suff);

    let check = |i: usize, k: usize| -> bool {
        if i + 2 * k > n + 1 {
            false
        } else {
            pref[i + k - 1] >= k && suff[i + k - 1] >= k
        }
    };

    let mut ans = 0;
    for i in 0..n {
        let mut lb = 1;
        let mut ub = n;
        while ub - lb > 1 {
            let m = (lb + ub) / 2;
            if check(i, m) {
                lb = m;
            } else {
                ub = m
            }
        }

        ans = ans.max(lb);
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
    read::<String>().chars().collect::<Vec<char>>()
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
