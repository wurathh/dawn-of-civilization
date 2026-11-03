use crate::state::UiFonts;
use crate::ui::components::layout::Space;
use crate::ui::components::text::{text_bundle, TextTone, TextVariant};
use crate::ui::theme::Theme;
use bevy::prelude::*;

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

/// Hover Tooltip kapsayıcısı (Node, BackgroundColor, BorderColor, ZIndex, UiTooltip).
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

/// İlerleme çubuğu (Progress Bar) oluşturucu.
pub fn spawn_progress_bar<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    progress: f32, // 0.0 .. 1.0
    width: Val,
) -> EntityCommands<'a> {
    let clamped_p = progress.clamp(0.0, 1.0);
    let mut bar = parent.spawn((
        Node {
            width,
            height: Val::Px(10.0),
            padding: UiRect::all(Val::Px(1.5)),
            border: UiRect::all(Val::Px(theme.borders.thin)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.sm)),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme.colors.surface_raised),
        BorderColor::all(theme.colors.border_subtle),
        UiProgressBar { progress: clamped_p },
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

/// Bildirim / Toast kutusu oluşturucu.
pub fn spawn_toast<'a>(
    parent: &'a mut ChildSpawnerCommands,
    theme: &Theme,
    fonts: &UiFonts,
    message: impl Into<String>,
    tone: TextTone,
) -> EntityCommands<'a> {
    let border_col = match tone {
        TextTone::Success => theme.colors.success,
        TextTone::Danger => theme.colors.danger,
        TextTone::Warning => theme.colors.warning,
        _ => theme.colors.border_accent,
    };

    let mut toast = parent.spawn((
        Node {
            padding: UiRect::axes(Space::MD.val(&theme.spacing), Space::SM.val(&theme.spacing)),
            border: UiRect::all(Val::Px(theme.borders.default)),
            border_radius: BorderRadius::all(Val::Px(theme.radius.md)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Space::SM.val(&theme.spacing),
            ..default()
        },
        BackgroundColor(theme.colors.surface_raised),
        BorderColor::all(border_col),
        ZIndex(90),
    ));

    toast.with_children(|t| {
        let text = text_bundle(theme, fonts, message, TextVariant::Body, tone);
        t.spawn(text);
    });

    toast
}
