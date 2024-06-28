#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        let s = reads();
        let n = s.len();

        let mut segs = vec![];
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && s[j] == s[i] {
                j += 1;
            }
            segs.push(s[i]);
            i = j;
        }

        let m = segs.len();
        let cnt0 = segs.iter().filter(|x| **x == '0').count();
        let cnt1 = segs.iter().filter(|x| **x == '1').count();

        if segs.len() == 1 {
            println!("1");
        } else {
            if (segs[0], segs[m - 1]) == ('0', '0') {
                // 010 => 01, 0
                // 01010 => 01, 0, 1, 0
                // 0101010 => 01, 0, 1, 0, 1, 0
                println!("{}", m - 1);
            } else if (segs[0], segs[m - 1]) == ('0', '1') {
                // 01 => 01
                // 0101 => 01, 0, 1
                // 010101 => 01, 0, 1, 0, 1
                println!("{}", m - 1);
            } else if (segs[0], segs[m - 1]) == ('1', '0') {
                // 10 => 1, 0
                // 1010 => 1, 01, 0
                // 101010 => 1, 01, 0, 1, 0
                // 10101010 => 1, 01, 0, 1, 0, 1, 0
                if m == 2 {
                    println!("2");
                } else {
                    println!("{}", m - 1);
                }
            } else {
                // 010 => 01, 0
                // 01010 => 01, 0, 1, 0
                println!("{}", m - 1);
            }
        }
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
    read::<String>().chars().collect::<_>()
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
