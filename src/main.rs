mod gauge;
mod player_camera;
mod environment;

use bevy::prelude::*;
use gauge::Gauge;
use crate::player_camera::PlayerCamera;
use crate::environment::Environment;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Gauge)
        .add_plugins(PlayerCamera)
        .add_plugins(Environment)
        .run();
}


