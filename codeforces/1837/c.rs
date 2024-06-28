#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        let mut s = reads();
        let n = s.len();
        
        for i in 0..n {
            if s[i] == '?' {
                s[i] = '0';
            } else {
                break;
            }
        }
        for i in (0..n).rev() {
            if s[i] == '?' {
                s[i] = '1';
            } else {
                break;
            }
        }

        let mut segs = vec![];
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && s[j] == s[i] {
                j += 1;
            }
            segs.push((s[i], j - i));
            i = j;
        }

        for i in 0..segs.len() {
            let (x, c) = segs[i];
            if x == '?' {
                if segs[i - 1].0 == '1' && segs[i + 1].0 == '1' {
                    segs[i] = ('1', c);
                }
                else if segs[i - 1].0 == '0' && segs[i + 1].0 == '0' {
                    segs[i] = ('0', c);
                }
                else {
                    segs[i] = ('0', c);
                }
            }
        }

        let mut ans = vec![];
        for &(x, c) in segs.iter() {
            ans.extend(vec![x; c]);
        }
        println!("{}", join(&ans, ""));
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
