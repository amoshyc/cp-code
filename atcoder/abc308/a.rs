#![allow(unused)]

fn main() {
    let arr = readv::<usize>();

    let mut ok = true;
    for i in 0..8 {
        if arr[i] < 100 || arr[i] > 675 {
            ok = false;
        }
        if arr[i] % 25 != 0 {
            ok = false;
        }
        if i >= 1 && arr[i] < arr[i - 1] {
            ok = false;
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
