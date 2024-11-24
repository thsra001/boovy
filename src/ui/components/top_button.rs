use bevy::prelude::*;
use bevy_lunex::{prelude::*, UiCorePlugin};

use crate::ui::{BoovyPalette, ColourBg, LuiBundle};
/// When this component is added, a UI system is built
#[derive(Component)]
pub struct TopButton {
    image: Option<Handle<Image>>,
    text: Option<String>,
}
// ui markerTopbar

// ui markerTopbar

#[derive(Component)]
// ui markerTopbar
struct TopButtonUi;

pub struct PTopButton;
impl Plugin for PTopButton {
    fn build(&self, app: &mut App) {
        app
            // Add Lunex plugins for our sandboxed UI
            .add_plugins(UiCorePlugin::<TopButtonUi>::new())
            // NOTE! Systems changing the UI need to run before UiSystems::Compute
            // or they will not get picked up by change detection.
            .add_systems(Update, build_top_button.before(UiSystems::Compute));
    }
}
/// System that builds the route when the component is added
fn build_top_button(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<Entity, Added<TopButton>>,
) {
    for entity in &query {
        commands
            .entity(entity)
            .insert((
                // Insert this bundle into the entity that just got the CustomButtom component
                // Note that CustomButtonUi is used here instead of MainUi
                UiTreeBundle::<TopButtonUi>::from(UiTree::new2d("TopButton")),
                // Now spawn the UI as children
            ))
            .with_children(|ui| {
                // Spawn some UI nodes
                ui.spawn((
                    // Note that CustomButtonUi is used here instead of MainUi
                    // TODO: uilink shits itself when its <TopButtonUi> // fix this so LuiBundle works later
                    UiLink::<TopButtonUi>::path("TopButton"),
                    UiLayout::window_full().pack::<Base>(),
                    Name::new("TopButtonInner"),
                    ColourBg {
                        col: Color::LIGHT_GREEN,
                    },
                ));
            });
    }
}
