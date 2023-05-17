use std::vec;

fn input<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read");
    buf.split_ascii_whitespace()
        .map(|t| String::from(t).parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

type C = Vec<i32>;

fn add(a: &C, b: &C) -> C {
    a.iter().zip(b).map(|(x, y)| x + y).collect::<Vec<i32>>()
}

fn sub(a: &C, b: &C) -> C {
    a.iter().zip(b).map(|(x, y)| x - y).collect::<Vec<i32>>()
}

fn main() {
    let x = input::<usize>();
    let (h, w, _, hh, ww) = (x[0], x[1], x[2], x[3], x[4]);

    let mut a = vec![];
    for _ in 0..h {
        a.push(input::<i32>());
    }

    let id = vec![0 as i32; 301];

    let mut p = vec![vec![id.clone(); w]; h];
    for r in 0..h {
        for c in 0..w {
            p[r][c][a[r][c] as usize] += 1;
        }
    }

    for r in 0..h {
        for c in 0..w {
            let x = if r >= 1 && c >= 1 {
                &p[r - 1][c - 1]
            } else {
                &id
            };
            let y = if r >= 1 { &p[r - 1][c] } else { &id };
            let z = if c >= 1 { &p[r][c - 1] } else { &id };
            p[r][c] = add(&sub(&add(y, z), x), &p[r][c]);
        }
    }

    for r in 0..=h - hh {
        for c in 0..=w - ww {
            let (r1, r2) = (r, r + hh - 1);
            let (c1, c2) = (c, c + ww - 1);
            let x = if r1 >= 1 && c1 >= 1 {
                &p[r1 - 1][c1 - 1]
            } else {
                &id
            };
            let y = if r1 >= 1 { &p[r1 - 1][c2] } else { &id };
            let z = if c1 >= 1 { &p[r2][c1 - 1] } else { &id };
            let cnt = &sub(&add(&p[r2][c2], x), &add(y, z));
            let f = sub(&p[h - 1][w - 1], cnt);
            let ans = f.iter().filter(|v| v > &&0).count();
            print!("{} ", ans);
        }
        println!();
    }
}
