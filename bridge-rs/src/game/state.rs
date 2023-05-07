pub enum GameState {
    Bidding(BiddingState),
    Playing(PlayingState),
}

pub struct BiddingState {

}

pub struct PlayingState {

}

impl GameState {
    pub fn new() -> Self {
        GameState::Bidding(BiddingState {})
    }
}
