use bevy::prelude::*;
use bevy::text::FontSource;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    ScenarioCatalog,
    ScenarioSelect,
    Settings,
    InGame,
}

#[derive(Resource, Debug, Clone)]
pub struct PreviousMenuState(pub AppState);

impl Default for PreviousMenuState {
    fn default() -> Self {
        Self(AppState::MainMenu)
    }
}

#[derive(Resource, Debug, Clone)]
pub struct GameSettings {
    pub fog_of_war: bool,
    pub difficulty: String,
    pub ai_aggressiveness: String,
    pub ironman: bool,
    pub show_modal: bool,
    pub fullscreen: bool,
    pub music: bool,
    pub volume: f32,
    pub tab: usize,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            fog_of_war: false,
            difficulty: "Normal".to_string(),
            ai_aggressiveness: "Standart".to_string(),
            ironman: false,
            show_modal: false,
            fullscreen: false,
            music: true,
            volume: 0.7,
            tab: 0,
        }
    }
}

#[derive(Resource, Clone)]
pub struct UiFonts {
    /// EU4 tarzı ana arayüz ve başlık fontu (Marcellus Regular)
    pub display: FontSource,
    /// Butonlar ve alt başlıklar için (Marcellus Regular)
    pub heading: FontSource,
    /// Genel arayüz metinleri için (Marcellus Regular)
    pub body: FontSource,
    /// İkincil detay metinleri için (Marcellus Regular)
    pub medium: FontSource,
    /// Harita üzerindeki ülke ve imparatorluk isimleri için klasik monumental font
    pub map_country: FontSource,
}

impl FromWorld for UiFonts {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let marcellus: Handle<Font> = asset_server.load("fonts/Marcellus-Regular.ttf");
        Self {
            display: FontSource::Handle(marcellus.clone()),
            heading: FontSource::Handle(marcellus.clone()),
            body: FontSource::Handle(marcellus.clone()),
            medium: FontSource::Handle(marcellus),
            map_country: FontSource::Handle(asset_server.load("fonts/EBGaramond-ExtraBold.ttf")),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct GameSession {
    pub scenario_id: String,
    pub player_civ_id: u32,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub turn: u32,
    pub speed: usize,
    pub country_panel_open: bool,
    pub event_open: bool,
}

impl Default for GameSession {
    fn default() -> Self {
        Self {
            scenario_id: "modernworld".to_string(),
            player_civ_id: 1, // Varsayılan: Türkiye
            year: 2026,
            month: 10,
            day: 29,
            turn: 1,
            speed: 3,
            country_panel_open: false,
            event_open: false,
        }
    }
}
