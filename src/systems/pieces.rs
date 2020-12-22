use crate::*;

mod rook;
mod king;
mod knight;
mod bishop;
mod queen;
mod pawn;
pub fn create_pieces(commands:&mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let king_handle:Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle:Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh1/Primitive0");
    let pawn_handle:Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh2/Primitive0");
    let knight_1_handle:Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh3/Primitive0");
    let knight_2_handle:Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh4/Primitive0");
    let rook_handle:Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh5/Primitive0");
    let bishop_handle:Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh6/Primitive0");
    let queen_handle:Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh7/Primitive0");
    
    let white_material = materials.add(Color::rgb(1.,0.8,0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2,0.2).into());

    king::spawn_king(commands, white_material.clone(), king_handle.clone(), king_cross_handle.clone(), PieceColor::White, (0 as u8,4 as u8));
    king::spawn_king(commands, black_material.clone(), king_handle.clone(), king_cross_handle.clone(), PieceColor::Black, (7 as u8,4 as u8));
    rook::spawn_rook(commands, white_material.clone(), rook_handle.clone(), PieceColor::White, (0 as u8,0 as u8));
    rook::spawn_rook(commands, white_material.clone(), rook_handle.clone(), PieceColor::White, (0 as u8,7 as u8));
    rook::spawn_rook(commands, black_material.clone(), rook_handle.clone(), PieceColor::Black, (7 as u8,0 as u8));
    rook::spawn_rook(commands, black_material.clone(), rook_handle.clone(), PieceColor::Black, (7 as u8,7 as u8));
    knight::spawn_knight(commands, white_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), PieceColor::White, (0 as u8,1 as u8));
    knight::spawn_knight(commands, black_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), PieceColor::Black, (7 as u8,1 as u8));
    knight::spawn_knight(commands, white_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), PieceColor::White, (0 as u8,6 as u8));
    knight::spawn_knight(commands, black_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), PieceColor::Black, (7 as u8,6 as u8));
    bishop::spawn_bishop(commands, white_material.clone(), bishop_handle.clone(), PieceColor::White, (0 as u8,2 as u8));
    bishop::spawn_bishop(commands, white_material.clone(), bishop_handle.clone(), PieceColor::White, (0 as u8,5 as u8));
    bishop::spawn_bishop(commands, black_material.clone(), bishop_handle.clone(), PieceColor::Black, (7 as u8,2 as u8));
    bishop::spawn_bishop(commands, black_material.clone(), bishop_handle.clone(), PieceColor::Black, (7 as u8,5 as u8));
    queen::spawn_queen(commands, white_material.clone(), queen_handle.clone(), PieceColor::White, (0 as u8,3 as u8));
    queen::spawn_queen(commands, black_material.clone(), queen_handle.clone(), PieceColor::Black, (7 as u8,3 as u8));
    for i in 0..8 {
        pawn::spawn_pawn(commands, white_material.clone(), 
        pawn_handle.clone(),
        PieceColor::White,
        (1,i));
    }
    for i in 0..8 {
        pawn::spawn_pawn(commands, black_material.clone(), 
        pawn_handle.clone(),
        PieceColor::Black,
        (6,i));
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy)]
pub struct Piece{
    pub color: PieceColor,
    pub piece_type: PieceType,
    pub x: u8,
    pub y: u8,
}