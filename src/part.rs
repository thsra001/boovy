use bevy::{prelude::*, transform::commands};
//use bevy_rapier3d::prelude::*;
use std::fs;

pub struct PartUtils;

impl Plugin for PartUtils {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, get_mats);
    }
}

// loops trough mats folder and gets mats

#[derive(Component)]
struct MatNormal(Handle<Image>);
#[derive(Component)]
struct MatColour(Handle<Image>);
#[derive(Component)]
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
            .insert(MatMetalRough(asset_server.load(pat.clone() + "/metalRough.png")))
            .insert(MatNormal(asset_server.load(pat.clone() + "/normal.png")))
            .id();
        commands.entity(mat_holder).add_child(chill);
    }
}
fn make_part(mut commands:Commands){
    let mut part=1;
}