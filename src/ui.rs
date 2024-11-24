// bevy 13.2
use bevy::{prelude::*, render::{camera::CameraRenderGraph, view::{RenderLayers, VisibleEntities}}, sprite::Anchor};
use bevy_lunex::prelude::*;

mod boiler;
use boiler::*;
mod components;
pub(crate) mod routes;
use components::*;
use routes::*;

use crate::BoovyStates;

pub struct CreatorUi; // plugin export// it mark ui ( just treat it like MainUi in lunex docs)

impl Plugin for CreatorUi {
    fn build(&self, app: &mut App) {
        app
            // twp local plugins
            .add_plugins(ComponentPlugin)  
            .add_plugins(RoutePlugin)       
            .add_plugins((UiDefaultPlugins,UiDebugPlugin::<MainUi>::new()))
            .add_systems(Startup, make_creator_start_ui)
            .insert_resource(ClearColor(Color::oklab(0.2, 0.070, -0.240)))
            .init_state::<BoovyStates>()
            // bevy lunex

            ;
    }
}
pub const  ui_layer: RenderLayers = RenderLayers::layer(16);
fn make_creator_start_ui(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut event1: EventWriter<actions::SetWindowDecorations>,
    mut event2: EventWriter<actions::SetWindowResolution>) {
    commands.spawn((
        // Add this marker component provided by Lunex.
        MainUi,
        // Our camera bundle with depth 1000.0 because UI starts at `0` and goes up with each layer.
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            camera: Camera{order:5, ..default()},

            ..default()
        },
        ui_layer,
        InheritedVisibility::VISIBLE,
        Name::new("cam2d"),
    )).with_children(|camera|{
        camera.spawn((StyledCursorBundle{
            cursor:Cursor2d::new(),
            atlas: TextureAtlas{
                layout: atlas_layout.add(TextureAtlasLayout::from_grid(UVec2::splat(48), 13, 1, None, None)),
                index: 0,
            },
            sprite: SpriteBundle{
                texture: asset_server.load("images/CurAtlas.png"),
                transform: Transform { scale: Vec3::new(1.0, 1.0, 1.0), ..default() },
                sprite: Sprite {
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },
            ..default()
        },ui_layer));
    });
    // laod cursor2d
    // ui item example > this is also the  mainui
    commands
        .spawn((
            // This makes the UI entity able to receive camera data
            SourceFromCamera,
            // This is our UI system
            UiTreeBundle::<MainUi>::from(UiTree::new2d("Root")),
            Name::new("root"),
            ui_layer
        ));
    //event1.send(actions::SetWindowDecorations(false));
    event2.send(actions::SetWindowResolution(Vec2::new(1920.0, 1080.0)));
    commands.spawn((Startpage,Name::new("startpage"),ui_layer));
}

