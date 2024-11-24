use bevy::diagnostic::Diagnostic;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
// When this component is added, a UI system is built
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_lunex::prelude::*;
use egui_dock::dock_state;
use egui_dock::egui::Pos2;

use crate::ui::BoovyPalette;
use crate::ui::ColourBg;
use crate::ui::LuiBundle;
use crate::BoovyStates;

//load egui deps
use bevy_inspector_egui::bevy_egui::EguiContext; //EguiPlugin};
use bevy_inspector_egui::bevy_inspector;
use bevy_inspector_egui::egui;
//use bevy_inspector_egui::prelude::*;

// import part
use bevy_inspector_egui::bevy_inspector::hierarchy::{hierarchy_ui, SelectedEntities};
use bevy_inspector_egui::bevy_inspector::{
    ui_for_entities_shared_components, ui_for_entity_with_children,
};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use std::any::TypeId;
// use bevy_mod_picking::backends::egui::EguiPointer;
// use bevy_mod_picking::prelude::*;
use bevy::reflect::TypeRegistry;
use bevy::render::camera::{CameraProjection, Viewport};
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::EguiSet;
use egui_dock::{DockArea, DockState, NodeIndex, Style};

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
            .add_systems(Update, hiearchy_ui.in_set(EditorSet))
            .add_systems(Update, components_ui.in_set(EditorSet))
            .add_systems(Update, fps_ui.in_set(EditorSet))
            //.add_systems(Update, fps_ui.in_set(EditorSet))
            .add_plugins(FrameTimeDiagnosticsPlugin);
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
            //bevy_inspector::ui_for_world(world, ui);
            bevy_inspector::hierarchy::hierarchy_ui(world, ui, &mut SelectedEntities::default());

            // works with any `Reflect` value, including `Handle`s
            // let mut any_reflect_value: i32 = 5;
            // bevy_inspector::ui_for_value(&mut any_reflect_value, ui, world);

            // egui::CollapsingHeader::new("Materials").show(ui, |ui| {
            //     bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            // });

            // ui.heading("Entities");
            // bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}
fn components_ui(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    egui::Window::new("Components").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            bevy_inspector::hierarchy::hierarchy_ui(world, ui, &mut SelectedEntities::default());
        });
    });
}

fn fps_ui(mut egu: EguiContexts) {
    let mut dock_state = DockState::new(vec!["tab1".to_string()]);
    egui::Window::new("fps").show(egu.ctx_mut(), |ui| {
        ui.label("za wardu");
        // Create a new window `Surface` with one tab inside it.
        let mut surface_index = dock_state.add_window(vec!["Window Tab".to_string()]);

        // Access the window state by its surface index and then move and resize it.
        let window_state = dock_state.get_window_state_mut(surface_index).unwrap();
        window_state.set_position(Pos2::ZERO);
        window_state.set_size(bevy_inspector_egui::egui::Vec2::splat(100.0));
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
        //camra.is_active = false;
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
