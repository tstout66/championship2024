use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{ButtonInput, Camera2d, Commands, Component, KeyCode, PostUpdate, PreUpdate, Query, Res, Single, Transform, Update, With};


#[derive(Component)]
pub struct PlayerCamera;

impl Plugin for PlayerCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, movement);
    }

}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d::default(),
        PlayerCamera,
    ));
}

fn movement(
    mut camera_transform: Single<&mut Transform, With<PlayerCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {

    if keys.pressed(KeyCode::KeyW){
        camera_transform.translation.y += 1.0;
        println!("{}", camera_transform.translation.y);
    }
}