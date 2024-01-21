use std::io;

fn input() -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();
    inp = inp.trim().to_string();
    inp
}

fn main() {
    let inp = input();
    if inp == "RRR" {
        println!("3");
    }
    else if inp.find("RR") != None {
        println!("2");
    }
    else if inp.find("R") != None {
        println!("1");
    }
    else {
        println!("0");
    }
}