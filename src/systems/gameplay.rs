use bevy_mod_picking::{Group, PickState, PickableMesh};
use super::pieces::{Piece, PieceColor};


enum Direction {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction{
    fn from_two_points(begin: (u8, u8), end: (u8, u8)) ->Self{
        if begin.0 < end.0 && begin.1 < end.1 {
            return Direction::UpRight;
        }
        if begin.0 < end.0 && begin.1 > end.1 {
            return Direction::UpLeft;
        }
        if begin.0 > end.0 && begin.1 < end.1 {
            return Direction::DownRight;
        }
        return Direction::DownLeft;
    }
}

pub fn color_of_square(pos:(u8, u8),pieces: &Vec<Piece>) -> Option<PieceColor> {
    for piece in pieces {
        if piece.x == pos.0 && piece.y == pos.1 {
            return Some(piece.color);
        }
    }
    None
}

pub fn is_path_empty(begin: (u8, u8), end: (u8, u8), pieces: &Vec<Piece>) -> bool {
    if begin.0 == end.0 {
        for piece in pieces {
            if piece.x == begin.0 
                && ((piece.y > begin.1 && piece.y < end.1)
                ||(piece.y > end.1 && piece.y < begin.1))
            {
                return false;
            }
        }
    }
    if begin.1 == end.1 {
        for piece in pieces {
            if piece.y == begin.1
                && ((piece.x > begin.0 && piece.x < end.0)
                    || (piece.x > end.0 && piece.x < begin.0))
            {
                return false;
            }
        }
    }
    let x_diff = (begin.0 as i8-end.0 as i8).abs();
    let y_diff = (begin.1 as i8-end.1 as i8).abs();
    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos = match Direction::from_two_points(begin, end) {
                Direction::UpRight => (begin.0 + i as u8, begin.1 +i as u8),
                Direction::DownRight => (begin.0 + i as u8, begin.1 - i as u8),
                Direction::UpLeft => (begin.0 - i as u8, begin.1 + i as u8),
                Direction::DownLeft => (begin.0 - i as u8, begin.1 - i as u8),
            };
            if color_of_square(pos, pieces).is_some() {
                return false;
            }
        }
    }
    true
}