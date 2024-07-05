// bevy 13.2
use bevy::prelude::*;
use bevy_lunex::prelude::*;

use crate::Bui::BoovyCreatorUiMarker;

/// When this component is added, a UI system is built
#[derive(Component)]
pub struct MyRoute;

/// System that builds the route when the component is added
fn build_route(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<Entity, Added<MyRoute>>,
) {
    for entity in &query {
        commands
            .entity(entity)
            .insert((
                // Insert this bundle into the entity that just got the MyRoute component
                UiTreeBundle::<BoovyCreatorUiMarker>::from(UiTree::new("MyRoute")),
                // Now spawn the UI as children
            ))
            .with_children(|ui| {
                // Spawn some UI nodes
                ui.spawn((
                    UiLink::<BoovyCreatorUiMarker>::path("Background"),
                    UiLayout::solid()
                        .size((1920.0, 1080.0))
                        .scaling(Scaling::Fill)
                        .pack(),
                    UiImage2dBundle::from(assets.load("images/background.png")),
                ));
            });
    }
}

pub struct MyRoutePlugin;
impl Plugin for MyRoutePlugin {
    fn build(&self, app: &mut App) {
        app
            // NOTE! Systems changing the UI need to run before UiSystems::Compute
            // or they will not get picked up by change detection.
            .add_systems(Update, build_route.before(UiSystems::Compute));
    }
}
