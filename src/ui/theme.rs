use bevy::prelude::*;

/// 1x asset'lerin 9-slice kenarları (slices.json -> nine_slice_border_px_1x)
pub const PANEL_BORDER: f32 = 22.0;
pub const PLAIN_BORDER: f32 = 14.0;
pub const BUTTON_BORDER: f32 = 12.0;
pub const BIG_BUTTON_BORDER: f32 = 20.0;
pub const HEADER_BORDER: f32 = 14.0;
pub const TAB_BORDER: f32 = 10.0;
pub const INSET_BORDER: f32 = 8.0;
pub const BAR_BORDER: f32 = 6.0;
pub const TIP_BORDER: f32 = 10.0;

/// Başlıklı normal panel: başlık bandı yüksekliği ve üst dilim (slices.json -> headed_*)
pub const HEAD_H: f32 = 40.0;
pub const HEAD_TOP_SLICE: f32 = 44.0;
pub const HEAD_SIDE: f32 = 22.0;
pub const HEAD_BOTTOM: f32 = 22.0;

/// Ana ekran için 2x büyük başlıklı panel
pub const HEAD_2X_H: f32 = 80.0;
pub const HEAD_2X_TOP_SLICE: f32 = 84.0;

/// Doku katmanı çerçevenin içinden başlasın
pub const TEXTURE_INSET: f32 = 4.0;

pub const GOLD_TEXT: Color = Color::srgb(0.95, 0.88, 0.68);
pub const BODY_TEXT: Color = Color::srgb(0.86, 0.85, 0.80);
pub const DIM_TEXT: Color = Color::srgb(0.62, 0.64, 0.68);
pub const DARK_TEXT: Color = Color::srgb(0.17, 0.11, 0.03);

pub const TIP_DEFAULT: &str = "İmleci bir düğmenin veya simgenin üstüne getir. (+/- : UI Ölçeği, Sol/Sağ Ok : Katalog Sayfası)";

#[derive(Component, Clone, Debug)]
pub struct ButtonSkin {
    pub text: Color,
    pub normal: Handle<Image>,
    pub hover: Handle<Image>,
    pub pressed: Handle<Image>,
    pub disabled: Handle<Image>,
}

#[derive(Resource, Clone)]
pub struct Skins {
    pub secondary: ButtonSkin,
    pub primary: ButtonSkin,
    pub danger: ButtonSkin,
    pub success: ButtonSkin,
    pub big_secondary: ButtonSkin,
    pub big_primary: ButtonSkin,
    pub big_danger: ButtonSkin,
    pub big_success: ButtonSkin,
    pub pill: ButtonSkin,
}

impl FromWorld for Skins {
    fn from_world(world: &mut World) -> Self {
        let server = world.resource::<AssetServer>();
        Self::load(server)
    }
}

impl Skins {
    pub fn load(server: &AssetServer) -> Self {
        let l = |p: &str| -> Handle<Image> { server.load(p.to_string()) };
        let skin = |kind: &str, big: bool, text: Color| {
            let pre = if big { "ui/button_big_" } else { "ui/button_" };
            ButtonSkin {
                text,
                normal: l(&format!("{pre}{kind}_normal.png")),
                hover: l(&format!("{pre}{kind}_hover.png")),
                pressed: l(&format!("{pre}{kind}_pressed.png")),
                disabled: l(&format!("{pre}{kind}_disabled.png")),
            }
        };
        Self {
            secondary: skin("secondary", false, BODY_TEXT),
            primary: skin("primary", false, DARK_TEXT),
            danger: skin("danger", false, Color::srgb(1.0, 0.92, 0.88)),
            success: skin("success", false, Color::srgb(0.94, 1.0, 0.92)),
            big_secondary: skin("secondary", true, BODY_TEXT),
            big_primary: skin("primary", true, DARK_TEXT),
            big_danger: skin("danger", true, Color::srgb(1.0, 0.92, 0.88)),
            big_success: skin("success", true, Color::srgb(0.94, 1.0, 0.92)),
            pill: ButtonSkin {
                text: GOLD_TEXT,
                normal: l("ui/hud_pill_normal.png"),
                hover: l("ui/hud_pill_hover.png"),
                pressed: l("ui/hud_pill_pressed.png"),
                disabled: l("ui/hud_pill_normal.png"),
            },
        }
    }
}

#[derive(Resource, Clone)]
pub struct Ui {
    pub font: Handle<Font>,
    pub panel: Handle<Image>,
    pub panel_plain: Handle<Image>,
    pub panel_alert: Handle<Image>,
    pub panel_success: Handle<Image>,
    pub headed: Handle<Image>,
    pub headed_2x: Handle<Image>,
    pub headed_plain: Handle<Image>,
    pub headed_plain_slate: Handle<Image>,
    pub headed_alert: Handle<Image>,
    pub headed_success: Handle<Image>,
    pub headed_slate: Handle<Image>,
    pub header: Handle<Image>,
    pub header_slate: Handle<Image>,
    pub header_alert: Handle<Image>,
    pub header_success: Handle<Image>,
    pub topbar: Handle<Image>,
    pub tooltip: Handle<Image>,
    pub inset: Handle<Image>,
    pub divider: Handle<Image>,
    pub vsep: Handle<Image>,
    pub texture: Handle<Image>,
    pub check_on: Handle<Image>,
    pub check_off: Handle<Image>,
    pub radio_on: Handle<Image>,
    pub radio_off: Handle<Image>,
    pub close_normal: Handle<Image>,
    pub close_hover: Handle<Image>,
    pub arrow_up: Handle<Image>,
    pub arrow_down: Handle<Image>,
    pub tab_on: Handle<Image>,
    pub tab_off: Handle<Image>,
    pub track: Handle<Image>,
    pub fill: Handle<Image>,
    pub handle: Handle<Image>,
    pub bar_green: Handle<Image>,
    pub bar_red: Handle<Image>,
    pub bar_blue: Handle<Image>,
    pub scroll_track: Handle<Image>,
    pub scroll_thumb: Handle<Image>,
    pub icon_frame: Handle<Image>,
    pub flag_frame: Handle<Image>,
    pub hud_crest: Handle<Image>,
    pub hud_ribbon: Handle<Image>,
    pub hud_vertical: Handle<Image>,
    pub hud_bottom: Handle<Image>,
    pub hud_date: Handle<Image>,
    pub pip_off: Handle<Image>,
    pub pip_on: Handle<Image>,
    pub flag_sample: Handle<Image>,
    pub menu_bg: Handle<Image>,
    pub vignette: Handle<Image>,
    pub icon_coin: Handle<Image>,
    pub icon_manpower: Handle<Image>,
    pub icon_shield: Handle<Image>,
    pub icon_prestige: Handle<Image>,
    pub icon_population: Handle<Image>,
    pub icon_crown: Handle<Image>,
    pub icon_science: Handle<Image>,
    pub icon_flag: Handle<Image>,
    pub icon_production: Handle<Image>,
}

impl FromWorld for Ui {
    fn from_world(world: &mut World) -> Self {
        let server = world.resource::<AssetServer>();
        Self::load(server)
    }
}

impl Ui {
    pub fn load(server: &AssetServer) -> Self {
        let l = |p: &str| -> Handle<Image> { server.load(p.to_string()) };
        Self {
            font: server.load("fonts/Marcellus-Regular.ttf"),
            panel: l("ui/panel.png"),
            panel_plain: l("ui/panel_plain.png"),
            panel_alert: l("ui/panel_alert.png"),
            panel_success: l("ui/panel_success.png"),
            headed: l("ui/panel_headed.png"),
            headed_2x: l("ui/panel_headed_2x.png"),
            headed_plain: l("ui/panel_headed_plain.png"),
            headed_plain_slate: l("ui/panel_headed_plain_slate.png"),
            headed_alert: l("ui/panel_headed_alert.png"),
            headed_success: l("ui/panel_headed_success.png"),
            headed_slate: l("ui/panel_headed_slate.png"),
            header: l("ui/header_bar.png"),
            header_slate: l("ui/header_bar_slate.png"),
            header_alert: l("ui/header_bar_alert.png"),
            header_success: l("ui/header_bar_success.png"),
            topbar: l("ui/topbar.png"),
            tooltip: l("ui/tooltip.png"),
            inset: l("ui/inset.png"),
            divider: l("ui/divider.png"),
            vsep: l("ui/vsep.png"),
            texture: l("ui/texture_diagonal_tile.png"),
            check_on: l("ui/checkbox_on.png"),
            check_off: l("ui/checkbox_off.png"),
            radio_on: l("ui/radio_on.png"),
            radio_off: l("ui/radio_off.png"),
            close_normal: l("ui/close_normal.png"),
            close_hover: l("ui/close_hover.png"),
            arrow_up: l("ui/arrow_up.png"),
            arrow_down: l("ui/arrow_down.png"),
            tab_on: l("ui/tab_active.png"),
            tab_off: l("ui/tab_inactive.png"),
            track: l("ui/slider_track.png"),
            fill: l("ui/slider_fill.png"),
            handle: l("ui/slider_handle.png"),
            bar_green: l("ui/progress_green.png"),
            bar_red: l("ui/progress_red.png"),
            bar_blue: l("ui/progress_blue.png"),
            scroll_track: l("ui/scroll_track.png"),
            scroll_thumb: l("ui/scroll_thumb.png"),
            icon_frame: l("ui/icon_frame.png"),
            flag_frame: l("ui/flag_frame.png"),
            hud_crest: l("ui/hud_flag_crest.png"),
            hud_ribbon: l("ui/hud_ribbon_bar.png"),
            hud_vertical: l("ui/hud_vertical_bar.png"),
            hud_bottom: l("ui/hud_bottom_bar.png"),
            hud_date: l("ui/hud_date_bar.png"),
            pip_off: l("ui/hud_speed_pip_off.png"),
            pip_on: l("ui/hud_speed_pip_on.png"),
            flag_sample: l("ui/flag_sample.png"),
            menu_bg: l("ui/menu_bg.png"),
            vignette: l("ui/menu_vignette.png"),
            icon_coin: l("ui/icon_coin.png"),
            icon_manpower: l("ui/icon_manpower.png"),
            icon_shield: l("ui/icon_shield.png"),
            icon_prestige: l("ui/icon_prestige.png"),
            icon_population: l("ui/icon_population.png"),
            icon_crown: l("ui/icon_crown.png"),
            icon_science: l("ui/icon_science.png"),
            icon_flag: l("ui/icon_flag.png"),
            icon_production: l("ui/icon_production.png"),
        }
    }

    pub fn icon(&self, name: &str) -> Handle<Image> {
        match name {
            "coin" => self.icon_coin.clone(),
            "manpower" => self.icon_manpower.clone(),
            "shield" => self.icon_shield.clone(),
            "prestige" => self.icon_prestige.clone(),
            "population" => self.icon_population.clone(),
            "crown" => self.icon_crown.clone(),
            "science" => self.icon_science.clone(),
            "flag" => self.icon_flag.clone(),
            "production" => self.icon_production.clone(),
            _ => self.icon_coin.clone(),
        }
    }
}

/// Tasarım dilinin tüm renk token'ları.
#[derive(Resource, Clone, Debug)]
pub struct ThemeColors {
    pub bg_base: Color,
    pub surface_flat: Color,
    pub surface_raised: Color,
    pub surface_overlay: Color,
    pub border_subtle: Color,
    pub border_default: Color,
    pub border_accent: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_disabled: Color,
    pub accent: Color,
    pub accent_hover: Color,
    pub accent_pressed: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub danger_hover: Color,
    pub danger_pressed: Color,
}

impl ThemeColors {
    #[inline] pub fn bg_void(&self) -> Color { self.bg_base }
    #[inline] pub fn overlay_dim(&self) -> Color { Color::srgba(0.03, 0.04, 0.07, 0.55) }
    #[inline] pub fn surface_panel(&self) -> Color { self.surface_flat }
    #[inline] pub fn accent_primary(&self) -> Color { self.accent }
    #[inline] pub fn accent_soft(&self) -> Color { Color::srgba(0.851, 0.647, 0.290, 0.22) }
    #[inline] pub fn accent_muted(&self) -> Color { Color::srgb(0.68, 0.62, 0.46) }
    #[inline] pub fn border_hover(&self) -> Color { self.border_default }
    #[inline] pub fn border_focus(&self) -> Color { self.border_accent }
    #[inline] pub fn text_inverse(&self) -> Color { DARK_TEXT }
    #[inline] pub fn text_inverse_secondary(&self) -> Color { Color::srgb(0.18, 0.22, 0.30) }
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            bg_base: Color::srgb(0.05, 0.07, 0.10),
            surface_flat: Color::srgba(0.08, 0.11, 0.15, 0.92),
            surface_raised: Color::srgba(0.11, 0.15, 0.20, 0.96),
            surface_overlay: Color::srgba(0.04, 0.06, 0.09, 0.96),
            border_subtle: Color::srgba(0.32, 0.36, 0.42, 0.65),
            border_default: Color::srgba(0.65, 0.54, 0.34, 0.85),
            border_accent: GOLD_TEXT,
            text_primary: BODY_TEXT,
            text_secondary: DIM_TEXT,
            text_disabled: Color::srgb(0.45, 0.47, 0.50),
            accent: GOLD_TEXT,
            accent_hover: Color::srgb(1.0, 0.93, 0.75),
            accent_pressed: Color::srgb(0.78, 0.65, 0.38),
            success: Color::srgb(0.38, 0.72, 0.42),
            warning: Color::srgb(0.88, 0.60, 0.22),
            danger: Color::srgb(0.82, 0.26, 0.24),
            danger_hover: Color::srgb(0.92, 0.34, 0.30),
            danger_pressed: Color::srgb(0.68, 0.18, 0.16),
        }
    }
}

/// 4px tabanlı katı ızgara boşluk token'ları.
#[derive(Resource, Clone, Copy, Debug)]
pub struct ThemeSpacing {
    pub xxs: f32, // 4px
    pub xs: f32,  // 8px
    pub sm: f32,  // 12px
    pub md: f32,  // 16px
    pub lg: f32,  // 24px
    pub xl: f32,  // 32px
    pub xxl: f32, // 48px
}

impl ThemeSpacing {
    #[inline] pub fn val_xxs(&self) -> Val { Val::Px(self.xxs) }
    #[inline] pub fn val_xs(&self) -> Val { Val::Px(self.xs) }
    #[inline] pub fn val_sm(&self) -> Val { Val::Px(self.sm) }
    #[inline] pub fn val_md(&self) -> Val { Val::Px(self.md) }
    #[inline] pub fn val_lg(&self) -> Val { Val::Px(self.lg) }
    #[inline] pub fn val_xl(&self) -> Val { Val::Px(self.xl) }
    #[inline] pub fn val_xxl(&self) -> Val { Val::Px(self.xxl) }
}

impl Default for ThemeSpacing {
    fn default() -> Self {
        Self {
            xxs: 4.0,
            xs: 8.0,
            sm: 12.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
            xxl: 48.0,
        }
    }
}

/// Yuvarlak köşe token'ları.
#[derive(Resource, Clone, Copy, Debug)]
pub struct ThemeRadius {
    pub xs: f32, // 2px
    pub sm: f32, // 4px
    pub md: f32, // 8px
    pub lg: f32, // 14px
}

impl ThemeRadius {
    #[inline] pub fn val_xs(&self) -> Val { Val::Px(self.xs) }
    #[inline] pub fn val_sm(&self) -> Val { Val::Px(self.sm) }
    #[inline] pub fn val_md(&self) -> Val { Val::Px(self.md) }
    #[inline] pub fn val_lg(&self) -> Val { Val::Px(self.lg) }
}

impl Default for ThemeRadius {
    fn default() -> Self {
        Self {
            xs: 2.0,
            sm: 4.0,
            md: 8.0,
            lg: 14.0,
        }
    }
}

/// Kenarlık kalınlık token'ları.
#[derive(Resource, Clone, Copy, Debug)]
pub struct ThemeBorders {
    pub thin: f32,    // 1.0px
    pub default: f32, // 1.5px
}

impl ThemeBorders {
    #[inline] pub fn hairline(&self) -> f32 { self.thin }
    #[inline] pub fn subtle(&self) -> f32 { self.default }
}

impl Default for ThemeBorders {
    fn default() -> Self {
        Self {
            thin: 1.0,
            default: 1.5,
        }
    }
}

/// Tipografi boyut ölçeği token'ları.
#[derive(Resource, Clone, Copy, Debug)]
pub struct ThemeTypography {
    pub display: f32,  // 34.0px
    pub headline: f32, // 22.0px
    pub title: f32,    // 18.0px
    pub label: f32,    // 15.0px
    pub body: f32,     // 14.0px
    pub caption: f32,  // 12.0px
}

impl ThemeTypography {
    #[inline] pub fn title_lg(&self) -> f32 { self.title }
    #[inline] pub fn title_sm(&self) -> f32 { 16.0 }
    #[inline] pub fn label_lg(&self) -> f32 { 16.0 }
    #[inline] pub fn label_md(&self) -> f32 { self.label }
    #[inline] pub fn label_sm(&self) -> f32 { 13.0 }
    #[inline] pub fn body_md(&self) -> f32 { 14.5 }
    #[inline] pub fn body_sm(&self) -> f32 { self.body }
}

impl Default for ThemeTypography {
    fn default() -> Self {
        Self {
            display: 34.0,
            headline: 22.0,
            title: 18.0,
            label: 15.0,
            body: 14.0,
            caption: 12.0,
        }
    }
}

/// Tüm token'ları merkezi olarak tutan Theme ana kaynağı.
#[derive(Resource, Default, Clone, Debug)]
pub struct Theme {
    pub colors: ThemeColors,
    pub spacing: ThemeSpacing,
    pub radius: ThemeRadius,
    pub borders: ThemeBorders,
    pub typography: ThemeTypography,
}
