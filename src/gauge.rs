use bevy::app::Plugin;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::prelude::*;

#[derive(Component)]
pub struct Gauge;

impl Plugin for Gauge {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_gauge)
        .add_systems(Startup, text_ui);
    }
}

fn text_ui(
    mut commands: Commands,
) {
    commands.spawn((
        Text::new("Test"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            left: Val::Px(50.0),
            ..default()
        },
    ));
}

fn spawn_gauge(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::hsl(180., 0.95, 0.7);
    commands.spawn((
        Gauge,
        Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
        MeshMaterial2d(materials.add(color)),
    ));
}

