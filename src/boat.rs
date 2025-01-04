use bevy::prelude::*;
use crate::gauge::Wind;

pub struct BoatPlugin;

impl Plugin for BoatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, wind_effects);
    }
}

#[derive(Component)]
pub struct Boat;

fn setup(mut commands: Commands) {
    let color = Color::WHITE;
    commands.spawn((
        Boat,
        Sprite {
            color,
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(0., 0., 0.0),
    ));
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