use bevy::prelude::*;

fn main(){
    App::build()
    .add_resource(Msaa { samples: 4 })
    .add_resource(WindowDescriptor {
        title: "Chess!".to_string(),
        width: 1600 as f32,
        height: 1600 as f32,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(hello.system())
    .run();
}

fn hello(commands:&mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {size:8.0})),
        material: materials.add(Color::rgb(1.,0.9,0.9).into()),
        transform: Transform::from_translation(Vec3::new(4.,0.,4.)),
        ..Default::default()
    })
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