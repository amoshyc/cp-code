#![allow(unused)]


fn main() {
    let n = read::<usize>();
    let mut pos = vec![];
    for _ in 0..n {
        let inp = readv::<i32>();
        pos.push((inp[0], inp[1]));
    }
    let dir = reads();

    let mut groups = std::collections::HashMap::new();
    for i in 0..n {
        let (x, y) = pos[i];
        let d = dir[i];
        groups.entry(y).or_insert(vec![]).push((x, d));
    }

    let mut any = false;
    for (k, v) in groups.iter_mut() {
        v.sort();
        let i = v.iter().position(|&(x, d)| d == 'R');
        let j = v.iter().rposition(|&(x, d)| d == 'L');
        if let (Some(i), Some(j)) = (i, j) {
            if i < j {
                any = true;
                break;
            }
        }
    }

    if any {
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
