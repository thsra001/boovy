pub mod startpage;
pub use startpage::*;

// #=== ROUTE PLUGIN ===#
use bevy::prelude::*;

pub struct RoutePlugin;
impl Plugin for RoutePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add each route plugin
            .add_plugins(PStartpage);
    }
}
