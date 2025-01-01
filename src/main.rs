mod gauge;
mod player_camera;

use bevy::prelude::*;
use gauge::GaugePlugin;
use crate::player_camera::PlayerCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GaugePlugin)
        .add_plugins(PlayerCamera)
        .run();
}


