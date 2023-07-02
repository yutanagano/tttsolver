use super::{GameState, Player, BOARD_SIZE};

pub struct Position {
    pub board: [[u8;BOARD_SIZE];BOARD_SIZE],
    pub player: Player,
    pub moves_played: u8,
    pub state: GameState
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
        let potential_loss_score = (moves_played as i8 + 1) / 2 - 6;
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