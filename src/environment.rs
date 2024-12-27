use bevy::app::{App, Plugin, Startup};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::prelude::{ColorMaterial, Commands, Mesh, Mesh2d, MeshMaterial2d, Rectangle, ResMut, Transform};

pub struct Environment;

impl Plugin for Environment {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_entities);
    }
}

fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::hsl(90., 0.95, 0.7);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(200.0, 200.0, 0.0),
    ));
}