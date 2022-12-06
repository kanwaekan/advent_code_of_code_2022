use std::io::stdin;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut lines = String::new();
    let mut cals = Vec::new();
    stdin().read_to_string(&mut lines)?;

    let (mut prev, mut curindex) = (0, 1);
    for item in lines.split("\n") {
        match item.trim().parse::<i32>() {
            Ok(val) => {
                prev += val;
            }
            Err(_) => {
                cals.push((curindex, prev));
                curindex += 1;
                prev = 0;
            }
        }
    }

    cals.sort_by(|a, b| b.1.cmp(&a.1));
    print!("{}", cals[0].1 + cals[1].1 + cals[2].1);
    Ok(())
}
