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

    king::spawn_king(commands, white_material.clone(), king_handle.clone(), king_cross_handle.clone(), Vec3::new(0.,0.,4.));
    king::spawn_king(commands, black_material.clone(), king_handle.clone(), king_cross_handle.clone(), Vec3::new(7.,0.,4.));
    rook::spawn_rook(commands, white_material.clone(), rook_handle.clone(), Vec3::new(0.,0.,0.));
    rook::spawn_rook(commands, white_material.clone(), rook_handle.clone(), Vec3::new(0.,0.,7.));
    rook::spawn_rook(commands, black_material.clone(), rook_handle.clone(), Vec3::new(7.,0.,0.));
    rook::spawn_rook(commands, black_material.clone(), rook_handle.clone(), Vec3::new(7.,0.,7.));
    knight::spawn_knight(commands, white_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(0.,0.,1.));
    knight::spawn_knight(commands, black_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(7.,0.,1.));
    knight::spawn_knight(commands, white_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(0.,0.,6.));
    knight::spawn_knight(commands, black_material.clone(), knight_1_handle.clone(), knight_2_handle.clone(), Vec3::new(7.,0.,6.));
    bishop::spawn_bishop(commands, white_material.clone(), bishop_handle.clone(), Vec3::new(0.,0.,2.));
    bishop::spawn_bishop(commands, white_material.clone(), bishop_handle.clone(), Vec3::new(0.,0.,5.));
    bishop::spawn_bishop(commands, black_material.clone(), bishop_handle.clone(), Vec3::new(7.,0.,2.));
    bishop::spawn_bishop(commands, black_material.clone(), bishop_handle.clone(), Vec3::new(7.,0.,5.));
    queen::spawn_queen(commands, white_material.clone(), queen_handle.clone(), Vec3::new(0.,0.,3.));
    queen::spawn_queen(commands, black_material.clone(), queen_handle.clone(), Vec3::new(7.,0.,3.));
    for i in 0..2 {
        for j in 0..8 {
            pawn::spawn_pawn(commands, match i {
                0 => white_material.clone(),
                1 => black_material.clone(),
                _ => white_material.clone(),
            }, pawn_handle.clone(), Vec3::new(match i {
                0 => 1.,
                1 => 6.,
                _ => 8.,
            },0.,j as f32))
        }
    }
}

