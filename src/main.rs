use bevy::prelude::*;

pub mod data;
pub mod systems;
pub mod plugin;

use plugin::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()), CameraPlugin, PlayerPlugin, GunPlugin, EnemyPlugin))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.8, 0.1)))
        .add_plugins(())
        .run();
}   

