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