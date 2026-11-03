use crate::state::UiFonts;
use crate::ui::components::layout::Space;
use crate::ui::components::text::{text_bundle, TextTone, TextVariant};
use crate::ui::theme::Theme;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UiCheckbox {
    pub checked: bool,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UiToggle {
    pub active: bool,
}

#[derive(Component)]
pub struct UiToggleKnob;

#[derive(Component)]
pub struct UiCheckboxCheck;

/// Checkbox satırı oluşturucu (ChildSpawnerCommands üzerinden).
pub fn spawn_checkbox<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    label_text: impl Into<String>,
    checked: bool,
) -> EntityCommands<'a> {
    let mut row_entity = parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Space::SM.val(&theme.spacing),
            ..default()
        },
        UiCheckbox { checked },
    ));

    row_entity.with_children(|row| {
        // Kutu
        let box_bg = if checked {
            theme.colors.accent
        } else {
            theme.colors.surface_raised
        };
        let box_border = if checked {
            theme.colors.border_accent
        } else {
            theme.colors.border_default
        };

        row.spawn((
            Node {
                width: Val::Px(20.0),
                height: Val::Px(20.0),
                border: UiRect::all(Val::Px(theme.borders.default)),
                border_radius: BorderRadius::all(Val::Px(theme.radius.sm)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(box_bg),
            BorderColor::all(box_border),
            Pickable::IGNORE,
        ))
        .with_children(|box_node| {
            let mark_text = if checked { "✓" } else { "" };
            let t = text_bundle(theme, fonts, mark_text, TextVariant::Body, TextTone::Primary);
            box_node.spawn((
                t.0,
                t.1,
                TextColor(Color::srgb(0.08, 0.06, 0.02)),
                UiCheckboxCheck,
                Pickable::IGNORE,
            ));
        });

        // Etiket
        let lbl = text_bundle(theme, fonts, label_text, TextVariant::Body, TextTone::Primary);
        row.spawn((lbl.0, lbl.1, lbl.2, Pickable::IGNORE));
    });

    row_entity
}

/// Toggle switch oluşturucu (ChildSpawnerCommands üzerinden).
pub fn spawn_toggle<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    label_text: impl Into<String>,
    active: bool,
) -> EntityCommands<'a> {
    let mut row_entity = parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            column_gap: Space::MD.val(&theme.spacing),
            width: Val::Percent(100.0),
            ..default()
        },
        UiToggle { active },
    ));

    row_entity.with_children(|row| {
        // Etiket
        let lbl = text_bundle(theme, fonts, label_text, TextVariant::Body, TextTone::Primary);
        row.spawn((lbl.0, lbl.1, lbl.2, Pickable::IGNORE));

        // Switch Hap (Pill)
        let pill_bg = if active {
            theme.colors.accent
        } else {
            theme.colors.surface_raised
        };
        let pill_border = if active {
            theme.colors.border_accent
        } else {
            theme.colors.border_default
        };

        row.spawn((
            Node {
                width: Val::Px(44.0),
                height: Val::Px(24.0),
                padding: UiRect::all(Val::Px(3.0)),
                border: UiRect::all(Val::Px(theme.borders.default)),
                border_radius: BorderRadius::all(Val::Px(theme.radius.lg)),
                justify_content: if active { JustifyContent::FlexEnd } else { JustifyContent::FlexStart },
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(pill_bg),
            BorderColor::all(pill_border),
            Pickable::IGNORE,
        ))
        .with_children(|pill| {
            // Knob
            let knob_bg = if active {
                Color::srgb(0.08, 0.06, 0.02)
            } else {
                theme.colors.text_secondary
            };
            pill.spawn((
                Node {
                    width: Val::Px(16.0),
                    height: Val::Px(16.0),
                    border_radius: BorderRadius::all(Val::Px(theme.radius.md)),
                    ..default()
                },
                BackgroundColor(knob_bg),
                UiToggleKnob,
                Pickable::IGNORE,
            ));
        });
    });

    row_entity
}
