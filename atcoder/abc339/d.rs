#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut ban = std::collections::HashSet::new();
    let mut rs = vec![0; 2];
    let mut cs = vec![0; 2];
    let mut i = 0;
    for r in 0..n {
        let row = reads();
        for c in 0..n {
            if row[c] == '#' {
                ban.insert((r as i32, c as i32));
            } else if row[c] == 'P' {
                rs[i] = r as i32;
                cs[i] = c as i32;
                i += 1;
            }
        }
    }

    let mut dis = std::collections::HashMap::new();
    let mut que = std::collections::VecDeque::new();
    let root = ((rs[0], cs[0], rs[1], cs[1]));
    dis.insert(root, 0);
    que.push_back(root);

    while let Some((r0, c0, r1, c1)) = que.pop_front() {
        let d = dis[&(r0, c0, r1, c1)];

        if (r0, c0) == (r1, c1) {
            println!("{}", d);
            return;
        }

        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nr0, nc0) = (r0 + dr, c0 + dc);
            let (nr1, nc1) = (r1 + dr, c1 + dc);
            let ok1 = (nr0 >= 0 && nr0 < n as i32 && nc0 >= 0 && nc0 < n as i32);
            let ok2 = (nr1 >= 0 && nr1 < n as i32 && nc1 >= 0 && nc1 < n as i32);
            let ok3 = !ban.contains(&(nr0, nc0));
            let ok4 = !ban.contains(&(nr1, nc1));
            let mut rr0 = r0;
            let mut cc0 = c0;
            if ok1 && ok3 {
                (rr0, cc0) = (nr0, nc0);
            }
            let mut rr1 = r1;
            let mut cc1 = c1;
            if ok2 && ok4 {
                (rr1, cc1) = (nr1, nc1);
            }

            let key = (rr0, cc0, rr1, cc1);
            if !dis.contains_key(&key) {
                dis.insert(key, d + 1);
                que.push_back(key);
            }
        }
    }

    println!("-1");
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
