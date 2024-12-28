mod gauge;
mod player_camera;
mod environment;

use bevy::prelude::*;
use gauge::GaugePlugin;
use crate::player_camera::PlayerCamera;
use crate::environment::Environment;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GaugePlugin)
        .add_plugins(PlayerCamera)
        .add_plugins(Environment)
        .run();
}


