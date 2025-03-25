use std::{fmt::{Debug, Display}, num::NonZeroUsize};

use meet_in_the_middle::{find_path, State};

fn main() {
    let rectangle_size = RectangleSize {
        height: 2000.try_into().unwrap(),
        width: 3000.try_into().unwrap()
    };

    let source = PositionInRectangle {
        size: &rectangle_size,
        x: 57,
        y: 234
    };

    let target = PositionInRectangle {
        size: &rectangle_size,
        x: 2763,
        y: 1467
    };

    let path: Vec<_> = find_path(&source, &target).into_iter().map(|n| n.to_string()).collect();
    println!("Path found with length: {:?}", path.len());
    println!("Start: {:?}, End: {:?}", path.iter().take(4).collect::<Vec<_>>(), path.iter().rev().take(4).rev().collect::<Vec<_>>());
}

// Rectangle where top-left is 0/0 and bottom-right is x/y
#[derive(PartialEq, Eq, Hash, Clone)]
struct PositionInRectangle<'a> {
    size: &'a RectangleSize,
    
    x: usize,
    y: usize,
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
struct RectangleSize {
    height: NonZeroUsize,
    width: NonZeroUsize
}

enum Move {
    Left,
    Up,
    Right,
    Down
}

static ALL_MOVES: [Move; 4] = [Move::Left, Move::Up, Move::Right, Move::Down];

impl Move {
    fn allowed(&self, pos: &PositionInRectangle) -> bool {
        match &self {
            Move::Left => pos.x > 0,
            Move::Up => pos.y > 0,
            Move::Right => pos.x < pos.size.width.get() - 1,
            Move::Down => pos.y < pos.size.height.get() - 1
        }
    }
}