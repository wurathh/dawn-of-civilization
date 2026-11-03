use crate::ui::components::buttons::{ButtonState, ButtonVariant};
use bevy::asset::RenderAssetUsages;
use bevy::image::Image;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// Menü, kart ve HUD için 9-slice master dokuları ve prosedürel görsel efektler.
#[derive(Resource, Clone, Debug)]
pub struct UiTextures {
    // 9-Slice Buton Dokuları (64x64, 16px corner slice)
    pub btn_primary_normal: Handle<Image>,
    pub btn_primary_hover: Handle<Image>,
    pub btn_primary_pressed: Handle<Image>,

    pub btn_royal_normal: Handle<Image>,
    pub btn_royal_hover: Handle<Image>,
    pub btn_royal_pressed: Handle<Image>,

    pub btn_secondary_normal: Handle<Image>,
    pub btn_secondary_hover: Handle<Image>,
    pub btn_secondary_pressed: Handle<Image>,

    pub btn_danger_normal: Handle<Image>,
    pub btn_danger_hover: Handle<Image>,
    pub btn_danger_pressed: Handle<Image>,

    // 9-Slice %100 Opak Panel Dokuları (96x96, 24px corner slice)
    pub panel_ornate_gold: Handle<Image>,
    pub panel_slate_card: Handle<Image>,

    // Prosedürel Efektler (Ayraçlar ve Ortam Işıltıları)
    pub divider_gold: Handle<Image>,
    pub header_gold_glow: Handle<Image>,
}

impl UiTextures {
    /// Belirtilen buton varyantı ve durumuna göre uygun 9-slice görsel handle'ını döner.
    pub fn button_image(&self, variant: ButtonVariant, state: ButtonState) -> Option<Handle<Image>> {
        let handle = match (variant, state) {
            (ButtonVariant::Primary, ButtonState::Normal) => &self.btn_primary_normal,
            (ButtonVariant::Primary, ButtonState::Hovered) => &self.btn_primary_hover,
            (ButtonVariant::Primary, ButtonState::Pressed) => &self.btn_primary_pressed,

            (ButtonVariant::Royal, ButtonState::Normal) => &self.btn_royal_normal,
            (ButtonVariant::Royal, ButtonState::Hovered) => &self.btn_royal_hover,
            (ButtonVariant::Royal, ButtonState::Pressed) => &self.btn_royal_pressed,

            (ButtonVariant::Secondary, ButtonState::Normal) => &self.btn_secondary_normal,
            (ButtonVariant::Secondary, ButtonState::Hovered) => &self.btn_secondary_hover,
            (ButtonVariant::Secondary, ButtonState::Pressed) => &self.btn_secondary_pressed,

            (ButtonVariant::Danger, ButtonState::Normal) => &self.btn_danger_normal,
            (ButtonVariant::Danger, ButtonState::Hovered) => &self.btn_danger_hover,
            (ButtonVariant::Danger, ButtonState::Pressed) => &self.btn_danger_pressed,

            (ButtonVariant::Ghost, _) => return None,
        };
        Some(handle.clone())
    }

    /// Butonlar için 16px kenar korumalı 9-slice dilimleyici.
    #[inline]
    pub fn button_slicer() -> TextureSlicer {
        TextureSlicer {
            border: BorderRect::all(16.0),
            ..default()
        }
    }

    /// Paneller için 24px kenar korumalı 9-slice dilimleyici.
    #[inline]
    pub fn panel_slicer() -> TextureSlicer {
        TextureSlicer {
            border: BorderRect::all(24.0),
            ..default()
        }
    }
}

impl FromWorld for UiTextures {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let btn_primary_normal = asset_server.load("ui_textures/buttons/button_primary_normal.png");
        let btn_primary_hover = asset_server.load("ui_textures/buttons/button_primary_hover.png");
        let btn_primary_pressed = asset_server.load("ui_textures/buttons/button_primary_pressed.png");

        let btn_royal_normal = asset_server.load("ui_textures/buttons/button_royal_normal.png");
        let btn_royal_hover = asset_server.load("ui_textures/buttons/button_royal_hover.png");
        let btn_royal_pressed = asset_server.load("ui_textures/buttons/button_royal_pressed.png");

        let btn_secondary_normal = asset_server.load("ui_textures/buttons/button_secondary_normal.png");
        let btn_secondary_hover = asset_server.load("ui_textures/buttons/button_secondary_hover.png");
        let btn_secondary_pressed = asset_server.load("ui_textures/buttons/button_secondary_pressed.png");

        let btn_danger_normal = asset_server.load("ui_textures/buttons/button_danger_normal.png");
        let btn_danger_hover = asset_server.load("ui_textures/buttons/button_danger_hover.png");
        let btn_danger_pressed = asset_server.load("ui_textures/buttons/button_danger_pressed.png");

        let panel_ornate_gold = asset_server.load("ui_textures/panels/panel_ornate_gold.png");
        let panel_slate_card = asset_server.load("ui_textures/panels/panel_slate_card.png");

        let mut images = world.resource_mut::<Assets<Image>>();

        // 1. Yatay Altın Ayraç Çizgisi (256x2)
        let divider_gold = {
            let width = 256;
            let height = 2;
            let mut data = Vec::with_capacity((width * height * 4) as usize);
            for _ in 0..height {
                for x in 0..width {
                    let norm = (x as f32 / (width as f32 - 1.0)) * 2.0 - 1.0;
                    let factor = (1.0 - norm.abs().powf(1.8)).max(0.0);
                    let alpha = (factor * 230.0) as u8;
                    data.extend_from_slice(&[217, 165, 74, alpha]);
                }
            }
            images.add(create_rgba_image(width, height, data))
        };

        // 2. Başlık Altın Işıltı Gradyanı (4x64)
        let header_gold_glow = {
            let width = 4;
            let height = 64;
            let mut data = Vec::with_capacity((width * height * 4) as usize);
            for y in 0..height {
                let factor = 1.0 - (y as f32 / (height as f32 - 1.0));
                let alpha = (factor.powf(2.0) * 85.0) as u8;
                for _ in 0..width {
                    data.extend_from_slice(&[217, 165, 74, alpha]);
                }
            }
            images.add(create_rgba_image(width, height, data))
        };

        Self {
            btn_primary_normal,
            btn_primary_hover,
            btn_primary_pressed,
            btn_royal_normal,
            btn_royal_hover,
            btn_royal_pressed,
            btn_secondary_normal,
            btn_secondary_hover,
            btn_secondary_pressed,
            btn_danger_normal,
            btn_danger_hover,
            btn_danger_pressed,
            panel_ornate_gold,
            panel_slate_card,
            divider_gold,
            header_gold_glow,
        }
    }
}

fn create_rgba_image(width: u32, height: u32, data: Vec<u8>) -> Image {
    Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}
