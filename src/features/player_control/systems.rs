use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::features::tile::components::{Temperature, Tile};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Transform::default(), 
        Camera2d::default(),
        Camera::default(),
    ));
}

pub fn camera_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut scroll: MessageReader<MouseWheel>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    let mut direction = Vec3::ZERO;

    let speed = 400.0;
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    camera.translation += direction * speed * time.delta_secs();

    for ev in scroll.read() {
        let zoom_speed = 0.1;
        camera.scale *= 1.0 - ev.y * zoom_speed;
        camera.scale = camera.scale.clamp(Vec3::splat(0.2), Vec3::splat(5.0));
    }
}

pub fn click_tile(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_q: Single<(&Camera, &GlobalTransform)>,
    tiles: Query<(&Tile, &Temperature)>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = *camera_q;

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            let tile_size = 32.0;
            let x = (world_pos.x / tile_size).round() as i32;
            let y = (world_pos.y / tile_size).round() as i32;

            for (tile, temp) in &tiles {
                if tile.x == x && tile.y == y {
                    println!(
                        "Clicked Tile ({}, {}): {:?}, Temp: {}°C",
                        tile.x, tile.y, tile.material, temp.0
                    );
                    return;
                }
            }
            println!("No tile found at ({}, {})", x, y);
        }
    }
}
