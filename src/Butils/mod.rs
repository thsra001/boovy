pub mod stateManager;
pub use stateManager::*;
pub mod editor;
pub use editor::*;

// #=== ROUTE PLUGIN ===#
use bevy::prelude::*;

pub struct RoutePlugin;
impl Plugin for RoutePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add each route plugin
            .add_plugins(PstateManager)
            .add_plugins(PEditor);
    }
}
