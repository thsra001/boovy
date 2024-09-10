use bevy::prelude::*;

pub struct PStateManager; // plugin export// it mark ui ( just treat it like MainUi in lunex docs)

impl Plugin for PStateManager {
    fn build(&self, app: &mut App) {
        app.add_systems()
    }
}

enum BoovyStates {
    Loading,
    Menu,
    Game,
}
