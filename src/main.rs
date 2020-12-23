use bevy::prelude::*;
use bevy_mod_picking::{DebugPickingPlugin, PickingPlugin};
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
    .add_startup_system(setup.system())
    .add_plugins(DefaultPlugins)
    .add_plugin(PickingPlugin)
    //.add_plugin(DebugPickingPlugin)
    .add_plugin(board::BoardPlugin)
    .add_plugin(pieces::PiecesPlugin)
    .add_plugin(ui::UIPlugin)
    
    .run();
}





