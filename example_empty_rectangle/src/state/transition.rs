use super::PositionInRectangle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move {
    Left,
    Up,
    Right,
    Down
}

pub static ALL_MOVES: [Move; 4] = [Move::Left, Move::Up, Move::Right, Move::Down];

impl Move {
    pub fn allowed(&self, pos: &PositionInRectangle) -> bool {
        match &self {
            Move::Left => pos.x > 0,
            Move::Up => pos.y > 0,
            Move::Right => pos.x < pos.size.width.get() - 1,
            Move::Down => pos.y < pos.size.height.get() - 1
        }
    }
}
