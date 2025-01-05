use bevy::prelude::*;
use crate::boat::Boat;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, follow_boat)
        ;
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

fn follow_boat (
    mut camera_transform: Query<&mut Transform, With<PlayerCamera>>,
    boat: Query<&Transform, (With<Boat>, Without<PlayerCamera>)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    
    if keys.pressed(KeyCode::KeyW){
        let boat_transform = boat.single();
        for mut camera_transform in camera_transform.iter_mut() {
            camera_transform.translation.y = boat_transform.translation.y;
            camera_transform.translation.x = boat_transform.translation.x;
        }
    }
}