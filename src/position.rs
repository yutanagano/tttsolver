const BOARD_SIZE: usize = 3;

#[derive(Debug, PartialEq)]
enum Player {
    Noughts,
    Crosses
}

impl Player {
    pub fn as_int(&self) -> u8 {
        match self {
            Player::Noughts => 1,
            Player::Crosses => 2
        }
    }
}

#[derive(Debug, PartialEq)]
enum GameState {
    InProgress,
    Lost(i8),
    Drawn
}

pub struct Position {
    board: [[u8;BOARD_SIZE];BOARD_SIZE],
    player: Player,
    moves_played: u8,
    state: GameState
}

impl Position {
    pub fn new() -> Self {
        Self {
            board: [[0;BOARD_SIZE];BOARD_SIZE],
            player: Player::Noughts,
            moves_played: 0,
            state: GameState::InProgress
        }
    }

    pub fn from_tpgn(tpgn: &str) -> Self {
        let mut position = Position::new();

        // Handle special root position case
        if tpgn == "R" {
            return position;
        }

        // Otherwise tpgn should have an even number of ints
        let tpgn_length = tpgn.chars().count();
        if tpgn_length % 2 != 0 {
            panic!("Incomplete tpgn: {tpgn}");
        };
        if tpgn_length > (BOARD_SIZE * BOARD_SIZE * 2) as usize{
            panic!("tpgn too long: {tpgn}");
        };

        let mut tpgn_chars = tpgn.chars();

        while let Some(char1) = tpgn_chars.next() {
            let char2 = tpgn_chars.next().unwrap();

            let x = char1.to_digit(10).expect(
                &format!("Invalid char in tpgn: {tpgn}")
            ) as usize;
            let y = char2.to_digit(10).expect(
                &format!("Invalid char in tpgn: {tpgn}")
            ) as usize;

            position = position.play(x, y);
        }

        position
    }

    pub fn can_play(&self, x: usize, y: usize) -> bool {
        match self.state {
            GameState::InProgress => self.board[x][y] == 0,
            GameState::Lost(_) => false,
            GameState::Drawn => false
        }
    }

    pub fn play(&self, x: usize, y: usize) -> Self {
        if !self.can_play(x, y) {
            panic!("can't play {x}, {y}");
        }

        let new_board = Position::update_board(&self.board, x, y, &self.player);
        let new_player = Position::switch_player(&self.player);
        let new_moves_played = self.moves_played + 1;
        let new_state = Position::get_state(&new_board, &self.player, new_moves_played);

        Position {
            board: new_board,
            player: new_player,
            moves_played: new_moves_played,
            state: new_state
        }
    }

    fn update_board(original_board: &[[u8;BOARD_SIZE];BOARD_SIZE], x: usize, y: usize, player: &Player) -> [[u8;BOARD_SIZE];BOARD_SIZE] {
        let mut new_board = *original_board;
        new_board[x][y] = player.as_int();
    
        new_board
    }

    fn switch_player(player: &Player) -> Player {
        match player {
            Player::Noughts => Player::Crosses,
            Player::Crosses => Player::Noughts
        }
    }

    fn get_state(board: &[[u8;BOARD_SIZE];BOARD_SIZE], opponent: &Player, moves_played: u8) -> GameState {
        let potential_loss_score = (moves_played as i8 + 2) / 2 - 6;
        let opponent_val = opponent.as_int();
        
        // Check for vertical wins by opponent
        for x in 0..BOARD_SIZE {
            let mut opponent_piece_count = 0;

            for y in 0..BOARD_SIZE {
                if board[x][y] != opponent_val {
                    break;
                }
                opponent_piece_count += 1;
            }

            if opponent_piece_count == BOARD_SIZE {
                //println!("vertical");
                return GameState::Lost(potential_loss_score);
            }
        }

        // Check for horizontal wins
        for y in 0..BOARD_SIZE {
            let mut opponent_piece_count = 0;

            for x in 0..BOARD_SIZE {
                if board[x][y] != opponent_val {
                    break;
                }
                opponent_piece_count += 1;
            }

            if opponent_piece_count == BOARD_SIZE {
                //println!("horizontal");
                return GameState::Lost(potential_loss_score);
            }
        }

        // Check for diagonal wins
        let mut opponent_piece_count_diag1 = 0;
        for x in 0..BOARD_SIZE {
            if board[x][x] != opponent_val {
                break;
            }
            opponent_piece_count_diag1 += 1;
        }
        if opponent_piece_count_diag1 == BOARD_SIZE {
            //println!("diag1");
            return GameState::Lost(potential_loss_score);
        }

        let mut opponent_piece_count_diag2 = 0;
        for x in 0..BOARD_SIZE {
            if board[x][BOARD_SIZE-1-x] != opponent_val {
                break;
            }
            opponent_piece_count_diag2 += 1;
        }
        if opponent_piece_count_diag2 == BOARD_SIZE {
            //println!("diag2");
            return GameState::Lost(potential_loss_score);
        }

        // Not lost and 9 moves played means drawn
        if moves_played == 9 {
            return GameState::Drawn;
        }

        // Otherwise game on!
        GameState::InProgress
    }
}

pub fn negamax(position: &Position, counter: &mut u32) -> i8 {
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

pub fn explore_and_print(position: &Position, current_tpgn: &str) {
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

#[cfg(test)]
mod tests {
    use super::{GameState, Player, Position};

    #[test]
    fn new_position_initialised_correctly() {
        let position = Position::new();

        assert_eq!(position.board, [[0;super::BOARD_SIZE];super::BOARD_SIZE]);
        assert_eq!(position.player, Player::Noughts);
        assert_eq!(position.moves_played, 0);
        assert_eq!(position.state, GameState::InProgress);
    }

    #[test]
    fn position_from_tpgn_created_correctly() {
        let position = Position::from_tpgn("001122");

        assert_eq!(
            position.board,
            [[1,0,0],
             [0,2,0],
             [0,0,1]]
        );
        assert_eq!(position.player, Player::Crosses);
        assert_eq!(position.moves_played, 3);
        assert_eq!(position.state, GameState::InProgress);
    }

    #[test]
    #[should_panic]
    fn bad_tpgn_causes_panic() {
        Position::from_tpgn("0");
    }

    #[test]
    fn test_can_play() {
        let position = Position::from_tpgn("001122");

        assert_eq!(position.can_play(0,0), false);
        assert_eq!(position.can_play(0,1), true);
    }

    #[test]
    fn test_play() {
        let position = Position::from_tpgn("001122");
        let position = position.play(0,1);

        assert_eq!(
            position.board,
            [[1,2,0],
             [0,2,0],
             [0,0,1]]
        );
        assert_eq!(position.player, Player::Noughts);
        assert_eq!(position.moves_played, 4);
        assert_eq!(position.state, GameState::InProgress);
    }

    #[test]
    #[should_panic]
    fn illegal_move_causes_panic() {
        let position = Position::from_tpgn("001122");
        position.play(0,0);
    }

    #[test]
    fn loss_detected() {
        let position = Position::from_tpgn("0001110222");

        assert_eq!(position.state, GameState::Lost(-3));
    }

    #[test]
    fn draw_detected() {
        let position = Position::from_tpgn("001122012120021210");

        assert_eq!(position.state, GameState::Drawn);
    }

    #[test]
    fn test_negamax() {
        let mut counter: u32 = 0;
        let position = Position::from_tpgn("00112201212002");

        let score = super::negamax(&position, &mut counter);

        assert_eq!(score, 0);
        assert_eq!(counter, 5);
    }
}