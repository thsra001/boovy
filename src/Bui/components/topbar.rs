use bevy::prelude::*;
use bevy_lunex::prelude::*;
/// When this component is added, a UI system is built
#[derive(Component)]
pub struct Topbar {
    // Any fields we want to interact with should be here.
    text: String,
}
#[derive(Component)]
struct TopbarUi;

pub struct P_Topbar;
impl Plugin for P_Topbar {
    fn build(&self, app: &mut App) {
        app
            // Add Lunex plugins for our sandboxed UI
            .add_plugins(UiGenericPlugin::<TopbarUi>::new())
            // NOTE! Systems changing the UI need to run before UiSystems::Compute
            // or they will not get picked up by change detection.
            .add_systems(Update, build_route.before(UiSystems::Compute));
    }
}

/// System that builds the route when the component is added
fn build_route(
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
                UiTreeBundle::<TopbarUi>::from(UiTree::new("CustomButton")),
                // Now spawn the UI as children
            ))
            .with_children(|ui| {
                // Spawn some UI nodes
                ui.spawn((
                    // Link this widget
                    // Note that CustomButtonUi is used here instead of MainUi
                    UiLink::<TopbarUi>::path("Image"),
                    // Add layout
                    UiLayout::window_full().pack::<Base>(),
                    // Give it a background image
                    UiImage2dBundle {
                        texture: assets.load("images/button.png"),
                        sprite: Sprite {
                            color: Color::oklab(0.3, 0.1, 0.7),
                            ..default()
                        },
                        ..default()
                    },
                    // Give the texture 9-slice tilling
                    ImageScaleMode::Sliced(TextureSlicer {
                        border: BorderRect::square(32.0),
                        ..default()
                    }),
                ));
            });
    }
}
