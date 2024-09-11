/// When this component is added, a UI system is built
use bevy::prelude::*;
use bevy_lunex::prelude::*;

use crate::BoovyStates;
use crate::Bui::BoovyPalette;
use crate::Bui::ColourBg;
use crate::Bui::LuiBundle;
#[derive(Component)]
pub struct Editor;

pub struct PEditor;
impl Plugin for PEditor {
    fn build(&self, app: &mut App) {
        app
            // NOTE! Systems changing the UI need to run before UiSystems::Compute
            // or they will not get picked up by change detection.
            .add_systems(Update, build_editor.before(UiSystems::Compute));
    }
}

/// System that builds the route when the component is added
fn build_editor(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<Entity, Added<Editor>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut next_state: ResMut<NextState<BoovyStates>>,
    mut cam2d: Query<&mut Camera, With<Camera2d>>,
) {
    for route_entity in &query {
        // Make our route a spatial entity
        let mut camra = cam2d.get_single_mut().unwrap();
        camra.is_active = false;
        commands
            .entity(route_entity)
            .insert(SpatialBundle::default())
            .with_children(|route| {
                // Spawn some additional non UI components if you need to.
                next_state.set(BoovyStates::Game);
                info!("we goin to editor");
                // Here you can spawn the UI
                route
                    .spawn((
                        // uitreebundle is for a top dog of da tree lolololol
                        UiTreeBundle::<MainUi>::from(UiTree::new2d("Editor")),
                        MovableByCamera,
                        Name::new("EditorMarker"),
                    ))
                    .with_children(|ui| {
                        // Spawn some UI nodes
                        let root = UiLink::<MainUi>::path("Root");
                        ui.spawn((
                            // Link the entity
                            LuiBundle {
                                path: root.clone(),
                                layout: UiLayout::window_full().pack::<Base>(),
                                name: Name::new("root"),
                            },
                        ));
                        ui.spawn((
                            LuiBundle {
                                path: root.add("Bg"),
                                layout: UiLayout::window_full().pack::<Base>(),
                                name: Name::new("TreeViewer"),
                            },
                            ColourBg {
                                col: Color::DARK_GREEN,
                            },
                        ));
                    });
            });
    }
}
