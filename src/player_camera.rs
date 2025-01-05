use bevy::prelude::*;
use crate::boat::Boat;

const CAMERA_DECAY_RATE: f32 = 1.;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, follow_boat)
        ;
    }
}

#[derive(Component)]
pub struct PlayerCamera;

fn setup(
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
    time: Res<Time>,
) {
    let boat_transform = boat.single();
    for mut camera_transform in camera_transform.iter_mut() {
        let Vec3 { x, y, .. } = boat_transform.translation;
        let new_target = Vec3::new(x, y, camera_transform.translation.z);
        camera_transform.translation.smooth_nudge(
            &new_target,
            CAMERA_DECAY_RATE,
            time.delta_secs()
        )
    }
    
}