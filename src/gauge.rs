use bevy::app::Plugin;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Curve};

pub struct GaugePlugin;

#[derive(Component)]
pub struct Gauge {
    pub curve_perlin: noise::Curve<T, Source, DIM>,
}

impl Plugin for GaugePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_gauge)
        .add_systems(Startup, text_ui)
        .add_systems(FixedUpdate, update_wind);
    }
}

fn text_ui(mut commands: Commands) {
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
        Gauge{
            perlin: Perlin::default()
        },
        Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
        MeshMaterial2d(materials.add(color)),
    ));
}

fn update_wind(
    wind_gauge_perlin: Single<&Gauge>,
) {
    println!("{:.32}", wind_gauge_perlin.perlin.get([3.0, 3.0, 4.0]));
}

