use std::{fmt::{Debug, Display}, num::NonZeroUsize};
use meet_in_the_middle::State;
use transition::{Move, ALL_MOVES};

mod transition;

// Rectangle where top-left is 0/0 and bottom-right is x/y
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct PositionInRectangle<'a> {
    size: &'a RectangleSize,
    x: usize,
    y: usize,
}

impl<'a> PositionInRectangle<'a> {
    pub fn new(size: &'a RectangleSize, x: usize, y: usize) -> PositionInRectangle<'a> {
        PositionInRectangle { size, x, y }
    }
}

impl<'a> Debug for PositionInRectangle<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PositionInRectangle")
            .field("size", &self.size)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("distance_from_origin", &(self.x + self.y))
            .finish()
    }
}

impl<'a> Display for PositionInRectangle<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PositionInRectangle(x: {}, y: {})", self.x, self.y)
    }
}

impl<'a> State for PositionInRectangle<'a> {
    type Transition = Move;

    fn apply(&self, change: &Self::Transition) -> Self {
        PositionInRectangle {
            size: self.size,
            x: match change {
                Move::Left => self.x - 1,
                Move::Right => self.x + 1,
                _ => self.x
            },
            y: match change {
                Move::Up => self.y - 1,
                Move::Down => self.y + 1,
                _ => self.y
            }
        }
    }

    fn get_possible_transitions(&self) -> impl Iterator<Item = &Self::Transition> {
        PossibleTransitionIterator::new(self.clone())
    }
}

struct PossibleTransitionIterator<'a> {
    pos: PositionInRectangle<'a>,
    next_check: usize,
}

impl<'a> PossibleTransitionIterator<'a> {
    fn new(pos: PositionInRectangle<'a>) -> PossibleTransitionIterator<'a> {
        PossibleTransitionIterator { pos, next_check: 0 }
    }
}

impl<'a> Iterator for PossibleTransitionIterator<'a> {
    type Item = &'a Move;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(m) = ALL_MOVES.get(self.next_check) {
            self.next_check += 1;
            if m.allowed(&self.pos) {
                return Some(m);
            }
        }

        None
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RectangleSize {
    height: NonZeroUsize,
    width: NonZeroUsize
}

impl RectangleSize {
    pub fn new(height: NonZeroUsize, width: NonZeroUsize) -> RectangleSize {
        RectangleSize { height, width }
    }
}
