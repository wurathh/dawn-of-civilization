use crate::state::UiFonts;
use bevy::picking::prelude::*;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct FpsCounterText;

#[derive(Resource)]
pub struct FpsDiagnostics {
    pub timer: Timer,
    pub frame_count: u32,
    pub current_fps: f32,
    pub current_frame_ms: f32,
}

impl Default for FpsDiagnostics {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(300), TimerMode::Repeating),
            frame_count: 0,
            current_fps: 60.0,
            current_frame_ms: 16.6,
        }
    }
}

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FpsDiagnostics>()
            .add_systems(Startup, setup_fps_counter)
            .add_systems(Update, update_fps_counter);
    }
}

fn setup_fps_counter(mut commands: Commands, fonts: Res<UiFonts>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                right: Val::Px(14.0),
                padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.07, 0.11, 0.88)),
            BorderColor::all(Color::srgba(0.65, 0.55, 0.35, 0.50)),
            GlobalZIndex(1000),
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("-- FPS"),
                TextFont {
                    font: fonts.heading.clone(),
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.35, 0.88, 0.55)),
                Pickable::IGNORE,
                FpsCounterText,
            ));
        });
}

fn update_fps_counter(
    time: Res<Time>,
    mut diag: ResMut<FpsDiagnostics>,
    mut text_q: Query<(&mut Text, &mut TextColor), With<FpsCounterText>>,
) {
    diag.frame_count += 1;
    diag.timer.tick(time.delta());

    if diag.timer.just_finished() {
        let elapsed = diag.timer.duration().as_secs_f32();
        if elapsed > 0.0 && diag.frame_count > 0 {
            diag.current_fps = diag.frame_count as f32 / elapsed;
            diag.current_frame_ms = (elapsed / diag.frame_count as f32) * 1000.0;
        }
        diag.frame_count = 0;

        for (mut text, mut color) in &mut text_q {
            text.0 = format!("{:.0} FPS • {:.1}ms", diag.current_fps, diag.current_frame_ms);

            color.0 = if diag.current_fps >= 55.0 {
                Color::srgb(0.35, 0.88, 0.55) // Canlı Zümrüt Yeşili
            } else if diag.current_fps >= 30.0 {
                Color::srgb(0.95, 0.78, 0.25) // Canlı Kehribar Sarısı
            } else {
                Color::srgb(0.95, 0.35, 0.35) // Mercan Kırmızısı
            };
        }
    }
}
