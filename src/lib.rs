mod position;

use position::Position;
use std::time::{Duration, Instant};

pub fn solve(tpgn: &str) -> (i8, u32, Duration) {
    let root_position = Position::from_tpgn(tpgn);
    let mut counter: u32 = 0;

    let now = Instant::now();
    
    let score = position::negamax(&root_position, &mut counter);

    let time_taken = now.elapsed();

    (score, counter, time_taken)
}

pub fn list_tpgns() {
    let root_position = Position::new();
    let current_tpgn = "";

    // Recursively explore all possible positions
    position::explore_and_print(&root_position, current_tpgn);
}