use crate::*;
pub fn spawn_queen(
    commands:&mut Commands,
    material:Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
){
    commands
    // Spawn parent entity
    .spawn(PbrBundle {
        transform: Transform::from_translation(position),
        ..Default::default()
    })
        // Add children to the parent
    .with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh,
            material: material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..Default::default()
        });
            
    });
}