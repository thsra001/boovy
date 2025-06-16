use bevy::{prelude::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};

use crate::{prop::PropType, BoovyStates};

pub struct PSaveScene; // plugin export// it mark ui ( just treat it like MainUi in lunex docs)

impl Plugin for PSaveScene {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(BoovyStates::EditorLoad), save_scene);
    }
}

const NEW_SCENE_FILE_PATH: &str = "scenes/save.roony";
fn save_scene(world: &mut World) {
    let mut scene_world = World::new();
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    scene_world.insert_resource(type_registry);

    //add items
    scene_world.spawn(Name::new("Scene")).with_children(|scen| {
        scen.spawn(PropType::BasicProp);
    });

    // With our sample world ready to go, we can now create our scene using DynamicScene or DynamicSceneBuilder.
    // For simplicity, we will create our scene using DynamicScene:
    let scene = DynamicScene::from_world(&scene_world);

    // Scenes can be serialized like this:
    let type_registry = world.resource::<AppTypeRegistry>();
    let type_registry = type_registry.read();
    let serialized_scene = scene.serialize(&type_registry).unwrap();

    //copy?

    // Showing the scene in the console
    info!("{}", serialized_scene);

    // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
    // as they are blocking
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}
