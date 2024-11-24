pub mod saveScene;
pub use saveScene::*;

// #=== ROUTE PLUGIN ===#
use bevy::prelude::*;

pub struct LoadButils;
impl Plugin for LoadButils {
    fn build(&self, app: &mut App) {
        app
            // Add each route plugin
            .add_plugins(PSaveScene);
    }
}
