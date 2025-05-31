use bevy::prelude::*;
use bevy_wasm_loading::*;

fn main() {
    App::new()
        .add_plugins((asset_reader::plugin, DefaultPlugins))
        .run();
}

//fn load_alotta_resources()
