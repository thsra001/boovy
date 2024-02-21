use bevy::{prelude::*, transform::commands};
//use bevy_rapier3d::prelude::*;
use std::fs;

pub struct PartUtils;

impl Plugin for PartUtils {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, get_mats);
        app.add_systems(PostStartup, part_functions_holder);
        app.register_type::<MaterialType>();
        app.register_type::<MatColour>();
        app.register_type::<MatMetalRough>();
        app.register_type::<MatNormal>();
    }
}

// loops trough mats folder and gets mats
#[derive(Component, Reflect)]
enum MaterialType {
    Grass,
    Sand,
    Wood,
    WoodPlank,
}
#[derive(Component, Reflect)]
struct MatNormal(Handle<Image>);
#[derive(Component, Reflect)]
struct MatColour(Handle<Image>);
#[derive(Component, Reflect)]
struct MatMetalRough(Handle<Image>);
//  mcolour: asset_server.load(pat.clone() + "/color.png")
fn get_mats(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("yup");
    let mat_holder = commands.spawn(Name::new("materialManager")).id();
    let paths = fs::read_dir("./assets/mats").unwrap();

    for path in paths {
        let fold = path.unwrap().file_name().into_string().unwrap();
        let pat = String::from("mats/") + &fold;

        println!("yup yup: {}", &pat);
        let chill = commands
            .spawn(Name::new(fold))
            .insert(MatColour(asset_server.load(pat.clone() + "/color.png")))
            .insert(MatMetalRough(
                asset_server.load(pat.clone() + "/metalRough.png"),
            ))
            .insert(MatNormal(asset_server.load(pat.clone() + "/normal.png")))
            .id();
        commands.entity(mat_holder).add_child(chill);
    }
}
fn part_functions_holder(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut create_object = |size: f32,
                             color: Color,
                             name: String,
                             xyz: (f32, f32, f32)|
     -> (PbrBundle, Name, MaterialType) {
        (
            PbrBundle {
                mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
                material: materials.add(Color::rgb(1.0, 0.1, 0.0)),
                transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2),
                ..default()
            },
            Name::new(name),
            MaterialType::Grass,
        )
    };
    commands.spawn(create_object(
        3.0,
        Color::RED,
        "Red Cube".to_string(),
        (-4.5, 1.5, -4.5),
    ));
}
