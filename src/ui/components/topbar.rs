use bevy::prelude::*;
use bevy_lunex::{prelude::*, UiCorePlugin};

use crate::ui::{BoovyPalette, ColourBg, LuiBundle};
/// When this component is added, a UI system is built
#[derive(Component)]
pub struct Topbar;
#[derive(Component)]
// ui marker
struct TopbarUi;

pub struct PTopbar;
impl Plugin for PTopbar {
    fn build(&self, app: &mut App) {
        app
            // Add Lunex plugins for our sandboxed UI
            .add_plugins(UiCorePlugin::<TopbarUi>::new())
            // NOTE! Systems changing the UI need to run before UiSystems::Compute
            // or they will not get picked up by change detection.
            .add_systems(Update, build_topbar.before(UiSystems::Compute));
    }
}
/// System that builds the route when the component is added
fn build_topbar(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<Entity, Added<Topbar>>,
) {
    for entity in &query {
        commands
            .entity(entity)
            .insert((
                // Insert this bundle into the entity that just got the CustomButtom component
                // Note that CustomButtonUi is used here instead of MainUi
                UiTreeBundle::<TopbarUi>::from(UiTree::new2d("Topbar")),
                // Now spawn the UI as children
            ))
            .with_children(|ui| {
                // Spawn some UI nodes
                ui.spawn((
                    LuiBundle {
                        path: UiLink::<TopbarUi>::path("Topbar"),
                        layout: UiLayout::window_full().pack::<Base>(),
                        name: Name::new("TopbarInner"),
                    },
                    ColourBg {
                        col: Color::LIGHT_GREEN,
                    },
                ));
                ui.spawn(LuiBundle {
                    path: UiLink::<TopbarUi>::path("Topbar/actionbar"),
                    layout: UiLayout::window().size(Em((6.0, 2.5))).pack::<Base>(),
                    name: Name::new("actionbar"),
                });
                ui.spawn(LuiBundle {
                    path: UiLink::<TopbarUi>::path("Topbar/navbar"),
                    layout: UiLayout::window().size(Em((12.0, 2.5))).pack::<Base>(),
                    name: Name::new("navbar"),
                });
            });
    }
}
