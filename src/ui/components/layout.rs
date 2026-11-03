use crate::ui::theme::{Theme, ThemeSpacing};
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Space {
    XXS, // 4px
    XS,  // 8px
    SM,  // 12px
    MD,  // 16px
    LG,  // 24px
    XL,  // 32px
    XXL, // 48px
}

impl Space {
    pub fn px(self, spacing: &ThemeSpacing) -> f32 {
        match self {
            Space::XXS => spacing.xxs,
            Space::XS => spacing.xs,
            Space::SM => spacing.sm,
            Space::MD => spacing.md,
            Space::LG => spacing.lg,
            Space::XL => spacing.xl,
            Space::XXL => spacing.xxl,
        }
    }

    pub fn val(self, spacing: &ThemeSpacing) -> Val {
        Val::Px(self.px(spacing))
    }
}

/// Standart yatay satır (FlexDirection::Row) layout düğümü.
pub fn row(spacing: &ThemeSpacing, gap: Space) -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: gap.val(spacing),
        ..default()
    }
}

/// Standart dikey sütun (FlexDirection::Column) layout düğümü.
pub fn column(spacing: &ThemeSpacing, gap: Space) -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        row_gap: gap.val(spacing),
        ..default()
    }
}

/// Yatay esnek spacer.
pub fn spacer_h(spacing: &ThemeSpacing, size: Space) -> Node {
    Node {
        width: size.val(spacing),
        height: Val::Px(1.0),
        ..default()
    }
}

/// Dikey esnek spacer.
pub fn spacer_v(spacing: &ThemeSpacing, size: Space) -> Node {
    Node {
        width: Val::Px(1.0),
        height: size.val(spacing),
        ..default()
    }
}

/// Yatay ince ayırıcı çizgi (divider).
pub fn divider_h(theme: &Theme) -> (Node, BackgroundColor) {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(theme.borders.thin),
            margin: UiRect::vertical(Val::Px(theme.spacing.xs)),
            ..default()
        },
        BackgroundColor(theme.colors.border_subtle),
    )
}

/// Dikey ince ayırıcı çizgi.
pub fn divider_v(theme: &Theme) -> (Node, BackgroundColor) {
    (
        Node {
            width: Val::Px(theme.borders.thin),
            height: Val::Percent(100.0),
            margin: UiRect::horizontal(Val::Px(theme.spacing.xs)),
            ..default()
        },
        BackgroundColor(theme.colors.border_subtle),
    )
}
