use bevy::asset::Assets;
use bevy::color::Color;
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

fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::hsl(90., 0.95, 0.7);
    // commands.spawn((
    //     Environment,
    //     Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
    //     MeshMaterial2d(materials.add(color)),
    //     Transform::from_xyz(200.0, 200.0, 0.0),
    // ));
    commands.spawn((
        Environment,
        Sprite {
            color,
            custom_size: Some(Vec2::new(0.0, 0.0)),
            ..default()
        },
        Transform::from_xyz(100., 100., 0.0),
    ));

}

fn move_with_wind(
    mut query: Query<(&mut Transform, &mut Sprite), With<Environment>>,
    wind: Single<&Gauge>
) {
    if wind.timer.just_finished() {
        for (mut transform, mut sprite) in &mut query {
            let extended_vector = wind.wind_vec.extend(0.0) * MOVEMENT_FACTOR;
            let length = transform.translation.distance(transform.translation + extended_vector);
            let diff = transform.translation - (transform.translation + extended_vector);
            let theta = diff.y.atan2(diff.x);
            let midpoint = (transform.translation + (transform.translation - extended_vector)) / 2.;


            transform.translation = Vec3::new(midpoint.x, midpoint.y, 0.);
            transform.rotation = Quat::from_rotation_z(theta);
            // transform = transform.with_rotation(Quat::from_rotation_z(theta));
            sprite.custom_size = Some(Vec2::new(length, 2.0));
            println!("{}", theta)

            // transform.translation += Vec3::new(
            //     wind.wind_vec.x * MOVEMENT_FACTOR,
            //     wind.wind_vec.y * MOVEMENT_FACTOR,
            //     0.0
            // );
        }
    }
}

fn line_segment(
    start: Vec2,
    end: Vec2,
    thickness: f32,
    color: Color,
) -> impl Bundle {
    let length = start.distance(end);
    let diff = start - end;
    let theta = diff.y.atan2(diff.x);
    let midpoint = (start + end) / 2.;

    let transform = Transform::from_xyz(midpoint.x, midpoint.y, 0.)
        .with_rotation(Quat::from_rotation_z(theta));

    (
        Sprite {
            color,
            custom_size: Some(Vec2::new(length, thickness)),
            ..default()
        },
        transform,
    )
}