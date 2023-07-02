use std::io;
use tttsovler::solve;

fn main() {
    let mut line = String::new();

    while let Ok(sig) = io::stdin().read_line(&mut line) {
        if sig == 0 {
            break;
        }

        let tpgn = line.trim();
        let score = solve(tpgn);

        println!("{} {}", tpgn, score);

        line.clear();
    }
}