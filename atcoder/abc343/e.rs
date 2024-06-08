#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (v1, v2, v3) = (inp[0], inp[1], inp[2]);

    let (x1, y1, z1) = (0, 0, 0);
    let c1 = Cube::new(x1, x1 + 7, y1, y1 + 7, z1, z1 + 7);
    for x2 in -7..=7 {
        for y2 in -7..=7 {
            for z2 in -7..=7 {
                let c2 = Cube::new(x2, x2 + 7, y2, y2 + 7, z2, z2 + 7);
                for x3 in -7..=7 {
                    for y3 in -7..=7 {
                        for z3 in -7..=7 {
                            let c3 = Cube::new(x3, x3 + 7, y3, y3 + 7, z3, z3 + 7);

                            let c12 = c1.intersect(&c2);
                            let c13 = c1.intersect(&c3);
                            let c23 = c2.intersect(&c3);
                            let c123 = c12.intersect(&c3);
                            let b3 = c123.vol();
                            let b2 = c12.vol() + c13.vol() + c23.vol() - 3 * b3;
                            let b1 = 3 * 7 * 7 * 7 - 2 * b2 - 3 * b3;

                            if (b1, b2, b3) == (v1, v2, v3) {
                                println!("Yes");
                                println!(
                                    "{} {} {} {} {} {} {} {} {}",
                                    x1, y1, z1, x2, y2, z2, x3, y3, z3
                                );
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("No");
}

#[derive(Clone, Copy, Debug)]
struct Cube {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
}

impl Cube {
    fn new(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> Self {
        Self {
            x1,
            x2,
            y1,
            y2,
            z1,
            z2,
        }
    }

    fn intersect(&self, other: &Cube) -> Cube {
        let x1 = self.x1.max(other.x1);
        let x2 = self.x2.min(other.x2);
        let y1 = self.y1.max(other.y1);
        let y2 = self.y2.min(other.y2);
        let z1 = self.z1.max(other.z1);
        let z2 = self.z2.min(other.z2);
        Cube {
            x1,
            x2,
            y1,
            y2,
            z1,
            z2,
        }
    }

    fn vol(&self) -> i64 {
        let dx = (self.x2 - self.x1).max(0);
        let dy = (self.y2 - self.y1).max(0);
        let dz = (self.z2 - self.z1).max(0);
        dx * dy * dz
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
