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
struct TimeStart(Instant);

#[derive(Component)]
pub struct GltfHandle(Handle<Gltf>);

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let list = get_asset_list();
    info!("Total asset count: {}", list.len());

    let gltf_handles = list
        .into_iter()
        .map(|asset| GltfHandle(asset_server.load(asset)))
        .collect::<Vec<_>>();

    commands.spawn_batch(gltf_handles);
    commands.insert_resource(TimeStart(Instant::now()));
}

fn check_assets_loaded(
    mut commands: Commands,
    gltfs: Res<Assets<Gltf>>,
    handles: Query<&GltfHandle>,
    time_started: Res<TimeStart>,
) {
    for handle in &handles {
        if !gltfs.contains(&handle.0) {
            return;
        }
    }
    let elapsed = time_started.0.elapsed();
    info!("All assets loaded! took {}ms.", elapsed.as_millis());
    commands.send_event(AppExit::Success);
}
