/// When this component is added, a UI system is built
use bevy::prelude::*;
use bevy_lunex::prelude::*;
#[derive(Component)]
pub struct Startpage;

pub struct PStartpage;
impl Plugin for PStartpage {
    fn build(&self, app: &mut App) {
        app
            // NOTE! Systems changing the UI need to run before UiSystems::Compute
            // or they will not get picked up by change detection.
            .add_systems(Update, build_startpage.before(UiSystems::Compute));
    }
}

/// System that builds the route when the component is added
fn build_startpage(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<Entity, Added<Startpage>>,
    asset_server: Res<AssetServer>,
) {
    for route_entity in &query {
        // Make our route a spatial entity
        commands
            .entity(route_entity)
            .insert(SpatialBundle::default())
            .with_children(|route| {
                // Spawn some additional non UI components if you need to.

                // Here you can spawn the UI
                route
                    .spawn((
                        UiTreeBundle::<MainUi>::from(UiTree::new("startpage")),
                        MovableByCamera,
                    ))
                    .with_children(|ui| {
                        // Spawn some UI nodes
                        ui.spawn((
                            // Link the entity
                            UiLink::<MainUi>::path("Root/Bg"),
                            // Specify UI layout
                            UiLayout::solid()
                                .size(Ab((1920.0, 1080.0)))
                                .scaling(Scaling::Fill)
                                .pack::<Base>(),
                            // Add image to the entity
                            // UiImage2dBundle::from(asset_server.load("images/bg.png")),
                            Name::new("background"),
                        ));
                    });
            });
    }
}
