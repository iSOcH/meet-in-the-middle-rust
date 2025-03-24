use std::fmt::Debug;

use meet_in_the_middle::{Solver, State};

fn main() {
    let rectangle_size = RectangleSize {
        height: 10000,
        width: 20000
    };

    let mut solver = Solver::new(
        PositionInRectangle {
            size: &rectangle_size,
            x: 0,
            y: 0
        },
        PositionInRectangle {
            size: &rectangle_size,
            x: 15000,
            y: 7500
        });

    solver.run();
}

// Rectangle where top-left is 0/0 and bottom-right is x/y
#[derive(PartialEq, Eq, Hash)]
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

        if self.x < self.size.width - 1 {
            possible_moves.push(Move::Right);
        }

        if self.y > 0 {
            possible_moves.push(Move::Up);
        }

        if self.y < self.size.height - 1 {
            possible_moves.push(Move::Down);
        }

        possible_moves
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct RectangleSize {
    height: usize,
    width: usize
}

enum Move {
    Left,
    Up,
    Right,
    Down
}