mod asset_loader;
mod camera;
mod movement;
mod spaceship;

use bevy::prelude::*;

use crate::asset_loader::AssetLoaderPlugin;
use crate::camera::CameraPlugin;
use crate::movement::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .run();
}
