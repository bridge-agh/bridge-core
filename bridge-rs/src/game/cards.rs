#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub enum Face {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Card(Face, Suit);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching() {
        let card = Card(Face::Two, Suit::Clubs);
        assert!(matches!(card, Card(Face::Two, _suit)));
        assert!(matches!(card, Card(_value, Suit::Clubs)));
        assert!(matches!(card, Card(Face::Two, Suit::Clubs)));
    }

    #[test]
    fn test_ranking_face() {
        assert!(Face::Five < Face::Six);
        assert!(Face::Five > Face::Four);
        assert!(Face::Five == Face::Five);
        assert!(Face::Five != Face::Six);
    }

    #[test]
    fn test_ranking_suit() {
        assert!(Suit::Clubs < Suit::Diamonds);
        assert!(Suit::Clubs < Suit::Hearts);
        assert!(Suit::Clubs < Suit::Spades);
        assert!(Suit::Diamonds < Suit::Hearts);
        assert!(Suit::Diamonds < Suit::Spades);
        assert!(Suit::Hearts < Suit::Spades);
        assert!(Suit::Clubs == Suit::Clubs);
        assert!(Suit::Clubs != Suit::Diamonds);
    }
}
