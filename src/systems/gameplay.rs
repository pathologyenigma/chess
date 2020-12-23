use bevy_mod_picking::{Group, PickState, PickableMesh};
use super::pieces::{Piece, PieceColor, PieceType};


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

impl Piece {
    pub fn is_move_valid(&self, new_postion: (u8, u8), pieces: Vec<Piece>) -> bool {
        if color_of_square(new_postion, &pieces) == Some(self.color) {
            return false;
        }
        let horizontal_any = self.y == new_postion.1&& self.x != new_postion.0;
        let vertical_any = self.x == new_postion.0&& self.y != new_postion.1;
        let diagonal_any = (self.x as i8 -new_postion.0 as i8).abs() == (self.y as i8 -new_postion.1 as i8).abs();
        let path_avaliable = is_path_empty((self.x,self.y), new_postion, &pieces);
        let diagonal_1_abs = (self.x as i8 -new_postion.0 as i8).abs() == 1 &&(self.y as i8 -new_postion.1 as i8).abs()==1;
        match self.piece_type {
            PieceType::King => {
                horizontal_some((self.x,self.y),new_postion,1) ||vertical_some((self.x,self.y),new_postion,1,true) || diagonal_1_abs
            }
            PieceType::Queen => {
                path_avaliable && (horizontal_any || vertical_any || diagonal_any)
            }
            PieceType::Bishop => {
                path_avaliable && diagonal_any
            }
            PieceType::Knight => {
                kinght_movable((self.x,self.y),new_postion)
            }
            PieceType::Rook => {
                path_avaliable && (horizontal_any || vertical_any)
            }
            PieceType::Pawn => {
               
                match self.color {
                    PieceColor::White => {
                        if vertical_some((self.x,self.y), new_postion, -1,false) {
                            if !already_some_one(new_postion,&pieces) {
                                return true;
                            }
                        }
                        if self.x==1 && vertical_some((self.x,self.y), new_postion, 2,false) && path_avaliable {
                            if !already_some_one(new_postion,&pieces) {
                                return true;
                            }
                        }
                        if there_is_a_avaliable_take( new_postion,&pieces,self.color) &&diagonal_1((self.x,self.y),new_postion,true){
                            return true;
                        }
                        
                        false
                    }
                    PieceColor::Black => {
                        if vertical_some((self.x,self.y), new_postion, 1,false) {
                            if !already_some_one(new_postion,&pieces) {
                                return true;
                            }
                        }
                        if self.x==6 && vertical_some((self.x,self.y), new_postion, -2,false) && path_avaliable {
                            if !already_some_one(new_postion,&pieces) {
                                return true;
                            }
                        }
                        if there_is_a_avaliable_take( new_postion,&pieces,self.color) &&diagonal_1((self.x,self.y),new_postion,false){
                            return true;
                        }
                        false
                    }
                }
            }
        }
    }
}

fn vertical_some(begin: (u8, u8),end: (u8, u8),step: i8,abs: bool) -> bool{
    match abs {
        true => (begin.0 as i8- end.0 as i8).abs() == step &&(begin.1==end.1),
        false=> (begin.0 as i8- end.0 as i8) == step &&(begin.1==end.1)
    }
    
}
fn horizontal_some(begin: (u8, u8),end: (u8, u8),step: i8) -> bool{
    (begin.1 as i8 -end.1 as i8).abs()== step &&(begin.0==end.0)
}
fn already_some_one(pos:(u8, u8),pieces: &Vec<Piece>) ->bool{
    !color_of_square(pos, pieces).is_none()
}
fn there_is_a_avaliable_take(pos: (u8, u8),pieces: &Vec<Piece>,color:PieceColor) ->bool{
    match color {
        PieceColor::White=>color_of_square(pos, pieces) == Some(PieceColor::Black),
        PieceColor::Black=>color_of_square(pos, pieces) == Some(PieceColor::White),
    }
}
fn diagonal_1(begin: (u8, u8),end: (u8, u8),positive: bool) ->bool{
    match positive{
        true=> (begin.0 as i8 -end.0 as i8) == -1 &&(begin.1 as i8 -end.1 as i8).abs()==1,
        false=> (begin.0 as i8 -end.0 as i8) == 1 &&(begin.1 as i8 -end.1 as i8).abs()==1,
    }
}
fn kinght_movable(begin: (u8, u8),end: (u8, u8)) -> bool{
    (begin.0 as i8- end.0 as i8).abs() == 1 && (begin.1 as i8 -end.1 as i8).abs()== 2
    || (begin.0 as i8- end.0 as i8).abs() == 2 && (begin.1 as i8 -end.1 as i8).abs()== 1
}



