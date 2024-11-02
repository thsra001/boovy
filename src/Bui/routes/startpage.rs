/// When this component is added, a UI system is built
use bevy::prelude::*;
use bevy_lunex::prelude::*;

use crate::BoovyStates;
//use crate::Bui::project_button;
use crate::Bui::ColourBg;
use crate::Bui::LuiBundle;
use crate::Bui::ProjectB;
use crate::Bui::Topbar;

use crate::Bui::BoovyPalette;

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
    //assets: Res<AssetServer>,
    query: Query<Entity, Added<Startpage>>,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
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
                        // uitreebundle is for a top dog of da tree lolololol
                        UiTreeBundle::<MainUi>::from(UiTree::new2d("startpage")),
                        //MovableByCamera,
                        StateScoped(BoovyStates::Menu),
                        Name::new("startpageMarker"),
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
                                path: root.add("topbar"),
                                layout: UiLayout::window()
                                    .size((Rl(100.0), Em(2.5)))
                                    .pack::<Base>(),
                                name: Name::new("topbarOuter"),
                            },
                            Topbar,
                            UiDepthBias(30.0),
                        ));
                        ui.spawn((
                            LuiBundle {
                                path: root.add("Bg"),
                                layout: UiLayout::window()
                                    .size((Rl(100.0), Rl(100.0) + Em(2.5)))
                                    .y(Em(2.5))
                                    .pack::<Base>(),
                                name: Name::new("background"),
                            },
                            ColourBg {
                                col: Color::DARK_GREEN,
                            },
                        ));
                        ui.spawn((
                            LuiBundle {
                                path: root.add("Bg/icon"),
                                layout: UiLayout::window().size(Em((9.0, 11.0))).pack::<Base>(),
                                name: Name::new("projectButtonOuter"),
                            },
                            ProjectB {
                                project_name: String::from("Le Rectangle"),
                                project_image: asset_server.load("images/templates/rectangle.png"),
                            },
                        ));
                    });
            });
    }
}
