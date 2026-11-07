#![allow(deprecated, clippy::too_many_arguments)]

use crate::state::UiFonts;
use crate::ui::components::feedback::Tip;
use crate::ui::components::layout::Space;
use crate::ui::components::panels::sliced;
use crate::ui::components::text::{text, text_bundle, TextTone, TextVariant};
use crate::ui::theme::{BIG_BUTTON_BORDER, BODY_TEXT, BUTTON_BORDER, ButtonSkin, DARK_TEXT, GOLD_TEXT, Theme, Ui};
use bevy::picking::prelude::*;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Royal,
    Secondary,
    Success,
    Ghost,
    Danger,
    Pill,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonStyle {
    #[default]
    Solid,
    Card,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonState {
    #[default]
    Normal,
    Hovered,
    Pressed,
}

#[derive(Component, Clone, Debug)]
pub struct UiButton {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub style: ButtonStyle,
    pub disabled: bool,
    pub focused: bool,
}

impl Default for UiButton {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            style: ButtonStyle::Solid,
            disabled: false,
            focused: false,
        }
    }
}

#[derive(Component, Clone)]
pub struct CloseButton {
    pub normal: Handle<Image>,
    pub hover: Handle<Image>,
}

/// `bevy_menu_example` içerisindeki 9-slice dokulu buton oluşturucu.
pub fn button<'a, A: Bundle>(
    p: &'a mut ChildSpawnerCommands,
    ui: &Ui,
    skin: &ButtonSkin,
    border: f32,
    label: &str,
    size: Vec2,
    action: A,
    tip: Option<&'static str>,
) -> EntityCommands<'a> {
    let mut e = p.spawn((
        Interaction::default(),
        FocusPolicy::Block,
        skin.clone(),
        action,
        Node {
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_shrink: 0.0,
            ..default()
        },
        sliced(&skin.normal, border),
    ));
    if let Some(t) = tip {
        e.insert(Tip(t));
    }
    let color = skin.text;
    e.with_children(|b| text(b, ui, label, 17.0, color));
    e
}

/// Observer (`.observe(|_: On<PointerClick>, ...|)`) ile kullanmak için eylemsiz 9-slice buton oluşturucu.
pub fn ui_button<'a>(
    p: &'a mut ChildSpawnerCommands,
    ui: &Ui,
    skin: &ButtonSkin,
    border: f32,
    label: &str,
    size: Vec2,
    tip: Option<&'static str>,
) -> EntityCommands<'a> {
    let mut e = p.spawn((
        Interaction::default(),
        FocusPolicy::Block,
        skin.clone(),
        Node {
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_shrink: 0.0,
            ..default()
        },
        sliced(&skin.normal, border),
    ));
    if let Some(t) = tip {
        e.insert(Tip(t));
    }
    let color = skin.text;
    e.with_children(|b| text(b, ui, label, 17.0, color));
    e
}

/// Statik (sabit durumlu) buton önizlemesi — normal/hover/pressed/disabled durumunu doğrudan gösterir.
pub fn static_button(
    p: &mut ChildSpawnerCommands,
    ui: &Ui,
    img: Handle<Image>,
    border: f32,
    label: &str,
    size: Vec2,
    text_color: Color,
) {
    p.spawn((
        Node {
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_shrink: 0.0,
            ..default()
        },
        sliced(&img, border),
        Pickable::IGNORE,
    ))
    .with_children(|b| text(b, ui, label, 16.0, text_color));
}

/// Kapat (X) düğmesi oluşturucu (`close_normal.png` / `close_hover.png`).
pub fn close_button<'a>(
    p: &'a mut ChildSpawnerCommands,
    ui: &Ui,
    tip: Option<&'static str>,
) -> EntityCommands<'a> {
    let mut e = p.spawn((
        Interaction::default(),
        FocusPolicy::Block,
        CloseButton {
            normal: ui.close_normal.clone(),
            hover: ui.close_hover.clone(),
        },
        Node {
            width: Val::Px(28.0),
            height: Val::Px(28.0),
            flex_shrink: 0.0,
            ..default()
        },
        ImageNode::new(ui.close_normal.clone()),
    ));
    if let Some(t) = tip {
        e.insert(Tip(t));
    }
    e
}

/// 9-slice EU4 buton dokularını kullanan genel `spawn_button` yardımcısı.
pub fn spawn_button<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    text_str: impl Into<String>,
    variant: ButtonVariant,
    size: ButtonSize,
) -> EntityCommands<'a> {
    let content = text_str.into();
    let (min_w, height, is_big, text_variant) = match size {
        ButtonSize::Small => (84.0, 32.0, false, TextVariant::Caption),
        ButtonSize::Medium => (134.0, 36.0, false, TextVariant::Label),
        ButtonSize::Large => (220.0, 44.0, true, TextVariant::Title),
    };

    let (kind_str, text_color, text_tone) = match variant {
        ButtonVariant::Primary | ButtonVariant::Royal => ("primary", DARK_TEXT, TextTone::Inverse),
        ButtonVariant::Secondary | ButtonVariant::Ghost => ("secondary", BODY_TEXT, TextTone::Primary),
        ButtonVariant::Danger => ("danger", Color::srgb(1.0, 0.92, 0.88), TextTone::Primary),
        ButtonVariant::Success => ("success", Color::srgb(0.94, 1.0, 0.92), TextTone::Primary),
        ButtonVariant::Pill => ("secondary", GOLD_TEXT, TextTone::Accent),
    };

    let border = if is_big { BIG_BUTTON_BORDER } else { BUTTON_BORDER };

    let btn_comp = UiButton {
        variant,
        size,
        style: ButtonStyle::Solid,
        disabled: false,
        focused: false,
    };

    let mut entity = parent.spawn((
        Interaction::default(),
        FocusPolicy::Block,
        Node {
            min_width: Val::Px(min_w),
            height: Val::Px(height),
            padding: UiRect::horizontal(Val::Px(if is_big { 22.0 } else { 14.0 })),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Space::XS.val(&theme.spacing),
            flex_shrink: 0.0,
            ..default()
        },
        btn_comp,
        ButtonState::Normal,
        DeferredButtonSkin {
            kind: kind_str,
            big: is_big,
            border,
            text_color,
        },
    ));

    entity.with_children(|btn| {
        let tb = text_bundle(theme, fonts, content, text_variant, text_tone);
        btn.spawn((
            tb.0,
            tb.1,
            TextColor(text_color),
            FocusPolicy::Pass,
            Pickable::IGNORE,
        ));
    });

    entity
}

/// İkonlu 9-slice EU4 butonu oluşturucu.
pub fn spawn_icon_button<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    text_str: impl Into<String>,
    icon_path: impl Into<String>,
    icon_on_right: bool,
    icon_size: f32,
    variant: ButtonVariant,
    size: ButtonSize,
    asset_server: &AssetServer,
) -> EntityCommands<'a> {
    let content = text_str.into();
    let (min_w, height, is_big, text_variant) = match size {
        ButtonSize::Small => (96.0, 32.0, false, TextVariant::Caption),
        ButtonSize::Medium => (150.0, 36.0, false, TextVariant::Label),
        ButtonSize::Large => (240.0, 46.0, true, TextVariant::Title),
    };

    let (kind_str, text_color, text_tone) = match variant {
        ButtonVariant::Primary | ButtonVariant::Royal => ("primary", DARK_TEXT, TextTone::Inverse),
        ButtonVariant::Secondary | ButtonVariant::Ghost => ("secondary", BODY_TEXT, TextTone::Primary),
        ButtonVariant::Danger => ("danger", Color::srgb(1.0, 0.92, 0.88), TextTone::Primary),
        ButtonVariant::Success => ("success", Color::srgb(0.94, 1.0, 0.92), TextTone::Primary),
        ButtonVariant::Pill => ("secondary", GOLD_TEXT, TextTone::Accent),
    };

    let pre = if is_big { "ui/button_big_" } else { "ui/button_" };
    let border = if is_big { BIG_BUTTON_BORDER } else { BUTTON_BORDER };
    let skin = ButtonSkin {
        text: text_color,
        normal: asset_server.load(format!("{pre}{kind_str}_normal.png")),
        hover: asset_server.load(format!("{pre}{kind_str}_hover.png")),
        pressed: asset_server.load(format!("{pre}{kind_str}_pressed.png")),
        disabled: asset_server.load(format!("{pre}{kind_str}_disabled.png")),
    };

    let icon_handle: Handle<Image> = asset_server.load(icon_path.into());
    let tb = text_bundle(theme, fonts, content, text_variant, text_tone);

    let mut entity = parent.spawn((
        Interaction::default(),
        FocusPolicy::Block,
        Node {
            min_width: Val::Px(min_w),
            height: Val::Px(height),
            padding: UiRect::horizontal(Val::Px(if is_big { 24.0 } else { 16.0 })),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Space::XS.val(&theme.spacing),
            flex_shrink: 0.0,
            ..default()
        },
        sliced(&skin.normal, border),
        skin,
        UiButton {
            variant,
            size,
            style: ButtonStyle::Solid,
            disabled: false,
            focused: false,
        },
        ButtonState::Normal,
    ));

    entity.with_children(|btn| {
        if !icon_on_right {
            btn.spawn((
                ImageNode::new(icon_handle.clone()).with_color(text_color),
                Node {
                    width: Val::Px(icon_size),
                    height: Val::Px(icon_size),
                    ..default()
                },
                FocusPolicy::Pass,
                Pickable::IGNORE,
            ));
        }

        btn.spawn((
            tb.0,
            tb.1,
            TextColor(text_color),
            FocusPolicy::Pass,
            Pickable::IGNORE,
        ));

        if icon_on_right {
            btn.spawn((
                ImageNode::new(icon_handle).with_color(text_color),
                Node {
                    width: Val::Px(icon_size),
                    height: Val::Px(icon_size),
                    ..default()
                },
                FocusPolicy::Pass,
                Pickable::IGNORE,
            ));
        }
    });

    entity
}

#[derive(Component)]
pub struct DeferredButtonSkin {
    pub kind: &'static str,
    pub big: bool,
    pub border: f32,
    pub text_color: Color,
}

pub fn attach_deferred_button_skins(
    mut commands: Commands,
    server: Res<AssetServer>,
    q: Query<(Entity, &DeferredButtonSkin, Option<&UiButton>)>,
) {
    for (entity, def, opt_btn) in &q {
        let pre = if def.big { "ui/button_big_" } else { "ui/button_" };
        let normal: Handle<Image> = server.load(format!("{pre}{}_normal.png", def.kind));
        let hover: Handle<Image> = server.load(format!("{pre}{}_hover.png", def.kind));
        let pressed: Handle<Image> = server.load(format!("{pre}{}_pressed.png", def.kind));
        let disabled: Handle<Image> = server.load(format!("{pre}{}_disabled.png", def.kind));

        let is_disabled = opt_btn.map(|b| b.disabled).unwrap_or(false);
        let initial_img = if is_disabled { &disabled } else { &normal };

        commands
            .entity(entity)
            .insert((
                sliced(initial_img, def.border),
                ButtonSkin {
                    text: def.text_color,
                    normal,
                    hover,
                    pressed,
                    disabled,
                },
            ))
            .remove::<DeferredButtonSkin>();
    }
}

pub fn button_visuals(
    mut q: Query<(&Interaction, &ButtonSkin, Option<&UiButton>, &mut ImageNode), Changed<Interaction>>,
    mut close_q: Query<(&Interaction, &CloseButton, &mut ImageNode), (Changed<Interaction>, Without<ButtonSkin>)>,
) {
    for (interaction, skin, opt_btn, mut img) in &mut q {
        if opt_btn.map(|b| b.disabled).unwrap_or(false) {
            img.image = skin.disabled.clone();
            continue;
        }
        img.image = match interaction {
            Interaction::None => skin.normal.clone(),
            Interaction::Hovered => skin.hover.clone(),
            Interaction::Pressed => skin.pressed.clone(),
        };
    }
    for (interaction, close, mut img) in &mut close_q {
        img.image = match interaction {
            Interaction::None => close.normal.clone(),
            Interaction::Hovered | Interaction::Pressed => close.hover.clone(),
        };
    }
}

pub fn handle_button_pointer_over(
    trigger: On<PointerOver>,
    mut q: Query<(Option<&UiButton>, Option<&mut ButtonState>, Option<&ButtonSkin>, Option<&CloseButton>, Option<&mut ImageNode>)>,
) {
    if let Ok((opt_btn, opt_state, opt_skin, opt_close, opt_img)) = q.get_mut(trigger.entity) {
        let disabled = opt_btn.map(|b| b.disabled).unwrap_or(false);
        if disabled {
            return;
        }
        if let Some(mut state) = opt_state {
            if *state != ButtonState::Pressed {
                *state = ButtonState::Hovered;
            }
        }
        if let Some(mut img) = opt_img {
            if let Some(skin) = opt_skin {
                img.image = skin.hover.clone();
            } else if let Some(close) = opt_close {
                img.image = close.hover.clone();
            }
        }
    }
}

pub fn handle_button_pointer_out(
    trigger: On<PointerOut>,
    mut q: Query<(Option<&UiButton>, Option<&mut ButtonState>, Option<&ButtonSkin>, Option<&CloseButton>, Option<&mut ImageNode>)>,
) {
    if let Ok((opt_btn, opt_state, opt_skin, opt_close, opt_img)) = q.get_mut(trigger.entity) {
        let disabled = opt_btn.map(|b| b.disabled).unwrap_or(false);
        if disabled {
            return;
        }
        if let Some(mut state) = opt_state {
            *state = ButtonState::Normal;
        }
        if let Some(mut img) = opt_img {
            if let Some(skin) = opt_skin {
                img.image = skin.normal.clone();
            } else if let Some(close) = opt_close {
                img.image = close.normal.clone();
            }
        }
    }
}

pub fn handle_button_pointer_press(
    trigger: On<PointerPress>,
    mut q: Query<(Option<&UiButton>, Option<&mut ButtonState>, Option<&ButtonSkin>, Option<&mut ImageNode>)>,
) {
    if let Ok((opt_btn, opt_state, opt_skin, opt_img)) = q.get_mut(trigger.entity) {
        let disabled = opt_btn.map(|b| b.disabled).unwrap_or(false);
        if disabled {
            return;
        }
        if let Some(mut state) = opt_state {
            *state = ButtonState::Pressed;
        }
        if let (Some(mut img), Some(skin)) = (opt_img, opt_skin) {
            img.image = skin.pressed.clone();
        }
    }
}

pub fn handle_button_pointer_release(
    trigger: On<PointerRelease>,
    mut q: Query<(Option<&UiButton>, Option<&mut ButtonState>, Option<&ButtonSkin>, Option<&mut ImageNode>)>,
) {
    if let Ok((opt_btn, opt_state, opt_skin, opt_img)) = q.get_mut(trigger.entity) {
        let disabled = opt_btn.map(|b| b.disabled).unwrap_or(false);
        if disabled {
            return;
        }
        if let Some(mut state) = opt_state {
            *state = ButtonState::Hovered;
        }
        if let (Some(mut img), Some(skin)) = (opt_img, opt_skin) {
            img.image = skin.hover.clone();
        }
    }
}

pub fn update_button_visuals(
    mut q: Query<(&UiButton, &ButtonSkin, &mut ImageNode), Changed<UiButton>>,
) {
    for (btn, skin, mut img) in &mut q {
        if btn.disabled {
            img.image = skin.disabled.clone();
        }
    }
}
