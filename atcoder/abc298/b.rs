#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        a.push(readv::<usize>());
    }
    for _ in 0..n {
        b.push(readv::<usize>());
    }

    for _ in 0..4 {
        let mut ok = true;
        for i in 0..n {
            for j in 0..n {
                if a[i][j] == 1 {
                    if b[i][j] == 0 {
                        ok = false;
                    }
                }
            }
        }

        if ok {
            println!("Yes");
            return;
        } else {
            let mut c = a.clone();
            for i in 0..n {
                for j in 0..n {
                    c[i][j] = a[n - 1 - j][i];
                }
            }
            a = c;
        }
    }

    println!("No");
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
