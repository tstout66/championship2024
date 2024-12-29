use bevy::asset::Assets;
use bevy::color::Color;
use bevy::prelude::*;
use crate::gauge::Gauge;

#[derive(Component)]
pub struct Environment;
const MOVEMENT_FACTOR: f32 = 3.0;
impl Plugin for Environment {
    fn build(&self, app: &mut App) {

        app.add_systems(Startup, spawn_entities)
            .add_systems(Update, move_with_wind);
    }
}

fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::hsl(90., 0.95, 0.7);
    commands.spawn((
        Environment,
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(200.0, 200.0, 0.0),
    ));
}

fn move_with_wind(
    mut query: Query<&mut Transform, With<Environment>>,
    wind: Single<&Gauge>
) {
    if wind.timer.just_finished() {
        for mut transform in &mut query {

            transform.translation += Vec3::new(
                wind.wind_vec.x * MOVEMENT_FACTOR,
                wind.wind_vec.y * MOVEMENT_FACTOR,
                0.0
            );

        }
    }
}