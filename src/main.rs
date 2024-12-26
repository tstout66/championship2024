use bevy::math::vec2;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_entities)
        .add_systems(Update, wind_gauge)
        .run();
}
#[derive(Component)]
struct Gauge;

fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let color = Color::hsl(180., 0.95, 0.7);
    commands.spawn((
        Gauge,
        Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
        MeshMaterial2d(materials.add(color)),
        // Transform::from_xyz(
        //     100.0,
        //     0.0,
        //     0.0,
        // ),
    ));
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
    //let (mut gauge_transform) = gauge_query;
    gauge_transform.translation = Vec3::new(point.x, point.y, 0.0);

}
