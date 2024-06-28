#![allow(unused)]

fn solve() -> String {
    let n = read::<usize>();
    let mut s = reads();

    let mut ans = vec![];

    while s.len() > 0 {
        let l = s.len();
        
        // cvc
        if l >= 3 {
            let mut ok = true;
            ok &= s[l - 3] == 'b' || s[l - 3] == 'c' || s[l - 3] == 'd';
            ok &= s[l - 2] == 'a' || s[l - 2] == 'e';
            ok &= s[l - 1] == 'b' || s[l - 1] == 'c' || s[l - 1] == 'd';
            if ok {
                ans.push(join(&s[l - 3..], ""));
                s.pop();
                s.pop();
                s.pop();
                continue;
            }
        }

        // cv
        if l >= 2 {
            let mut ok = true;
            ok &= s[l - 2] == 'b' || s[l - 2] == 'c' || s[l - 2] == 'd';
            ok &= s[l - 1] == 'a' || s[l - 1] == 'e';
            if ok {
                ans.push(join(&s[l - 2..], ""));
                s.pop();
                s.pop();
            }
        }
    }

    ans.reverse();
    join(&ans, ".")
}

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![];
    for _ in 0..tc {
        ans.push(solve());
    }
    println!("{}", ans.join("\n"));
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
