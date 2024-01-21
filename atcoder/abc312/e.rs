#![allow(unused)]

use std::collections::HashSet;


fn main() {
    let mut voxel = vec![vec![vec![!0; 100 + 1]; 100 + 1]; 100 + 1];

    let n = read::<usize>();
    let mut arr = vec![];
    for i in 0..n {
        let inp = readv::<usize>();
        arr.push(inp.clone());
        let (x1, y1, z1) = (inp[0], inp[1], inp[2]);
        let (x2, y2, z2) = (inp[3], inp[4], inp[5]);
        // voxel [x1..x2, y1..y2, z1..z2]
        for x in x1..x2 {
            for y in y1..y2 {
                for z in z1..z2 {
                    voxel[x][y][z] = i;
                }
            }
        }
    }

    let mut ans = vec![];
    for i in 0..n {
        let (x1, y1, z1) = (arr[i][0], arr[i][1], arr[i][2]);
        let (x2, y2, z2) = (arr[i][3], arr[i][4], arr[i][5]);

        let mut set = HashSet::new();

        for y in y1..y2 {
            for z in z1..z2 {
                if x1 >= 1 {
                    set.insert(voxel[x1 - 1][y][z]);
                }
                if x2 <= 100 {
                    set.insert(voxel[x2][y][z]);
                }
            }
        }
        for x in x1..x2 {
            for z in z1..z2 {
                if y1 >= 1 {
                    set.insert(voxel[x][y1 - 1][z]);
                }
                if y2 <= 100 {
                    set.insert(voxel[x][y2][z]);
                }
            }
        }
        for &z in [z1, z2].iter() {
            for x in x1..x2 {
                for y in y1..y2 {
                    if z1 >= 1 {
                        set.insert(voxel[x][y][z1 - 1]);
                    }
                    if z2 <= 100 {
                        set.insert(voxel[x][y][z2]);
                    }
                }
            }
        }

        set.remove(&i);
        set.remove(&!0);
        ans.push(set.len());
    }

    println!("{}", join(&ans, "\n"));
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
