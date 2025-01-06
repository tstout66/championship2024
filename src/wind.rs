use std::time::Duration;
use bevy::prelude::*;
use perlin_noise::PerlinNoise;
use rand::Rng;

const WIND_PERLIN_SCALE: f32 = 10000.0;

pub struct WindPlugin;

impl Plugin for WindPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, update)
        ;
    }
}

#[derive(Resource)]
pub struct Wind {
    pub timer: Timer,
    pub perlin_x: PerlinNoise,
    pub perlin_y: PerlinNoise,
    pub offset: Vec2,
    pub scale: f32,
}

fn setup(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    commands.insert_resource(Wind {
        timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
        perlin_x: PerlinNoise::new(),
        perlin_y: PerlinNoise::new(),
        //offset: Vec2::ZERO,
        offset: Vec2::new(
            rng.gen_range::<f32>(0.0, 1.0),
            rng.gen_range::<f32>(0.0, 1.0),
        ),
        scale: WIND_PERLIN_SCALE,
    });
}

fn update(time: Res<Time>, mut wind: ResMut<Wind>) {
    wind.timer.tick(time.delta());
    if wind.timer.just_finished() {
        wind.offset += Vec2::new(0.01, 0.01);
    }
}
