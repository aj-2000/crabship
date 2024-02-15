mod asset_loader;

use bevy::prelude::*;

use crate::asset_loader::AssetLoaderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .run();
}
