use bevy::prelude::*;
use crate::gauge::Gauge;

#[derive(Component)]
pub struct Environment;

const MOVEMENT_FACTOR: f32 = 50.0;

impl Plugin for Environment {
    fn build(&self, app: &mut App) {

        app.add_systems(Startup, spawn_entities)
            .add_systems(Update, move_with_wind);
    }
}

#[derive(Component)]
pub struct SpawnPoint {
    pub pos: Vec3,
}

fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::hsl(90., 0.95, 0.7);
    commands.spawn((
        Environment,
        SpawnPoint {
            pos: Vec3::new(50.,50., 50.),
        },
        Sprite {
            color,
            custom_size: Some(Vec2::new(0.0, 0.0)),
            ..default()
        },
        Transform::from_xyz(0., 0., 0.0),
    ));
    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..default()
        },
        Transform::from_xyz(50., 50., 50.),
    ));

}

fn move_with_wind(
    mut query: Query<(&SpawnPoint, &mut Transform, &mut Sprite), With<Environment>>,
    wind: Single<&Gauge>
) {
    if wind.timer.just_finished() {
        for (spawn_point, mut transform, mut sprite) in &mut query {
            let extended_vector = wind.wind_vec.extend(0.0) * MOVEMENT_FACTOR;
            let length = transform.translation.distance(transform.translation + extended_vector);
            let diff = transform.translation - (transform.translation + extended_vector);
            let theta = diff.y.atan2(diff.x);
            let midpoint = (transform.translation + (transform.translation - extended_vector)) / 2.;

            transform.rotation = Quat::from_rotation_z(theta);
            transform.translation = spawn_point.pos + transform.local_x().as_vec3() * length/2.0;
            sprite.custom_size = Some(Vec2::new(length, 2.0));
        }
    }
}
