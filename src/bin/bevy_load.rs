use bevy::{platform::time::Instant, prelude::*};
use bevy_wasm_loading::*;

fn main() {
    App::new()
        .add_plugins((asset_reader::plugin, DefaultPlugins))
        .add_systems(Startup, load_assets)
        .add_systems(Update, check_assets_loaded)
        .run();
}

#[derive(Resource)]
pub struct GltfHandles {
    handles: Vec<Handle<Gltf>>,
    time_started: Instant,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let list = get_asset_list();
    info!("Total asset count: {}", list.len());
    let gltf_handles = list
        .into_iter()
        .take(20)
        .map(|asset| asset_server.load(asset))
        .collect();

    commands.insert_resource(GltfHandles {
        handles: gltf_handles,
        time_started: Instant::now(),
    });
}

fn check_assets_loaded(
    mut commands: Commands,
    gltfs: Res<Assets<Gltf>>,
    handles: Res<GltfHandles>,
) {
    for handle in &handles.handles {
        if !gltfs.contains(handle) {
            return;
        }
    }
    let elapsed = handles.time_started.elapsed();
    info!("All assets loaded! took {}ms.", elapsed.as_millis());
    commands.send_event(AppExit::Success);
}
