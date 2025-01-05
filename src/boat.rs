use bevy::prelude::*;
use crate::wind::Wind;

pub struct BoatPlugin;

impl Plugin for BoatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (wind_effects, sail_control, boat_control));
    }
}

#[derive(Component)]
#[require(Sprite)]
pub struct Boat;

#[derive(Component)]
#[require(Sprite)]
pub struct BoatSail;

fn setup(mut commands: Commands) {
    let color = Color::WHITE;
    commands.spawn((
        Boat,
        Transform::from_xyz(0., 0., 0.),
        Sprite {
            color: Color::WHITE.with_alpha(0.5),
            custom_size: Some(Vec2::new(10.0, 40.0)),
            ..default()
        },
    ))
        .with_children(|parent| {
            parent.spawn((
                BoatSail,
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(30., 3.)),
                    ..default()
                },
                Transform::from_xyz(0., 10., 0.),
            ));
        });
}

fn wind_effects(
    mut boats: Query<&mut Transform, With<Boat>>,
    wind: Res<Wind>,
) {
    if wind.timer.just_finished() {
        for mut boat in boats.iter_mut() {
            let projected_x_pos = (boat.translation.x / wind.scale) as f64;
            let projected_y_pos = (boat.translation.y / wind.scale) as f64;

            boat.translation.x += (wind.perlin_x.get2d([
                projected_x_pos + wind.offset.x as f64,
                projected_y_pos + wind.offset.y as f64,
            ]) as f32
                - 0.5) * 10.0;
            boat.translation.y += (wind.perlin_y.get2d([
                projected_x_pos + wind.offset.x as f64,
                projected_y_pos + wind.offset.y as f64,
            ]) as f32
                - 0.5) * 10.0;
        }
    }
}

fn sail_control (
    mut boat_sail_transform: Query<&mut Transform, With<BoatSail>>,
    keys: Res<ButtonInput<KeyCode>>,
) {

    if keys.pressed(KeyCode::KeyQ){
        for mut transform in boat_sail_transform.iter_mut() {
            transform.rotate_local_z(0.01);
        }
    }
    if keys.pressed(KeyCode::KeyE){
        for mut transform in boat_sail_transform.iter_mut() {
            transform.rotate_local_z(-0.01);
        }
    }
}

fn boat_control (
    mut boats: Query<&mut Transform, With<Boat>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::KeyA){
        for mut transform in boats.iter_mut() {
            transform.rotate_local_z(0.01);
        }
    }
    if keys.pressed(KeyCode::KeyD){
        for mut transform in boats.iter_mut() {
            transform.rotate_local_z(-0.01);
        }
    }
}