use std::collections::HashMap;
use std::io;

fn solve1() {
    let stdin = io::stdin();
    let mut line = String::new();

    let (mut score1, mut score2) = (0u32, 0u32);
    let map = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    while stdin.read_line(&mut line).is_ok() {
        let round = line.trim().split(' ').collect::<Vec<_>>();
        println!("{:?}", round);
        if round.len() < 2 {
            break;
        }
        let (mut play1, play2) = (round[0], round[1]);

        play1 = if play1 == "A" { "X" } else { play1 };
        play1 = if play1 == "B" { "Y" } else { play1 };
        play1 = if play1 == "C" { "Z" } else { play1 };

        score1 += map[play2];
        score2 += map[play1];

        match (play1, play2) {
            ("X", "Y") | ("Y", "Z") | ("Z", "X") => {
                score1 += 6;
            }
            ("Y", "X") | ("Z", "Y") | ("X", "Z") => {
                score2 += 6;
            }
            _ => {
                score1 += 3;
                score2 += 3;
            }
        }
        line.clear();
    }

    println!("Score1: {}, Score2: {}", score1, score2);
}

fn solve2() {
    let stdin = io::stdin();
    let mut line = String::new();

    let (mut score1) = (0u32);
    let map = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let fail = HashMap::from([("X", 3), ("Y", 1), ("Z", 2)]);
    let succ = HashMap::from([("X", 2), ("Y", 3), ("Z", 1)]);

    while stdin.read_line(&mut line).is_ok() {
        let round = line.trim().split(' ').collect::<Vec<_>>();
        println!("{:?}", round);
        if round.len() < 2 {
            break;
        }
        let (mut play1, play2) = (round[0], round[1]);

        play1 = if play1 == "A" { "X" } else { play1 };
        play1 = if play1 == "B" { "Y" } else { play1 };
        play1 = if play1 == "C" { "Z" } else { play1 };

        match (play1, play2) {
            (play, "X") => score1 += fail[play],
            (play, "Y") => {
                score1 += map[play] + 3;
            }
            (play, "Z") => {
                score1 += succ[play] + 6;
            }
            _ => {}
        }
        line.clear();
    }

    println!("Score: {}", score1);
}
fn main() {
    solve2();
}
