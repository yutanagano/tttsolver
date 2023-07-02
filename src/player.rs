pub enum Player {
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