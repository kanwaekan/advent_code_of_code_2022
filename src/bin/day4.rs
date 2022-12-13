use regex::Regex;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut cnt = 0;

    while stdin.read_line(&mut buf).is_ok() && !buf.is_empty() {
        let re = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
        let input = re.captures(&buf).expect("Failed to Parse");

        let x1: i32 = input[1].parse().unwrap();
        let y1: i32 = input[2].parse().unwrap();
        let x2: i32 = input[3].parse().unwrap();
        let y2: i32 = input[4].parse().unwrap();

        if (x1 < x2 && y1 < x2) {
        } else if (x2 < x1 && y2 < x1) {
        } else {
            cnt += 1;
        }

        buf.clear();
    }
    println!("Count: {}", cnt);
}
