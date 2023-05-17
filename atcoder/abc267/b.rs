#![allow(unused)]

fn main() {
    let mut s = reads();
    s.insert(0, ' ');

    let b = [
        s[7] == '0',
        s[4] == '0',
        s[8] == '0' && s[2] == '0',
        s[5] == '0' && s[1] == '0',
        s[3] == '0' && s[9] == '0',
        s[6] == '0',
        s[10] == '0',
    ];

    for i in 0..7 {
        for j in i + 1..7 {
            let ok1 = b[i] == false;
            let ok2 = b[j] == false;
            let ok3 = b[i + 1..j].iter().any(|&x| x == true);
            let ok4 = s[1] == '0';
            if ok1 && ok2 && ok3 && ok4 {
                println!("Yes");
                return;
            }
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