#![allow(unused)]

fn main() {
    let mut arr = vec![];
    for r in 0..9 {
        arr.push(readv::<usize>());
    }

    let mut ok = true;

    for r in 0..9 {
        let mut cnt = vec![0; 10];
        for c in 0..9 {
            cnt[arr[r][c]] += 1;
        }
        ok &= (1..=9).all(|x| cnt[x] == 1);
    }

    for c in 0..9 {
        let mut cnt = vec![0; 10];
        for r in 0..9 {
            cnt[arr[r][c]] += 1;
        }
        ok &= (1..=9).all(|x| cnt[x] == 1);
    }

    let mut cnt = vec![vec![vec![0; 10]; 3]; 3];
    for r in 0..9 {
        for c in 0..9 {
            cnt[r / 3][c / 3][arr[r][c]] += 1;
        }
    }

    for r in 0..3 {
        for c in 0..3 {
            ok &= (1..=9).all(|x| cnt[r][c][x] == 1);
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
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
