use bevy::prelude::*;

use crate::systems::*;
//use crate::data;

pub struct CameraPlugin;

impl Plugin for CameraPlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera);
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_player);
        app.add_systems(Update, move_player);
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_enemy);
        app.add_systems(Update, move_enemy);
    }
}

pub struct GunPlugin;

impl Plugin for GunPlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (shoot_gun, move_bullet));
    }
}

