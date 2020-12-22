pub mod board;
pub mod pieces;
use crate::*;
pub fn setup(commands:&mut Commands) {
    commands
    .spawn(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3,-0.5,-0.3,0.5).normalize(),
            Vec3::new(-7.0,20.0,4.0),
        )),
        ..Default::default()
    })
    .spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0,8.0,4.0)),
        ..Default::default()
    });
}