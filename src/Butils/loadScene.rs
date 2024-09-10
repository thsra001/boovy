use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_common_assets::ron::RonAssetPlugin;

pub struct LoadScene; // plugin export// it mark ui ( just treat it like MainUi in lunex docs)

impl Plugin for LoadScene {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<Scene>::new(&[".roony"]));
    }
}

fn loadWorld() {}

fn loadWorld2(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((Name::new("Scene")))
}
