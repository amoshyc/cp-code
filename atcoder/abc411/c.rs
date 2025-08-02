#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let ask = readv::<usize>();
    let ask = mapv(&ask, |&x| x - 1);

    let mut is_white = vec![true; n];
    let mut cnt_segs = 0;
    let mut ans = vec![];
    for x in ask {
        if is_white[x] {
            if x >= 1 && x + 1 < n && !is_white[x - 1] && !is_white[x + 1] {
                // BWB -> BBB
                cnt_segs -= 1;
            } else if (x >= 1 && !is_white[x - 1]) || (x + 1 < n && !is_white[x + 1]) {
                // BWW or WWB -> BBW or WBB
                cnt_segs += 0;
            } else {
                // WWW -> WBW
                cnt_segs += 1;
            }
            is_white[x] = false;
        } else {
            if x >= 1 && x + 1 < n && !is_white[x - 1] && !is_white[x + 1] {
                // BBB -> BWB
                cnt_segs += 1;
            } else if (x >= 1 && !is_white[x - 1]) || (x + 1 < n && !is_white[x + 1]) {
                // BBW or WBB ->  BWW or WWB
                cnt_segs += 0;
            } else {
                // WBW -> WWW
                cnt_segs -= 1;
            }
            is_white[x] = true;
        }
        ans.push(cnt_segs);
    }

    println!("{}", join(&ans, "\n"));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect()
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
