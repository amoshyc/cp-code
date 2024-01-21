#![allow(unused)]

fn main() {
    let (n, m) = read2::<usize, usize>();
    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    for _ in 0..n {
        let (t, x) = read2::<usize, i64>();
        if t == 0 {
            a.push(x);
        } else if t == 1 {
            b.push(x);
        } else {
            c.push(x);
        }
    }

    a.sort();
    a.reverse();
    let mut ans = 0;
    let mut smaller = std::collections::BTreeSet::new();
    let mut larger = std::collections::BTreeSet::new();
    for i in 0..a.len() {
        if i < m {
            larger.insert((a[i], 'a', i));
            ans += a[i];
        } else {
            smaller.insert((a[i], 'a', i));
        }
    }
    
    b.sort();
    b.reverse();
    
    c.sort();
    c.reverse();
    
    let mut pref_c = 0;
    
    let mut cnt = ans;
    let mut index_b = 0;
    for index_c in 0..c.len() {
        pref_c += c[index_c];
        while index_b < b.len() && index_b as i64 + 1 <= pref_c {
            smaller.insert((b[index_b], 'b', index_b));
            index_b += 1;
        }
        
        while larger.len() + (index_c + 1) < m && smaller.len() > 0 {
            let (x, t, i) = *smaller.iter().last().unwrap();
            smaller.remove(&(x, t, i));
            larger.insert((x, t, i));
            cnt += x;
        }

        while larger.len() + (index_c + 1) > m && larger.len() > 0 {
            let (x, t, i) = *larger.iter().next().unwrap();
            larger.remove(&(x, t, i));
            smaller.insert((x, t, i));
            cnt -= x;
        }

        while larger.len() > 0 && smaller.len() > 0 {
            let (x1, t1, i1) = *larger.iter().next().unwrap();
            let (x2, t2, i2) = *smaller.iter().last().unwrap();
            if x1 < x2 {
                larger.remove(&(x1, t1, i1));
                larger.insert((x2, t2, i2));
                smaller.remove(&(x2, t2, i2));
                smaller.insert((x1, t1, i1));
                cnt += x2;
                cnt -= x1;
            } else {
                break;
            }
        }

        ans = ans.max(cnt);
    }

    println!("{}", ans);
}


fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read2<T: std::str::FromStr, F: std::str::FromStr>() -> (T, F) {
    let mut inp = read::<String>();
    let mut token = inp.split_ascii_whitespace();
    let a: T = token.next().unwrap().parse().ok().unwrap();
    let b: F = token.next().unwrap().parse().ok().unwrap();
    (a, b)
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
