#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);

    let mut arr1 = vec![];
    for _ in 0..h {
        arr1.push(reads());
    }

    let mut arr2 = vec![];
    for _ in 0..h {
        arr2.push(reads());
    }

    for s in 0..h {
        for t in 0..w {
            let mut arr = arr1.clone();
            arr.rotate_right(s);
            for i in 0..h {
                arr[i].rotate_right(t);
            }

            let mut same = true;
            for r in 0..h {
                for c in 0..w {
                    if arr[r][c] != arr2[r][c] {
                        same = false;
                    }
                }
            }

            if same {
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}