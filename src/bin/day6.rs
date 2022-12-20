use std::io;

fn solve1() {
    let mut stdin = io::stdin();
    let mut buf = String::new();

    while stdin.read_line(&mut buf).is_ok() && buf.len() > 0 {
        let arr = buf.chars().collect::<Vec<_>>();
        let mut map = [0u32; 30];
        let mut cnt = 0;

        for i in 0..4 {
            let val = arr[i] as usize - 97;
            map[val] += 1;
            if map[val] == 2 {
                cnt += 1;
            }
        }
        if cnt == 0 {
            println!("{}", 5);
            continue;
        }

        for i in 4..arr.len() {
            let rval = arr[i] as usize - 97;
            let lval = arr[i - 4] as usize - 97;
            map[rval] += 1;
            if map[rval] == 2 {
                cnt += 1;
            }
            map[lval] -= 1;
            if map[lval] == 1 {
                cnt -= 1;
            }
            if cnt == 0 {
                println!("{}", i + 1);
                break;
            }
        }
    }
}

fn solve2() {
    let mut stdin = io::stdin();
    let mut buf = String::new();

    while stdin.read_line(&mut buf).is_ok() && buf.len() > 0 {
        let arr = buf.chars().collect::<Vec<_>>();
        let mut map = [0u32; 30];
        let mut cnt = 0;

        for i in 0..14 {
            let val = arr[i] as usize - 97;
            map[val] += 1;
            if map[val] == 2 {
                cnt += 1;
            }
        }
        if cnt == 0 {
            println!("{}", 5);
            continue;
        }

        for i in 14..arr.len() {
            let rval = arr[i] as usize - 97;
            let lval = arr[i - 14] as usize - 97;
            map[rval] += 1;
            if map[rval] == 2 {
                cnt += 1;
            }
            map[lval] -= 1;
            if map[lval] == 1 {
                cnt -= 1;
            }
            if cnt == 0 {
                println!("{}", i + 1);
                break;
            }
        }
    }
}

fn main() {
    solve2();
}
