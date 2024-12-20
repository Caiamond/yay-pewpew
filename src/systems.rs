use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::data::*;


pub fn hello_world()
{
    println!("hello world");
}

pub fn add_camera(mut commands: Commands)
{
    commands.spawn(
        Camera2d{
            ..Default::default()
        }
    );
}

pub fn add_player(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn((
        Player{
            movespeed: 250.0
        },
        Sprite{
            image: asset_server.load("sprites/yay.png"),
            custom_size: Some(Vec2::new(50., 50.)),
            ..Default::default()
        }
    ));
}

pub fn add_enemy(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn((
        Enemy{
            movespeed: 200.0
        },
        Health {
            max_health: 10.0,
            health: 10.0
        },
        Transform::from_xyz(200.0, 200.0, 0.0),
        Sprite{
            image: asset_server.load("sprites/aya.png"),
            custom_size: Some(Vec2::new(50., 50.)),
            ..Default::default()
        }
    ));
}

pub fn move_enemy(time: Res<Time>, player_query: Query<&mut Transform, With<Player>> , mut enemy_query: Query<(&mut Transform, &mut Sprite, &mut Enemy), Without<Player>>)
{
    let player_position: Vec3;

    if let Ok(player_transform) = player_query.get_single()
    {
        player_position = player_transform.translation;
    }
    else
    {
        return;    
    }

    for (mut transform, mut sprite, enemy) in &mut enemy_query
    {
        let move_direction: Vec3 = player_position - transform.translation;

        if move_direction.length() > 10.0
        {
            transform.translation += move_direction.normalize() * enemy.movespeed * time.delta_secs();
            if move_direction.x > 0.0
            {
                sprite.flip_x = false;
            }
            else if move_direction.x < 0.0
            {
                sprite.flip_x = true;    
            }
        }
    }
}

pub fn move_player(keyboard: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut player_query: Query<(&mut Transform, &mut Sprite, &mut Player)>)
{
    let mut direction = Vec3::ZERO;     
    
    for (mut transform, mut sprite, player) in &mut player_query
    {
        if keyboard.pressed(KeyCode::KeyW)
        {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard.pressed(KeyCode::KeyA)
            {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard.pressed(KeyCode::KeyS)
        {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard.pressed(KeyCode::KeyD)
        {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if direction.length() > 0.0
        {
            direction = direction.normalize();

            if direction.x > 0.0
            {
                sprite.flip_x = false;
            }
            else if direction.x < 0.0
            {
                sprite.flip_x = true;
            }
        }

        transform.translation += direction * player.movespeed * time.delta_secs();
    }
}

pub fn shoot_gun(mut commands: Commands, asset_server: Res<AssetServer>, mouse: Res<ButtonInput<MouseButton>>, player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>)
{
    let Ok(player_transform) = player_query.get_single() else { return; };
    let Ok(window) = window_query.get_single() else { return; };
    let Some(mut cursor_position) = window.cursor_position() else { return; };

    if mouse.just_pressed(MouseButton::Left)
    {
        let player_position: Vec3 = player_transform.translation;
        cursor_position.x -= window.width()/2.0;
        cursor_position.y = -cursor_position.y + window.height()/2.0; 
        let bullet_direction:Vec3 = (cursor_position.extend(0.0) - player_position).normalize();
        commands.spawn((
            Transform::from_xyz(player_position.x, player_position.y, player_position.z),
            Sprite {
                image: asset_server.load("sprites/bullet.png"),
                custom_size: Some(Vec2::new(10., 10.)),
                ..Default::default()
            },
            Bullet {
                speed: 750.0,
                direction: bullet_direction
            }
        ));
    }
}

pub fn move_bullet(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>)
{
    for (mut transform, bullet) in &mut bullet_query
    {
        transform.translation += bullet.direction * bullet.speed * time.delta_secs();
    }
}

