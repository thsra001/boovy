// When this component is added, a UI system is built
use bevy::prelude::*;
use bevy_lunex::prelude::*;

use crate::BoovyStates;
use crate::Bui::BoovyPalette;
use crate::Bui::ColourBg;
use crate::Bui::LuiBundle;

//load egui deps
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext; //EguiPlugin};
use bevy_inspector_egui::bevy_inspector;
//use bevy_inspector_egui::prelude::*;

#[derive(Component)]
pub struct ScenePath(pub String);
#[derive(Component)]
pub struct Editor;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EditorSet;
pub struct PEditor;
impl Plugin for PEditor {
    fn build(&self, app: &mut App) {
        app
            // make set for editor ui
            .configure_sets(Update, EditorSet.run_if(in_state(BoovyStates::Editor)))
            // NOTE! Systems changing the UI need to run before UiSystems::Compute
            // or they will not get picked up by change detection.
            .add_systems(Update, build_editor)
            .add_systems(Update, hiearchy_ui.in_set(EditorSet));
    }
}
fn hiearchy_ui(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    egui::Window::new("Hiearchy").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector::ui_for_world(world, ui);

            // works with any `Reflect` value, including `Handle`s
            let mut any_reflect_value: i32 = 5;
            bevy_inspector::ui_for_value(&mut any_reflect_value, ui, world);

            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

            ui.heading("Entities");
            bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}

/// System that builds the route when the component is added
fn build_editor(
    mut commands: Commands,
    //assets: Res<AssetServer>,
    query: Query<Entity, Added<Editor>>,
    // asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
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
                next_state.set(BoovyStates::Editor);
                info!("we goin to editor");
                // Here you can spawn the UI
                route
                    .spawn((
                        // uitreebundle is for a top dog of da tree lolololol
                        UiTreeBundle::<MainUi>::from(UiTree::new2d("Editor")),
                        //MovableByCamera,
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
