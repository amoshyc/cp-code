#![allow(unused)]

fn main() {
    let s = reads();

    let mut stack = vec![];
    let mut cnt = vec![0; 26];
    let mut ans = true;
    for c in s {
        match c {
            '(' => stack.push(c),
            ')' => {
                while let Some(p) = stack.pop() {
                    if p == '(' {
                        break;
                    }
                    let x = p as usize - 'a' as usize;
                    cnt[x] -= 1;
                }
            }
            'a'..='z' => {
                stack.push(c);
                let x = c as usize - 'a' as usize;
                cnt[x] += 1;
                if cnt[x] >= 2 {
                    ans = false;
                    break;
                }
            }
            _ => (),
        }
    }

    if ans {
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
