// bevy 13.2
use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_lunex::;
// components/custom_button.rs

/// When this component is added, a UI system is built
#[derive(Component)]
pub struct CustomButtom {
    // Any fields we want to interact with should be here.
    text: String,
}
/// Marker struct for the sandboxed UI
#[derive(Component)]
struct CustomButtonUi;

/// System that builds the route when the component is added
fn build_route(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<Entity, Added<CustomButtom>>,
) {
    for entity in &query {
        commands
            .entity(entity)
            .insert((
                // Insert this bundle into the entity that just got the CustomButtom component
                // Note that CustomButtonUi is used here instead of MainUi
                UiTreeBundle::<CustomButtonUi>::from(UiTree::new("CustomButton")),
                // Now spawn the UI as children
            ))
            .with_children(|ui| {
                // Spawn some UI nodes
                ui.spawn((
                    // Link this widget
                    // Note that CustomButtonUi is used here instead of MainUi
                    UiLink::<CustomButtonUi>::path("Image"),
                    // Add layout
                    UiLayout::window_full().pack(),
                    // Give it a background image
                    UiImage2dBundle {
                        texture: assets.load("images/button.png"),
                        sprite: Sprite {
                            color: Color::RED,
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
// components/custom_button.rs

pub struct CustomButtonPlugin;
impl Plugin for CustomButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Lunex plugins for our sandboxed UI
            .add_plugins(UiGenericPlugin::<CustomButtonUi>::new())
            // NOTE! Systems changing the UI need to run before UiSystems::Compute
            // or they will not get picked up by change detection.
            .add_systems(Update, build_route.before(UiSystems::Compute));
    }
}
