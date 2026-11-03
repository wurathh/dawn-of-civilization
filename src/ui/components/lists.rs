use crate::state::UiFonts;
use crate::ui::components::layout::Space;
use crate::ui::components::text::{text_bundle, TextTone, TextVariant};
use crate::ui::theme::Theme;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UiSelectableRow {
    pub selected: bool,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UiTabItem {
    pub selected: bool,
    pub index: usize,
}

/// Seçilebilir liste satırı (örnek: senaryo satırı, kayıt dosyası satırı).
pub fn selectable_row_node(
    theme: &Theme,
    selected: bool,
) -> (Node, BackgroundColor, BorderColor, UiSelectableRow) {
    let bg = if selected {
        Color::srgba(0.851, 0.647, 0.290, 0.18) // accent tint
    } else {
        theme.colors.surface_raised
    };

    let border = if selected {
        theme.colors.border_accent
    } else {
        theme.colors.border_subtle
    };

    (
        Node {
            width: Val::Percent(100.0),
            padding: UiRect::axes(Space::MD.val(&theme.spacing), Space::SM.val(&theme.spacing)),
            border: UiRect::all(Val::Px(theme.borders.thin)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.md)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        BackgroundColor(bg),
        BorderColor::all(border),
        UiSelectableRow { selected },
    )
}

/// Sekme öğesi (Tab Button).
pub fn spawn_tab_item<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    title: impl Into<String>,
    index: usize,
    selected: bool,
) -> EntityCommands<'a> {
    let (bg, border, text_tone) = if selected {
        (
            theme.colors.surface_raised,
            theme.colors.border_accent,
            TextTone::Accent,
        )
    } else {
        (
            Color::srgba(0.0, 0.0, 0.0, 0.0),
            Color::srgba(0.0, 0.0, 0.0, 0.0),
            TextTone::Secondary,
        )
    };

    let mut tab_entity = parent.spawn((
        Node {
            padding: UiRect::axes(Space::MD.val(&theme.spacing), Space::XS.val(&theme.spacing)),
            border: UiRect::bottom(Val::Px(theme.borders.default)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(bg),
        BorderColor::all(border),
        UiTabItem { selected, index },
    ));

    tab_entity.with_children(|tab| {
        let t = text_bundle(theme, fonts, title, TextVariant::Label, text_tone);
        tab.spawn((t.0, t.1, t.2, Pickable::IGNORE));
    });

    tab_entity
}
