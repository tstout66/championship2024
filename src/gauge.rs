use std::time::Duration;
use bevy::prelude::*;
use perlin_noise::PerlinNoise;

const MOVEMENT_FACTOR: f32 = 50.0;

pub struct GaugePlugin;

impl Plugin for GaugePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_gauge)
        .add_systems(Startup, text_ui)
        .add_systems(Update, update_wind)
        .add_systems(Update, move_with_wind);
    }
}

#[derive(Component)]
pub struct Gauge {
    pub pos: Vec2,
    pub timer: Timer,
    pub wind_vec: Vec2,
    pub wind_seed: Vec2,
    pub perlin: PerlinNoise,
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
) {
    let color = Color::hsl(180., 0.95, 0.7);

    commands.spawn((
        Gauge{
            pos: Vec2::new(50.0, 50.0),
            timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
            wind_vec: Vec2::ZERO,
            wind_seed: Vec2::new(50.0, 50.0),
            perlin: PerlinNoise::new()
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

fn update_wind(
    mut gauge: Single<&mut Gauge>,
    mut text: Single<&mut Text>,
    time: Res<Time>,
) {
    gauge.timer.tick(time.delta());
    if gauge.timer.just_finished() {

        gauge.wind_seed.x += 0.001;
        gauge.wind_seed.y += 0.002;
        gauge.wind_vec.x = gauge.perlin.get(gauge.wind_seed.x as f64) as f32 - 0.5;
        gauge.wind_vec.y = gauge.perlin.get(gauge.wind_seed.y as f64) as f32 - 0.5;
        text.0 = format!("Wind Direction: {}", gauge.wind_vec);
    }
}

fn move_with_wind(
    mut query: Query<(&Gauge, &mut Transform, &mut Sprite)>,
    wind: Single<&Gauge>
) {
    if wind.timer.just_finished() {
        for (gauge, mut transform, mut sprite) in &mut query {
            let extended_vector = wind.wind_vec.extend(0.0) * MOVEMENT_FACTOR;
            let length = transform.translation.distance(transform.translation + extended_vector);
            let diff = transform.translation - (transform.translation + extended_vector);
            let theta = diff.y.atan2(diff.x);
            //let midpoint = (transform.translation + (transform.translation - extended_vector)) / 2.;

            transform.rotation = Quat::from_rotation_z(theta);
            transform.translation = gauge.pos.extend(0.0) + transform.local_x().as_vec3() * length/2.0;
            sprite.custom_size = Some(Vec2::new(length, 2.0));
        }
    }
}

