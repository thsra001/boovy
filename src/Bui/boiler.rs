use std::default;

use bevy::prelude::*;
use bevy_lunex::prelude::*;
/// Custom color palette for Bevypunk
pub trait BoovyPalette {
    const DARK_GREEN: Color;
    const LIGHT_GREEN: Color;
    const TEXT_GREEN: Color;
    const BLACK: Color;
}
impl BoovyPalette for Color {
    const DARK_GREEN: Color = Color::oklab(0.3, -0.13, 0.03);
    const LIGHT_GREEN: Color = Color::oklab(0.38, -0.08, 0.04);
    const TEXT_GREEN: Color = Color::oklab(0.3, -0.13, 0.03);
    const BLACK: Color = Color::oklab(0.3, -0.13, 0.03);
    //backup
    // const DARK_GREEN: Color = Color::oklab(0.3, -0.13, 0.03);
    // const LIGHT_GREEN: Color = Color::oklab(0.4, -0.15, 0.02);
    // const TEXT_GREEN: Color = Color::oklab(0.3, -0.13, 0.03);
    // const BLACK: Color = Color::oklab(0.3, -0.13, 0.03);
}
#[derive(Bundle)]
// lunex ui bundle
pub struct LuiBundle<T: Component> {
    //pub path: UiLink,
    pub path: UiLink<T>,
    pub layout: UiLayout,
    pub name: Name,
}
