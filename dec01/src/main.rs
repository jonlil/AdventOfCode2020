use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut expenses: Vec<u32> = vec![];

    for line in stdin.lock().lines() {
        expenses.push(line.unwrap().parse::<u32>().unwrap());
    }

    for e1 in &expenses {
        for e2 in &expenses {
            for e3 in &expenses {
                if e1 + e2 + e3 == 2020 {
                    eprintln!("{} + {} + {} = {}", e1, e2, e3, e1 * e2 * e3);
                }
            }
        }
    }
}
