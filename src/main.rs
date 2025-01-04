mod gauge;
mod player_camera;
mod boat;

use bevy::prelude::*;
use gauge::GaugePlugin;
use crate::boat::BoatPlugin;
use crate::player_camera::PlayerCameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GaugePlugin)
        .add_plugins(PlayerCameraPlugin)
        .add_plugins(BoatPlugin)
        .run();
}


