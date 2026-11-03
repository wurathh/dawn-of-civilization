use bevy::prelude::*;

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
    #[inline] pub fn text_inverse(&self) -> Color { Color::srgb(0.04, 0.05, 0.08) }
    #[inline] pub fn text_inverse_secondary(&self) -> Color { Color::srgb(0.18, 0.22, 0.30) }
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            bg_base: Color::srgb(0.03, 0.05, 0.08),           // #080C14
            surface_flat: Color::srgba(0.055, 0.078, 0.125, 0.85), // rgba(14, 20, 32, 0.85)
            surface_raised: Color::srgba(0.082, 0.114, 0.173, 0.92), // rgba(21, 29, 44, 0.92)
            surface_overlay: Color::srgba(0.024, 0.035, 0.059, 0.96), // rgba(6, 9, 15, 0.96)
            border_subtle: Color::srgba(0.149, 0.196, 0.275, 0.55), // rgba(38, 50, 70, 0.55)
            border_default: Color::srgba(0.196, 0.259, 0.361, 0.80), // rgba(50, 66, 92, 0.80)
            border_accent: Color::srgba(0.851, 0.647, 0.290, 0.85),  // rgba(217, 165, 74, 0.85)
            text_primary: Color::srgb(0.925, 0.910, 0.875),  // #ECE8DF
            text_secondary: Color::srgb(0.608, 0.639, 0.690), // #9BA3B0
            text_disabled: Color::srgb(0.322, 0.357, 0.424),  // #525B6C
            accent: Color::srgb(0.851, 0.647, 0.290),        // #D9A54A
            accent_hover: Color::srgb(0.929, 0.722, 0.369),  // #EDB85E
            accent_pressed: Color::srgb(0.722, 0.522, 0.196), // #B88532
            success: Color::srgb(0.220, 0.631, 0.412),       // #38A169
            warning: Color::srgb(0.867, 0.420, 0.125),       // #DD6B20
            danger: Color::srgb(0.898, 0.243, 0.243),        // #E53E3E
            danger_hover: Color::srgb(0.957, 0.357, 0.357),  // #F55B5B
            danger_pressed: Color::srgb(0.753, 0.173, 0.173), // #C02C2C
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
    pub display: f32,  // 36.0px (EB Garamond ExtraBold)
    pub headline: f32, // 24.0px (EB Garamond ExtraBold)
    pub title: f32,    // 18.0px (Inter Bold)
    pub label: f32,    // 14.0px (Inter Bold)
    pub body: f32,     // 13.0px (Inter SemiBold)
    pub caption: f32,  // 11.0px (Inter Medium)
}

impl ThemeTypography {
    #[inline] pub fn title_lg(&self) -> f32 { self.title }
    #[inline] pub fn title_sm(&self) -> f32 { 16.0 }
    #[inline] pub fn label_lg(&self) -> f32 { 15.0 }
    #[inline] pub fn label_md(&self) -> f32 { self.label }
    #[inline] pub fn label_sm(&self) -> f32 { 12.0 }
    #[inline] pub fn body_md(&self) -> f32 { 14.0 }
    #[inline] pub fn body_sm(&self) -> f32 { self.body }
}

impl Default for ThemeTypography {
    fn default() -> Self {
        Self {
            display: 36.0,
            headline: 24.0,
            title: 18.0,
            label: 14.0,
            body: 13.0,
            caption: 11.0,
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
