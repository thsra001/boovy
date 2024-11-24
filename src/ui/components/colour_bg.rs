use crate::ui::BoovyPalette;
use bevy::prelude::*;
use bevy_lunex::prelude::*;
/// When this component is added, a UI system is built
#[derive(Component)]
pub struct ColourBg {
    pub col: Color,
}

pub struct PColourBg;
impl Plugin for PColourBg {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, build_colour_bg.before(UiSystems::Compute));
    }
}
/// System that builds the route when the component is added
fn build_colour_bg(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<(Entity, &ColourBg), Added<ColourBg>>,
) {
    for (ent, Col) in &query {
        commands.entity(ent).insert((
            UiColor::<Base>::new(Col.col),
            UiColor::<Hover>::new(Col.col),
            UiAnimator::<Hover>::new(),
            UiImage2dBundle {
                texture: assets.load("images/1x1.png"),
                ..default()
            },
        ));
    }
}
