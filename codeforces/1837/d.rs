#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        let n = read::<usize>();
        let s = reads();

        let cnt1 = s.iter().filter(|&&c| c == '(').count();
        let cnt2 = s.iter().filter(|&&c| c == ')').count();
        if cnt1 != cnt2 {
            println!("-1");
            continue;
        }

        let mut color = vec![0; n];

        let mut i = 0;
        while i < n {
            let mut f = if s[i] == '(' { 1 } else { -1 };
            let mut j = i + 1;
            while j < n && f != 0 {
                f += if s[j] == '(' { 1 } else { -1 };
                j += 1;
            }

            if s[i] == '(' {
                for k in i..j {
                    color[k] = 1;
                }
            } else {
                for k in i..j {
                    color[k] = 2;
                }
            }

            i = j;
        }

        if color.iter().all(|&c| c == 2) {
            for i in 0..n {
                color[i] = 1;
            }
        }

        println!("{}", color.iter().max().unwrap());
        println!("{}", join(&color, " "));
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
