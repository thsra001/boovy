pub mod top_button;
pub use top_button::*;
pub mod colour_bg;
pub use colour_bg::*;
pub mod project_button;
pub use project_button::*;
pub mod topbar;
pub use topbar::*;
// #=== ROUTE PLUGIN ===#
use bevy::prelude::*;

pub struct ComponentPlugin;
impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add each component plugin
            .add_plugins(PTopButton)
            .add_plugins(PColourBg)
            .add_plugins(PProjectB)
            .add_plugins(PTopbar);
    }
}
