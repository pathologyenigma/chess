use bevy::prelude::*;
use my_bevy_game::systems::*;

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
    .add_startup_system(setup.system())
    .add_startup_system(board::create_board.system())
    .add_startup_system(pieces::create_pieces.system())
    .run();
}





