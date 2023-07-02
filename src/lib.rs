mod gamestate;
mod player;
mod position;

use gamestate::GameState;
use player::Player;
use position::Position;

const BOARD_SIZE: usize = 3;

pub fn solve(tpgn: &str) -> i8 {
    let root_position = Position::from_tpgn(tpgn);

    negamax(&root_position)
}

fn negamax(position: &Position) -> i8 {
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
                let score = -negamax(&next_position);
                if score > best_score {
                    best_score = score;
                }
            }
        }
    }

    best_score
}