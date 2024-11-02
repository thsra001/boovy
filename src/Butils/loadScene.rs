use bevy::{prelude::*, tasks::IoTaskPool, utils::Duration};
use std::{fs::File, io::Write};

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
