use crate::state::UiFonts;
use crate::ui::components::layout::Space;
use crate::ui::components::text::{text_bundle, TextTone, TextVariant};
use crate::ui::theme::Theme;
use bevy::prelude::*;

#[derive(Component)]
pub struct UiChip;

/// İstatistik / Metrik gösterge kutucuğu (Örnek: Nüfus, Ekonomi, Eyalet sayısı).
pub fn spawn_stat_card<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    label_text: impl Into<String>,
    value_text: impl Into<String>,
) -> EntityCommands<'a> {
    let mut card = parent.spawn((
        Node {
            padding: UiRect::axes(Space::SM.val(&theme.spacing), Space::XS.val(&theme.spacing)),
            border: UiRect::all(Val::Px(theme.borders.thin)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.md)),
            flex_direction: FlexDirection::Column,
            row_gap: Space::XXS.val(&theme.spacing),
            ..default()
        },
        BackgroundColor(theme.colors.surface_raised),
        BorderColor::all(theme.colors.border_subtle),
    ));

    card.with_children(|c| {
        let lbl = text_bundle(theme, fonts, label_text, TextVariant::Caption, TextTone::Secondary);
        c.spawn(lbl);

        let val = text_bundle(theme, fonts, value_text, TextVariant::Label, TextTone::Primary);
        c.spawn(val);
    });

    card
}

/// Durum / Sayaç rozeti (Chip / Badge).
pub fn spawn_badge_chip<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    text: impl Into<String>,
    tone: TextTone,
) -> EntityCommands<'a> {
    let (bg, border) = match tone {
        TextTone::Accent => (
            Color::srgba(0.851, 0.647, 0.290, 0.20),
            theme.colors.border_accent,
        ),
        TextTone::Success => (
            Color::srgba(0.220, 0.631, 0.412, 0.20),
            theme.colors.success,
        ),
        TextTone::Danger => (
            Color::srgba(0.898, 0.243, 0.243, 0.20),
            theme.colors.danger,
        ),
        _ => (
            theme.colors.surface_raised,
            theme.colors.border_subtle,
        ),
    };

    let mut chip = parent.spawn((
        Node {
            padding: UiRect::axes(Space::XS.val(&theme.spacing), Space::XXS.val(&theme.spacing)),
            border: UiRect::all(Val::Px(theme.borders.thin)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.sm)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(bg),
        BorderColor::all(border),
        UiChip,
    ));

    chip.with_children(|c| {
        let t = text_bundle(theme, fonts, text, TextVariant::Caption, tone);
        c.spawn(t);
    });

    chip
}

/// Bayrak / İkon çerçevesi (Rounded bordered icon slot).
pub fn flag_slot_node(theme: &Theme, width: f32, height: f32) -> (Node, BackgroundColor, BorderColor) {
    (
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            border: UiRect::all(Val::Px(theme.borders.thin)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.sm)),
            overflow: Overflow::clip(),
            ..default()
        },
        BackgroundColor(theme.colors.surface_raised),
        BorderColor::all(theme.colors.border_default),
    )
}
