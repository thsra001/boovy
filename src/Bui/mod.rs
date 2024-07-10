// bevy 13.2
use bevy::prelude::*;
use bevy_lunex::prelude::*;

mod components;
mod routes;
use components::*;
use routes::*;

pub struct CreatorUi; // plugin export// it mark ui ( just treat it like MainUi in lunex docs)

impl Plugin for CreatorUi {
    fn build(&self, app: &mut App) {
        app
            // twp local plugins
            .add_plugins(ComponentPlugin)  
            .add_plugins(RoutePlugin)       
            .add_plugins((UiPlugin,UiDebugPlugin::<MainUi>::new()))
            .add_systems(Startup, make_creator_start_ui)
            .insert_resource(ClearColor(Color::oklab(0.2, 0.070, -0.240)))
            // bevy lunex

            ;
    }
}

fn make_creator_start_ui(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    commands.spawn((
        // Add this marker component provided by Lunex.
        MainUi,
        // Our camera bundle with depth 1000.0 because UI starts at `0` and goes up with each layer.
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            ..default()
        },
        Name::new("mainUi")
    ));

    commands.spawn((Startpage,Name::new("startpage")));
    // ui item example > this is also the  mainui
    commands
        .spawn((
            // This makes the UI entity able to receive camera data
            MovableByCamera,
            // This is our UI system
            UiTreeBundle::<MainUi>::from(UiTree::new("Root")),
            Name::new("root")
        ));
}
