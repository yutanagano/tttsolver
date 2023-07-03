use std::io;
use tttsovler::solve;

fn main() {
    let mut line = String::new();

    while let Ok(sig) = io::stdin().read_line(&mut line) {
        if sig == 0 {
            break;
        }

        let tpgn = line.trim();
        let (score, num_positions_visited, time_taken) = solve(tpgn);

        println!("{} {} {} {}", tpgn, score, time_taken.as_micros(), num_positions_visited);

        line.clear();
    }
}