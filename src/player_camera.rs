﻿use bevy::prelude::*;
use crate::gauge::Gauge;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(PreUpdate, movement);
    }

}

#[derive(Component)]
pub struct PlayerCamera;

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d,
        PlayerCamera,
    ));
}

fn movement(
    mut camera_transform: Query<&mut Transform, Or<(With<PlayerCamera>, With<Gauge>)>>,
    keys: Res<ButtonInput<KeyCode>>,
) {

    if keys.pressed(KeyCode::KeyW){
        for mut camera_transform in camera_transform.iter_mut() {
            camera_transform.translation.y += 1.0;
        }

    }
}