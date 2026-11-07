#![allow(deprecated)]

use crate::state::UiFonts;
use crate::ui::components::buttons::{button, ui_button};
use crate::ui::components::layout::Space;
use crate::ui::components::panels::sliced_bar;
use crate::ui::components::text::{text, text_bundle, TextTone, TextVariant};
use crate::ui::theme::{BODY_TEXT, BUTTON_BORDER, DIM_TEXT, GOLD_TEXT, Skins, Theme, Ui};
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

#[derive(Component)]
pub struct VolumeFill(pub f32);

#[derive(Component)]
pub struct VolumeHandle;

#[derive(Component)]
pub struct VolumeLabel;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UiCheckbox {
    pub checked: bool,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UiToggle {
    pub active: bool,
}

#[derive(Component)]
pub struct UiCheckboxIcon;

/// Belirtilen sabit genişlikte (`bar_w`) ve yüzde (`pct`: 0..100) değerinde,
/// dolgu çubuğu ASLA yeniden ölçeklenmeden (sadece `Overflow::clip()` maskesiyle kesilerek) çizilen çubuk.
pub fn spawn_clipped_bar(
    p: &mut ChildSpawnerCommands,
    track: &Handle<Image>,
    fill_img: &Handle<Image>,
    bar_w: f32,
    pct: f32,
    track_volume_fill: bool,
) {
    let fill_w = (bar_w * (pct.clamp(0.0, 100.0) / 100.0)).round();
    p.spawn((
        Node {
            width: Val::Px(bar_w),
            height: Val::Px(14.0),
            flex_shrink: 0.0,
            position_type: PositionType::Relative,
            ..default()
        },
        sliced_bar(track),
        Pickable::IGNORE,
    ))
    .with_children(|t| {
        let mut clip_cmd = t.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(fill_w),
                height: Val::Px(14.0),
                overflow: Overflow::clip(),
                ..default()
            },
            Pickable::IGNORE,
        ));
        if track_volume_fill {
            clip_cmd.insert(VolumeFill(bar_w));
        }
        clip_cmd.with_child((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(bar_w),
                height: Val::Px(14.0),
                ..default()
            },
            sliced_bar(fill_img),
            Pickable::IGNORE,
        ));
    });
}

pub fn progress(p: &mut ChildSpawnerCommands, ui: &Ui, label: &str, bar: &Handle<Image>, pct: f32) {
    progress_width(p, ui, label, bar, 260.0, pct);
}

pub fn progress_width(
    p: &mut ChildSpawnerCommands,
    ui: &Ui,
    label: &str,
    bar: &Handle<Image>,
    bar_w: f32,
    pct: f32,
) {
    p.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        },
        Pickable::IGNORE,
    ))
    .with_children(|r| {
        text(r, ui, label, 15.0, DIM_TEXT);
        spawn_clipped_bar(r, &ui.track, bar, bar_w, pct, false);
    });
}

/// `bevy_menu_example` içerisindeki etkileşimli slider (özel Action bileşenleriyle).
pub fn spawn_slider_with_actions<A: Bundle + Clone>(
    p: &mut ChildSpawnerCommands,
    ui: &Ui,
    skins: &Skins,
    bar_w: f32,
    init_pct: f32,
    down_action: A,
    up_action: A,
) {
    let fill_w = (bar_w * (init_pct.clamp(0.0, 100.0) / 100.0)).round();
    p.spawn(Node {
        align_items: AlignItems::Center,
        column_gap: Val::Px(10.0),
        ..default()
    })
    .with_children(|r| {
        button(
            r,
            ui,
            &skins.secondary,
            BUTTON_BORDER,
            "-",
            Vec2::new(34.0, 30.0),
            down_action,
            Some("Sesi azalt"),
        );
        r.spawn((
            Node {
                width: Val::Px(bar_w),
                height: Val::Px(14.0),
                margin: UiRect::horizontal(Val::Px(6.0)),
                flex_shrink: 0.0,
                position_type: PositionType::Relative,
                ..default()
            },
            sliced_bar(&ui.track),
        ))
        .with_children(|t| {
            // Kırpma (clip) kabı: sadece genişliği değişir, içindeki dolgu çubuğu hep tam `bar_w` genişliğindedir!
            t.spawn((
                VolumeFill(bar_w),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Px(fill_w),
                    height: Val::Px(14.0),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ))
            .with_child((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Px(bar_w),
                    height: Val::Px(14.0),
                    ..default()
                },
                sliced_bar(&ui.fill),
            ));
            t.spawn((
                VolumeHandle,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(init_pct.clamp(0.0, 100.0)),
                    top: Val::Px(-7.0),
                    margin: UiRect::left(Val::Px(-7.0)),
                    width: Val::Px(14.0),
                    height: Val::Px(28.0),
                    ..default()
                },
                ImageNode::new(ui.handle.clone()),
            ));
        });
        button(
            r,
            ui,
            &skins.secondary,
            BUTTON_BORDER,
            "+",
            Vec2::new(34.0, 30.0),
            up_action,
            Some("Sesi artır"),
        );
        r.spawn((
            VolumeLabel,
            Text::new(format!("{}%", init_pct.round() as i32)),
            TextFont {
                font: ui.font.clone().into(),
                font_size: FontSize::Px(18.0),
                ..default()
            },
            TextColor(GOLD_TEXT),
        ));
    });
}

/// Eylemsiz (Observer ile kullanıma uygun) slider oluşturucu.
pub fn spawn_slider_interactive(
    p: &mut ChildSpawnerCommands,
    ui: &Ui,
    skins: &Skins,
    bar_w: f32,
    init_pct: f32,
    on_down: impl FnOnce(&mut EntityCommands),
    on_up: impl FnOnce(&mut EntityCommands),
) {
    let fill_w = (bar_w * (init_pct.clamp(0.0, 100.0) / 100.0)).round();
    p.spawn(Node {
        align_items: AlignItems::Center,
        column_gap: Val::Px(10.0),
        ..default()
    })
    .with_children(|r| {
        let mut down_btn = ui_button(
            r,
            ui,
            &skins.secondary,
            BUTTON_BORDER,
            "-",
            Vec2::new(34.0, 30.0),
            Some("Azalt"),
        );
        on_down(&mut down_btn);

        r.spawn((
            Node {
                width: Val::Px(bar_w),
                height: Val::Px(14.0),
                margin: UiRect::horizontal(Val::Px(6.0)),
                flex_shrink: 0.0,
                position_type: PositionType::Relative,
                ..default()
            },
            sliced_bar(&ui.track),
        ))
        .with_children(|t| {
            t.spawn((
                VolumeFill(bar_w),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Px(fill_w),
                    height: Val::Px(14.0),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ))
            .with_child((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Px(bar_w),
                    height: Val::Px(14.0),
                    ..default()
                },
                sliced_bar(&ui.fill),
            ));
            t.spawn((
                VolumeHandle,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(init_pct.clamp(0.0, 100.0)),
                    top: Val::Px(-7.0),
                    margin: UiRect::left(Val::Px(-7.0)),
                    width: Val::Px(14.0),
                    height: Val::Px(28.0),
                    ..default()
                },
                ImageNode::new(ui.handle.clone()),
            ));
        });

        let mut up_btn = ui_button(
            r,
            ui,
            &skins.secondary,
            BUTTON_BORDER,
            "+",
            Vec2::new(34.0, 30.0),
            Some("Artır"),
        );
        on_up(&mut up_btn);

        r.spawn((
            VolumeLabel,
            Text::new(format!("{}%", init_pct.round() as i32)),
            TextFont {
                font: ui.font.clone().into(),
                font_size: FontSize::Px(18.0),
                ..default()
            },
            TextColor(GOLD_TEXT),
        ));
    });
}

/// `bevy_menu_example` içerisindeki onay kutusu (checkbox) satırı.
pub fn checkbox_row<K: Component, A: Bundle>(
    p: &mut ChildSpawnerCommands,
    ui: &Ui,
    label: &str,
    kind: K,
    action: A,
) {
    p.spawn(Node {
        align_items: AlignItems::Center,
        column_gap: Val::Px(12.0),
        ..default()
    })
    .with_children(|r| {
        r.spawn((
            Interaction::default(),
            FocusPolicy::Block,
            kind,
            action,
            Node {
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                ..default()
            },
            ImageNode::new(ui.check_off.clone()),
        ));
        text(r, ui, label, 18.0, BODY_TEXT);
    });
}

/// Observer ile kullanmak için EU4 2x onay kutusu (`ui/checkbox_on.png` / `ui/checkbox_off.png`).
pub fn spawn_eu4_checkbox<'a>(
    p: &'a mut ChildSpawnerCommands,
    ui: &Ui,
    label_str: &str,
    checked: bool,
) -> EntityCommands<'a> {
    let img = if checked {
        ui.check_on.clone()
    } else {
        ui.check_off.clone()
    };
    let mut cmd = p.spawn((
        Interaction::default(),
        FocusPolicy::Block,
        UiCheckbox { checked },
        Node {
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        },
    ));
    cmd.with_children(|r| {
        r.spawn((
            UiCheckboxIcon,
            Node {
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                flex_shrink: 0.0,
                ..default()
            },
            ImageNode::new(img),
            FocusPolicy::Pass,
            Pickable::IGNORE,
        ));
        text(r, ui, label_str, 17.0, BODY_TEXT);
    });
    cmd
}

/// Statik / genel `spawn_checkbox` yardımcısı (`ui/checkbox_on.png` / `ui/checkbox_off.png` ile).
pub fn spawn_checkbox<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    label_text: impl Into<String>,
    checked: bool,
) -> EntityCommands<'a> {
    let mut row_entity = parent.spawn((
        Interaction::default(),
        FocusPolicy::Block,
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Space::SM.val(&theme.spacing),
            ..default()
        },
        UiCheckbox { checked },
        DeferredCheckboxIcon(checked),
    ));

    row_entity.with_children(|row| {
        row.spawn((
            UiCheckboxIcon,
            Node {
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                flex_shrink: 0.0,
                ..default()
            },
            ImageNode::default(),
            FocusPolicy::Pass,
            Pickable::IGNORE,
        ));
        let lbl = text_bundle(theme, fonts, label_text, TextVariant::Body, TextTone::Primary);
        row.spawn((lbl.0, lbl.1, lbl.2, FocusPolicy::Pass, Pickable::IGNORE));
    });

    row_entity
}

/// Toggle switch oluşturucu (`ui/radio_on.png` / `ui/radio_off.png` ile).
pub fn spawn_toggle<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    label_text: impl Into<String>,
    active: bool,
) -> EntityCommands<'a> {
    let mut row_entity = parent.spawn((
        Interaction::default(),
        FocusPolicy::Block,
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            column_gap: Space::MD.val(&theme.spacing),
            width: Val::Percent(100.0),
            ..default()
        },
        UiToggle { active },
        DeferredRadioIcon(active),
    ));

    row_entity.with_children(|row| {
        let lbl = text_bundle(theme, fonts, label_text, TextVariant::Body, TextTone::Primary);
        row.spawn((lbl.0, lbl.1, lbl.2, FocusPolicy::Pass, Pickable::IGNORE));
        row.spawn((
            UiCheckboxIcon,
            Node {
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                flex_shrink: 0.0,
                ..default()
            },
            ImageNode::default(),
            FocusPolicy::Pass,
            Pickable::IGNORE,
        ));
    });

    row_entity
}

#[derive(Component)]
pub struct DeferredCheckboxIcon(pub bool);

#[derive(Component)]
pub struct DeferredRadioIcon(pub bool);

pub fn sync_checkbox_and_radio_icons(
    mut commands: Commands,
    ui: Option<Res<Ui>>,
    def_chk_q: Query<(Entity, &DeferredCheckboxIcon, &Children)>,
    def_rad_q: Query<(Entity, &DeferredRadioIcon, &Children)>,
    changed_chk_q: Query<(&UiCheckbox, &Children), Changed<UiCheckbox>>,
    changed_tog_q: Query<(&UiToggle, &Children), Changed<UiToggle>>,
    mut icon_q: Query<&mut ImageNode, With<UiCheckboxIcon>>,
) {
    let Some(ui) = ui else { return };

    for (entity, def, children) in &def_chk_q {
        for child in children.iter() {
            if let Ok(mut img) = icon_q.get_mut(child) {
                img.image = if def.0 { ui.check_on.clone() } else { ui.check_off.clone() };
            }
        }
        commands.entity(entity).remove::<DeferredCheckboxIcon>();
    }

    for (entity, def, children) in &def_rad_q {
        for child in children.iter() {
            if let Ok(mut img) = icon_q.get_mut(child) {
                img.image = if def.0 { ui.radio_on.clone() } else { ui.radio_off.clone() };
            }
        }
        commands.entity(entity).remove::<DeferredRadioIcon>();
    }

    for (chk, children) in &changed_chk_q {
        for child in children.iter() {
            if let Ok(mut img) = icon_q.get_mut(child) {
                img.image = if chk.checked { ui.check_on.clone() } else { ui.check_off.clone() };
            }
        }
    }

    for (tog, children) in &changed_tog_q {
        for child in children.iter() {
            if let Ok(mut img) = icon_q.get_mut(child) {
                img.image = if tog.active { ui.radio_on.clone() } else { ui.radio_off.clone() };
            }
        }
    }
}
