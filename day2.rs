use std::io;

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    while (stdin.read_line(&mut line).is_ok()) {
        let round = line.split(' ').collect::<Vec<_>>();
        let (mut play1, play2) = (round[0], round[1]);

        play1 = if play1 == "A" { "X" } else { play1 };
        play1 = if play1 == "B" { "Y" } else { play1 };
        play1 = if play1 == "C" { "Z" } else { play1 };
    }
}
