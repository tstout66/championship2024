mod gauge;
mod player_camera;
mod boat;
mod wind;

use bevy::prelude::*;
use gauge::GaugePlugin;
use boat::BoatPlugin;
use player_camera::PlayerCameraPlugin;
use wind::WindPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WindPlugin)
        .add_plugins(GaugePlugin)
        .add_plugins(PlayerCameraPlugin)
        .add_plugins(BoatPlugin)
        .run();
}


