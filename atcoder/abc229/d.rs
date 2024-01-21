#![allow(unused)]


fn main() {
    let s = reads();
    let n = s.len();
    let k = read::<usize>();

    let ok = |m: usize| -> bool {
        let mut cnt = 0;
        for i in 0..m {
            if s[i] == '.' {
                cnt += 1;
            }
        }
        if cnt <= k {
            return true;
        }

        for i in m..n {
            if s[i] == '.' {
                cnt += 1;
            }
            if s[i - m] == '.' {
                cnt -= 1;
            }
            if cnt <= k {
                return true;
            }
        }

        false
    };

    // 1 1 1 0 0 0
    let mut lb = 0;
    let mut ub = n;
    if ok(ub) {
        println!("{}", ub);
        return;
    }
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            lb = m;
        } else {
            ub = m;
        }
    }

    println!("{}", lb);
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
