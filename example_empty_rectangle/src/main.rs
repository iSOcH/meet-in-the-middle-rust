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

    fn get_possible_transitions(&self) -> Vec<Self::Transition> {
        let mut possible_moves = vec![];

        if self.x > 0 {
            possible_moves.push(Move::Left);
        }

        if self.x < self.size.width.get() - 1 {
            possible_moves.push(Move::Right);
        }

        if self.y > 0 {
            possible_moves.push(Move::Up);
        }

        if self.y < self.size.height.get() - 1 {
            possible_moves.push(Move::Down);
        }

        possible_moves
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