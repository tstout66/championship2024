use bevy::app::Plugin;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::{vec2, Vec3};
use bevy::prelude::*;

#[derive(Component)]
pub struct Gauge;

impl Plugin for Gauge {
    fn build(&self, app: &mut App) {
       // app.add_systems(Startup, spawn_entities)
        app.add_systems(Startup, wind_gauge_ui);
    }

}
fn wind_gauge(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut gauge_transform: Single<&mut Transform, With<Gauge>>
) {
    let (camera, camera_transform) = *camera_query;
    let Ok(point) = camera.viewport_to_world_2d(
        camera_transform,
        vec2(100.0, 100.0),
    ) else {
        return;
    };
    gauge_transform.translation = Vec3::new(point.x, point.y, 0.0);

}

fn wind_gauge_ui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::hsl(180., 0.95, 0.7);

    commands.spawn((
        Gauge,
        Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
        MeshMaterial2d(materials.add(color)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            left: Val::Px(50.0),
            // width: Val::Percent(50.0),
            // height: Val::Percent(50.0),
            // justify_content: JustifyContent::Center,
            ..default()
        },
    ));
}


// fn spawn_entities(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let color = Color::hsl(180., 0.95, 0.7);
//     commands.spawn((
//         Gauge,
//         Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
//         MeshMaterial2d(materials.add(color)),
//     ));
// }