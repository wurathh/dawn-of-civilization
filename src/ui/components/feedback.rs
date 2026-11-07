#![allow(deprecated)]

use crate::state::UiFonts;
use crate::ui::components::layout::Space;
use crate::ui::components::panels::{frame, sliced};
use crate::ui::components::text::{text, text_bundle, TextTone, TextVariant};
use crate::ui::theme::{BODY_TEXT, DIM_TEXT, TIP_BORDER, TIP_DEFAULT, Theme, Ui};
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

#[derive(Component, Clone, Copy)]
pub struct Tip(pub &'static str);

#[derive(Component, Clone)]
pub struct DynamicTip(pub String);

#[derive(Component)]
pub struct TipText;

#[derive(Component)]
pub struct UiTooltip;

#[derive(Component)]
pub struct UiProgressBar {
    pub progress: f32, // 0.0 .. 1.0
}

#[derive(Component)]
pub struct UiProgressBarFill;

#[derive(Component)]
pub struct UiModalBackdrop;

/// `tooltip.png` 9-slice çerçeveli statik/dinamik ipucu kutusu.
pub fn spawn_tooltip_box(
    p: &mut ChildSpawnerCommands,
    ui: &Ui,
    size: (Val, Val),
    txt: &str,
) {
    p.spawn(Node {
        width: size.0,
        height: size.1,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::horizontal(Val::Px(14.0)),
        position_type: PositionType::Relative,
        ..default()
    })
    .with_children(|tp| {
        frame(tp, &ui.tooltip, TIP_BORDER);
        text(tp, ui, txt, 15.0, BODY_TEXT);
    });
}

/// Sol-alt ipucu çubuğu (`TipText` bileşeni ile imleç altındaki `Tip` açıklamasını gösterir).
pub fn spawn_tooltip_bar(
    p: &mut ChildSpawnerCommands,
    ui: &Ui,
    left: f32,
    bottom: f32,
    width: f32,
    default_msg: &str,
) {
    p.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(left),
            bottom: Val::Px(bottom),
            width: Val::Px(width),
            height: Val::Px(36.0),
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(Val::Px(16.0)),
            ..default()
        },
        FocusPolicy::Pass,
        Pickable::IGNORE,
    ))
    .with_children(|b| {
        frame(b, &ui.tooltip, TIP_BORDER);
        b.spawn((
            TipText,
            Text::new(default_msg),
            TextFont {
                font: ui.font.clone().into(),
                font_size: FontSize::Px(13.5),
                ..default()
            },
            TextColor(DIM_TEXT),
            FocusPolicy::Pass,
            Pickable::IGNORE,
        ));
    });
}

pub fn tooltip_system(
    tips: Query<(&Interaction, Option<&Tip>, Option<&DynamicTip>)>,
    mut label: Query<&mut Text, With<TipText>>,
) {
    let found = tips.iter().find_map(|(i, static_t, dyn_t)| {
        if *i != Interaction::None {
            if let Some(t) = static_t {
                return Some(t.0.to_string());
            }
            if let Some(dt) = dyn_t {
                return Some(dt.0.clone());
            }
        }
        None
    });
    let msg = found.as_deref().unwrap_or(TIP_DEFAULT);
    for mut t in &mut label {
        if t.0 != msg {
            t.0 = msg.to_string();
        }
    }
}

pub fn tooltip_node(theme: &Theme) -> (Node, BackgroundColor, BorderColor, ZIndex, UiTooltip) {
    (
        Node {
            position_type: PositionType::Absolute,
            padding: UiRect::axes(Space::SM.val(&theme.spacing), Space::XS.val(&theme.spacing)),
            border: UiRect::all(Val::Px(theme.borders.thin)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.md)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Space::XS.val(&theme.spacing),
            display: Display::None,
            ..default()
        },
        BackgroundColor(theme.colors.surface_overlay),
        BorderColor::all(theme.colors.border_accent),
        ZIndex(100),
        UiTooltip,
    )
}

pub fn spawn_progress_bar<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    progress_val: f32, // 0.0 .. 1.0
    width: Val,
) -> EntityCommands<'a> {
    let clamped_p = progress_val.clamp(0.0, 1.0);
    let mut bar = parent.spawn((
        Node {
            width,
            height: Val::Px(14.0),
            padding: UiRect::all(Val::Px(1.5)),
            border: UiRect::all(Val::Px(theme.borders.thin)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.sm)),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme.colors.surface_raised),
        BorderColor::all(theme.colors.border_subtle),
        UiProgressBar {
            progress: clamped_p,
        },
    ));

    bar.with_children(|b| {
        b.spawn((
            Node {
                width: Val::Percent(clamped_p * 100.0),
                height: Val::Percent(100.0),
                border_radius: BorderRadius::all(Val::Px(theme.radius.sm * 0.7)),
                ..default()
            },
            BackgroundColor(theme.colors.accent),
            UiProgressBarFill,
        ));
    });

    bar
}

pub fn spawn_toast<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    message: impl Into<String>,
    tone: TextTone,
) -> EntityCommands<'a> {
    let mut toast = parent.spawn((
        Node {
            padding: UiRect::axes(Space::MD.val(&theme.spacing), Space::SM.val(&theme.spacing)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Space::SM.val(&theme.spacing),
            ..default()
        },
        ZIndex(90),
        DeferredTooltipFrame,
    ));

    toast.with_children(|t| {
        let txt = text_bundle(theme, fonts, message, TextVariant::Body, tone);
        t.spawn(txt);
    });

    toast
}

#[derive(Component)]
pub struct DeferredTooltipFrame;

pub fn sync_deferred_tooltip_frames(
    mut commands: Commands,
    ui: Option<Res<Ui>>,
    q: Query<Entity, With<DeferredTooltipFrame>>,
) {
    let Some(ui) = ui else { return };
    for entity in &q {
        commands
            .entity(entity)
            .insert(sliced(&ui.tooltip, TIP_BORDER))
            .remove::<DeferredTooltipFrame>();
    }
}
