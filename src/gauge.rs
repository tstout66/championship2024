use std::time::Duration;
use bevy::prelude::*;
use perlin_noise::PerlinNoise;
use rand::Rng;

const MOVEMENT_FACTOR: f32 = 200.0;

pub struct GaugePlugin;

impl Plugin for GaugePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_gauge, spawn_timer))
        .add_systems(Startup, text_ui)
        .add_systems(Update, update_wind)
        .add_systems(Update, move_with_wind);
    }
}

#[derive(Resource)]
pub struct Wind {
    pub timer: Timer,
    pub perlin_x: PerlinNoise,
    pub perlin_y: PerlinNoise,
    pub offset: Vec2,
}

#[derive(Component)]
pub struct Gauge {
    pub pos: Vec2,
    pub wind_vec: Vec2,
    //pub wind_seed: Vec2,
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

fn spawn_timer(
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    commands.insert_resource(
        Wind {
            timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
            perlin_x: PerlinNoise::new(),
            perlin_y: PerlinNoise::new(),
            //offset: Vec2::ZERO,
            offset: Vec2::new(rng.gen_range::<f32>(0.0,1.0), rng.gen_range::<f32>(0.0,1.0)),
        }
    );
}

fn spawn_gauge(
    mut commands: Commands,
) {
    let color = Color::hsl(180., 0.95, 0.7);

    for y in -10..10 {
        for x in -10..10 {
            commands.spawn((
                Gauge {
                    pos: Vec2::new(50.0 * x as f32, 50.0 * y as f32),
                    wind_vec: Vec2::ZERO,
                    //wind_seed: Vec2::new(50.0 * i as f32, 50.0),

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
                Transform::from_xyz(50. * x as f32, 50. * y as f32, 50.),
            ));
        }
    }
}

fn update_wind(
    mut gauge: Query<(&mut Gauge)>,
    mut text: Single<&mut Text>,
    mut wind: ResMut<Wind>,
    time: Res<Time>,
) {
    wind.timer.tick(time.delta());
    if wind.timer.just_finished() {
        wind.offset += Vec2::new(0.001, 0.001);
        for (mut gauge) in gauge.iter_mut() {
            let projected_x_pos = (gauge.pos.x/1000.0) as f64;
            let projected_y_pos = (gauge.pos.y/1000.0) as f64;

            gauge.wind_vec.x = wind.perlin_x.get2d([projected_x_pos + wind.offset.x as f64, projected_y_pos + wind.offset.y as f64]) as f32 - 0.5;
            gauge.wind_vec.y = wind.perlin_y.get2d([projected_x_pos + wind.offset.x as f64, projected_y_pos + wind.offset.y as f64]) as f32 - 0.5;
            text.0 = format!("Wind Direction: {}", gauge.wind_vec);
        }
    }
}

fn move_with_wind(
    mut query: Query<(&Gauge, &mut Transform, &mut Sprite)>,
    wind_timer: ResMut<Wind>,
) {
    if wind_timer.timer.just_finished() {
        for (gauge, mut transform, mut sprite) in &mut query {
            let extended_vector = gauge.wind_vec.extend(0.0) * MOVEMENT_FACTOR;
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

