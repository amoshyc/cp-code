#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let mut arr = readv::<i64>();

    if k == 1 {
        println!("0");
        return;
    }

    if (2 * n - k) % 2 == 0 {
        let mut ans = 0;
        for i in (1..k).step_by(2) {
            ans += arr[i] - arr[i - 1];
        }
        println!("{}", ans);
    } else {
        let mut pref = vec![0; k];
        pref[1] = arr[1] - arr[0];
        for i in (2..k) {
            pref[i] = pref[i - 1];
            if i % 2 == 1 {
                pref[i] += arr[i] - arr[i - 1];
            }
        }

        let mut suff = vec![0; k];
        for i in (1..=(k - 2)).rev() {
            suff[i] = suff[i + 1];
            if i % 2 == 1 {
                suff[i] += arr[i + 1] - arr[i];
            }
        }

        let mut ans = 10i64.pow(18);
        for i in 0..k {
            if i % 2 == 0 {
                let mut val = 0;
                if i > 0 {
                    val += pref[i - 1];
                }
                if i + 1 < k {
                    val += suff[i + 1];
                }
                ans = ans.min(val);
            } else {
                let mut val = arr[i + 1] - arr[i - 1];
                if i > 2 {
                    val += pref[i - 2];
                }
                if i + 2 < k {
                    val += suff[i + 2];
                }
                ans = ans.min(val);
            }
        }
        println!("{}", ans);
    }
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
