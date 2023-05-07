pub enum Player {
    North,
    East,
    South,
    West,
}

impl Player {
    pub fn next(&self) -> Player {
        match self {
            Player::North => Player::East,
            Player::East => Player::South,
            Player::South => Player::West,
            Player::West => Player::North,
        }
    }
}
