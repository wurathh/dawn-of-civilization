pub mod fps;
pub mod map;
pub mod menu;
pub mod scenario;
pub mod state;
pub mod ui;

use bevy::prelude::*;
use bevy::window::PresentMode;
use fps::FpsPlugin;
use map::MapPlugin;
use menu::MenuPlugin;
use state::{AppState, GameSession, GameSettings, UiFonts};
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dawn of Civilization".to_string(),
                resolution: (1440, 850).into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.11, 0.17)))
        .init_resource::<UiFonts>()
        .init_resource::<GameSession>()
        .init_resource::<GameSettings>()
        .init_state::<AppState>()
        .add_plugins((FpsPlugin, MapPlugin, MenuPlugin, UiPlugin))
        .run();
}
