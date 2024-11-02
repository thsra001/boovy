use bevy::prelude::*;
use bevy_lunex::{prelude::*, UiCorePlugin};

use crate::Bui::{BoovyPalette, ColourBg, Editor, LuiBundle, ScenePath};
/// When this component is added, a UI system is built
#[derive(Component)]
pub struct ProjectB {
    //gameIcon:
    pub project_name: String,
    pub project_image: Handle<Image>,
}
#[derive(Component)]
// ui marker
struct ProjectBUi;

#[derive(Component)]
struct ButImg;
pub struct PProjectB;
impl Plugin for PProjectB {
    fn build(&self, app: &mut App) {
        app
            // Add Lunex plugins for our sandboxed UI
            .add_plugins(UiCorePlugin::<ProjectBUi>::new())
            // NOTE! Systems changing the UI need to run before UiSystems::Compute
            // or they will not get picked up by change detection.
            .add_systems(Update, build_project_button.before(UiSystems::Compute));
    }
}
/// System that builds the route when the component is added
fn build_project_button(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<(Entity, &ProjectB), Added<ProjectB>>,
) {
    for (entity, tings) in &query {
        commands
            .entity(entity)
            .insert((
                // Insert this bundle into the entity that just got the CustomButtom component
                // Note that CustomButtonUi is used here instead of MainUi
                UiTreeBundle::<ProjectBUi>::from(UiTree::new2d("ProjectB")),
                // Now spawn the UI as children
            ))
            .with_children(|ui| {
                ui.spawn((
                    LuiBundle {
                        path: UiLink::<ProjectBUi>::path("ProjectB"),
                        layout: UiLayout::window_full().pack::<Base>(),
                        name: Name::new("ProjectBInner"),
                    },
                    ColourBg {
                        col: Color::LIGHT_GREEN,
                    },
                    Pickable::IGNORE,
                ));
                ui.spawn((
                    LuiBundle {
                        path: UiLink::<ProjectBUi>::path("ProjectB/Image"),
                        layout: UiLayout::window().size(Em(8.0)).pos(Em(0.5)).pack::<Base>(),
                        name: Name::new("ProjectImage"),
                    },
                    UiImage2dBundle {
                        texture: tings.project_image.clone(),
                        ..default()
                    },
                    UiClickEmitter::SELF,
                    //OnUiClickDespawn::new(entity),
                    OnUiClickCommands::new(|commands| {
                        commands.spawn((Editor, ScenePath(String::from("bob"))));
                    }),
                    ButImg,
                ));
                ui.spawn((
                    (LuiBundle {
                        path: UiLink::<ProjectBUi>::path("ProjectB/Label"),
                        layout: UiLayout::window()
                            .size(Em((8.0, 1.5)))
                            .pos(Em((0.5, 9.0)))
                            .pack::<Base>(),
                        name: Name::new("ProjectLabel"),
                    },),
                    UiText2dBundle {
                        text: Text::from_section(
                            &tings.project_name,
                            TextStyle {
                                font: assets.load("fonts/med.ttf"),
                                font_size: 60.0, // By default hardcoded as Relative height (Rh) - so 60% of the node height
                                color: Color::TEXT_GREEN.with_luminance(0.5),
                            },
                        ),
                        ..default()
                    },
                    UiTextSize::new().size(Em(1.5)),
                ));
            });
    }
}
// TODO: this shit is fucked, solve later or something
fn project_button_click_sys(mut events: EventReader<UiClickEvent>, query: Query<&ButImg>) {
    // Iterate over all events
    for event in events.read() {
        // Get our entity
        if let Ok(button) = query.get(event.target) {
            // Process our button click
            info!("skbidi my friend");
        }
        info!(" pressed {}", event.target);
        info!("sikbi");
    }
}
