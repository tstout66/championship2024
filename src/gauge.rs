use std::time::Duration;
use bevy::prelude::*;
use perlin_noise::PerlinNoise;

pub struct GaugePlugin;

#[derive(Component)]
pub struct Gauge {
    pub timer: Timer,
    pub wind_vec: Vec2,
    pub wind_seed: Vec2,
    pub perlin: PerlinNoise,
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
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::hsl(180., 0.95, 0.7);

    commands.spawn((
        Gauge{
            timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
            wind_vec: Vec2::ZERO,
            wind_seed: Vec2{x: rand::random::<f32>(), y: rand::random::<f32>()},
            perlin: PerlinNoise::new()
        },
        // Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
        // MeshMaterial2d(materials.add(color)),
    ));
}

fn update_wind(
    mut gauge: Single<&mut Gauge>,
    mut text: Single<&mut Text>,
    time: Res<Time>,
) {
    gauge.timer.tick(time.delta());
    if gauge.timer.just_finished() {
        gauge.wind_seed += 0.01;
        gauge.wind_vec.x = gauge.perlin.get(gauge.wind_seed.x as f64) as f32 - 0.5;
        gauge.wind_vec.y = gauge.perlin.get(gauge.wind_seed.y as f64) as f32 - 0.5;
        text.0 = format!("Wind Direction: {}", gauge.wind_vec);
    }

}

