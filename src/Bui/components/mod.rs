pub mod topbar;
pub use topbar::*;

// #=== ROUTE PLUGIN ===#
use bevy::prelude::*;

pub struct ComponentPlugin;
impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add each component plugin
            .add_plugins(P_Topbar);
    }
}
