#![allow(unused)]

fn main() {
    let mut cands = vec![vec![]; 3];

    for i in 0..3 {
        let mut generate = |dat: &Vec<Vec<char>>| {
            for dr in [-3, -2, -1, 0, 1, 2, 3] {
                for dc in [-3, -2, -1, 0, 1, 2, 3] {
                    let mut mask = 0u32;
                    let mut ok = true;
                    for r in 0..4 {
                        for c in 0..4 {
                            if dat[r][c] == '#' {
                                let nr = r.checked_add_signed(dr).unwrap_or(!0);
                                let nc = c.checked_add_signed(dc).unwrap_or(!0);
                                if nr >= 4 || nc >= 4 {
                                    ok = false;
                                } else {
                                    mask |= 1 << (nr * 4 + nc);
                                }
                            }
                        }
                    }
                    if ok {
                        cands[i].push(mask);
                    }
                }
            }
        };

        let mut arr = vec![];
        for r in 0..4 {
            arr.push(reads());
        }
        generate(&arr);
        arr = r90cw(&arr);
        generate(&arr);
        arr = r90cw(&arr);
        generate(&arr);
        arr = r90cw(&arr);
        generate(&arr);
    }

    // println!("{:?}", cands[0]);
    // println!("{:?}", cands[1]);
    // println!("{:?}", cands[2]);

    // let mut vis = |mask| {
    //     let mut vis = vec![vec![0; 4]; 4];
    //     for r in 0..4 {
    //         for c in 0..4 {
    //             if mask & (1 << (r * 4 + c)) > 0 {
    //                 vis[r][c] = 1;
    //             }
    //         }
    //     }
    //     for r in 0..4 {
    //         println!("{}", join(&vis[r], " "));
    //     }
    //     println!("------");
    // };

    for &mask_a in cands[0].iter() {
        for &mask_b in cands[1].iter() {
            for &mask_c in cands[2].iter() {
                let mut mask = mask_a | mask_b | mask_c;
                let mut ok = true;
                ok &= mask.count_ones() == 16;
                ok &= (mask_a & mask_b) == 0;
                ok &= (mask_a & mask_c) == 0;
                ok &= (mask_b & mask_c) == 0;
                if ok {
                    // vis(mask_a);
                    // vis(mask_b);
                    // vis(mask_c);
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}

// rotate clockwise
fn r90cw<T: Default + Clone>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let (n, m) = (arr.len(), arr[0].len());
    let mut res = vec![vec![T::default(); n]; m];
    for r in 0..n {
        for c in 0..m {
            res[r][c] = arr[c][n - 1 - r].clone();
        }
    }
    res
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
