// bevy 13.2
use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_mod_picking::prelude::*;
mod components;
mod routes;
use components::*;
use routes::*;

pub struct BoovyCreatorUi;
#[derive(Component)]
pub struct BoovyCreatorUiMarker;

impl Plugin for BoovyCreatorUi {
    fn build(&self, app: &mut App) {
        app
            // twp local plugins
            .add_plugins(ComponentPlugin)
            .add_plugins(RoutePlugin)
            .add_systems(Startup, make_creator_start_ui)
            .insert_resource(ClearColor(Color::hex("882211").unwrap()))
            // bevy lunex
            .add_plugins(UiGeneralPlugin)
            .add_plugins(UiPlugin::<BoovyCreatorUiMarker>::new())
            .add_plugins(UiDebugPlugin::<BoovyCreatorUiMarker>::new())
            .add_plugins(DefaultPickingPlugins);
    }
}
fn make_creator_start_ui(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    commands.spawn((
        // Add this marker
        BoovyCreatorUiMarker,
        // Our camera bundle with depth 1000.0 because UI starts at `0` and goes up with each layer.
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            ..default()
        },
    ));

    commands
        .spawn((
            // This makes the UI entity able to receive camera data
            MovableByCamera,
            // This is our UI system
            UiTreeBundle::<BoovyCreatorUiMarker>::from(UiTree::new("Boovy creator ui")),
            Name::new("boovy creator ui"),
        ))
        .with_children(|ui| {
            // Here we will spawn our UI as children
            ui.spawn((
                // Link the entity
                UiLink::<BoovyCreatorUiMarker>::path("Root"),
                // Specify UI layout
                UiLayout::window_full()
                    .pos(Em((0.0, 1.5)))
                    .size((Rl(100.0), Rl(100.0) - Em(3.0)))
                    .pack(),
            ));

            ui.spawn((
                // Link the entity
                UiLink::<BoovyCreatorUiMarker>::path("Root/Rectangle"),
                // Specify UI layout
                UiLayout::solid()
                    .size(Ab((1920.0, 1080.0)))
                    .scaling(Scaling::Fill)
                    .pack(),
                // Add image to the entity
                UiImage2dBundle::from(asset_server.load("images/bg.png")),
            ));
        });
}
