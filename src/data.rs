use bevy::prelude::*;

#[derive(Component)]
pub struct CollisionObject
{
    pub collision_boxes: Vec<CollisionBox>
}

pub struct CollisionBox
{
    pub debug_gizmos: bool,
    pub size: Vec2,
    pub offset: Vec2
}

#[derive(Component)]
pub struct Player
{
    pub movespeed: f32
}

#[derive(Component)]
pub struct Health
{
    pub max_health: f32,
    pub health: f32
}

#[derive(Component)]
pub struct Enemy
{
    pub movespeed: f32
}

#[derive(Component)]
pub struct Bullet
{
    pub speed: f32,
    pub direction: Vec3
}