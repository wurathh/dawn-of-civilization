#![allow(deprecated)]

use crate::state::UiFonts;
use crate::ui::components::layout::Space;
use crate::ui::components::panels::{frame, sliced};
use crate::ui::components::text::{text, text_bundle, TextTone, TextVariant};
use crate::ui::theme::{BAR_BORDER, BODY_TEXT, GOLD_TEXT, INSET_BORDER, TAB_BORDER, Theme, Ui};
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

#[derive(Component, Clone, Copy)]
pub struct TabButton(pub usize);

#[derive(Component)]
pub struct TabText;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UiSelectableRow {
    pub selected: bool,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UiTabItem {
    pub selected: bool,
    pub index: usize,
}

/// EU4 9-slice sekme düğmesi (`tab_active.png` / `tab_inactive.png`).
pub fn spawn_tab_button<'a>(
    p: &'a mut ChildSpawnerCommands,
    ui: &Ui,
    label_str: &str,
    index: usize,
    active: bool,
    size: Vec2,
) -> EntityCommands<'a> {
    let img = if active { &ui.tab_on } else { &ui.tab_off };
    let tcol = if active { GOLD_TEXT } else { BODY_TEXT };
    let mut cmd = p.spawn((
        Interaction::default(),
        FocusPolicy::Block,
        TabButton(index),
        UiTabItem {
            selected: active,
            index,
        },
        Node {
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_shrink: 0.0,
            ..default()
        },
        sliced(img, TAB_BORDER),
    ));
    cmd.with_children(|t| text(t, ui, label_str, 16.0, tcol));
    cmd
}

/// EU4 `inset.png` + `scroll_track.png` + `scroll_thumb.png` ile kaydırma kutusu.
pub fn spawn_scroll_box<'a>(
    p: &'a mut ChildSpawnerCommands,
    ui: &Ui,
    size: (Val, Val),
    thumb_top: f32,
    thumb_h: f32,
    body: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
    let mut cmd = p.spawn(Node {
        width: size.0,
        height: size.1,
        padding: UiRect::all(Val::Px(12.0)),
        justify_content: JustifyContent::SpaceBetween,
        position_type: PositionType::Relative,
        ..default()
    });
    cmd.with_children(|box_node| {
        frame(box_node, &ui.inset, INSET_BORDER);
        box_node
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                flex_grow: 1.0,
                ..default()
            })
            .with_children(body);
        box_node
            .spawn((
                Node {
                    width: Val::Px(14.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Relative,
                    flex_shrink: 0.0,
                    ..default()
                },
                sliced(&ui.scroll_track, BAR_BORDER),
                Pickable::IGNORE,
            ))
            .with_child((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(thumb_top),
                    left: Val::Px(0.0),
                    width: Val::Px(14.0),
                    height: Val::Px(thumb_h),
                    ..default()
                },
                sliced(&ui.scroll_thumb, BAR_BORDER),
                Pickable::IGNORE,
            ));
    });
    cmd
}

pub fn selectable_row_node(
    theme: &Theme,
    selected: bool,
) -> (Node, BackgroundColor, BorderColor, UiSelectableRow) {
    let bg = if selected {
        Color::srgba(0.851, 0.647, 0.290, 0.18)
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

pub fn spawn_tab_item<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    title_str: impl Into<String>,
    index: usize,
    selected: bool,
) -> EntityCommands<'a> {
    let text_tone = if selected {
        TextTone::Accent
    } else {
        TextTone::Primary
    };

    let mut tab_entity = parent.spawn((
        Interaction::default(),
        FocusPolicy::Block,
        Node {
            width: Val::Px(112.0),
            height: Val::Px(36.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_shrink: 0.0,
            ..default()
        },
        TabButton(index),
        UiTabItem { selected, index },
        DeferredTabImage(selected),
    ));

    tab_entity.with_children(|tab| {
        let t = text_bundle(theme, fonts, title_str, TextVariant::Label, text_tone);
        tab.spawn((t.0, t.1, t.2, FocusPolicy::Pass, Pickable::IGNORE));
    });

    tab_entity
}

#[derive(Component)]
pub struct DeferredTabImage(pub bool);

pub fn sync_deferred_tab_images(
    mut commands: Commands,
    ui: Option<Res<Ui>>,
    q: Query<(Entity, &DeferredTabImage)>,
) {
    let Some(ui) = ui else { return };
    for (entity, def) in &q {
        let img = if def.0 { &ui.tab_on } else { &ui.tab_off };
        commands
            .entity(entity)
            .insert(sliced(img, TAB_BORDER))
            .remove::<DeferredTabImage>();
    }
}
