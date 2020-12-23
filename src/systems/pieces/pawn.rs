use super::*;
pub fn spawn_pawn(
    commands:&mut Commands,
    material:Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    piece_color: PieceColor,
    position: (u8, u8),
){
    commands
    // Spawn parent entity
    .spawn(PbrBundle {
        transform: Transform::from_translation(Vec3::new(
            position.0 as f32,
            0.,
            position.1 as f32
        )),
        ..Default::default()
    })
    .with(
        Piece {
            color: piece_color,
            piece_type: PieceType::Pawn,
            x: position.0,
            y: position.1,
    })
        // Add children to the parent
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh,
            material: material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
            
    });
}