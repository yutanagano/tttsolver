mod gamestate;
mod player;
mod position;

use gamestate::GameState;
use player::Player;
use position::Position;

use std::time::{Duration, Instant};

const BOARD_SIZE: usize = 3;

pub fn solve(tpgn: &str) -> (i8, u32, Duration) {
    let root_position = Position::from_tpgn(tpgn);
    let mut counter: u32 = 0;

    let now = Instant::now();
    
    let score = negamax(&root_position, &mut counter);

    let time_taken = now.elapsed();

    (score, counter, time_taken)
}

pub fn list_tpgns() {
    let root_position = Position::new();
    let current_tpgn = "";

    // Recursively explore all possible positions
    explore_and_print(&root_position, current_tpgn);
}

fn explore_and_print(position: &Position, current_tpgn: &str) {
    // Handle special root case
    if current_tpgn == "" {
        println!("R");
    } else {
        println!("{current_tpgn}");
    }

    // Exit at terminal state
    match position.state {
        GameState::InProgress => (),
        _ => return
    }

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if position.can_play(x, y) {
                let next_position = position.play(x, y);
                let next_tpgn = current_tpgn.to_string() + &format!("{x}{y}");
                explore_and_print(&next_position, &next_tpgn);
            }
        }
    }
}

fn negamax(position: &Position, counter: &mut u32) -> i8 {
    *counter += 1;

    // Exit at terminal state
    match position.state {
        GameState::Drawn => return 0,
        GameState::Lost(score) => return score,
        GameState::InProgress => ()
    }

    // Continue tree search until terminal state
    let mut best_score = -1 * BOARD_SIZE as i8;

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if position.can_play(x, y) {
                let next_position = position.play(x, y);
                let score = -negamax(&next_position, counter);
                if score > best_score {
                    best_score = score;
                }
            }
        }
    }

    best_score
}