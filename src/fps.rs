use crate::state::UiFonts;
use crate::ui::theme::Theme;
use bevy::picking::prelude::*;
use bevy::prelude::*;
use bevy::render::renderer::RenderAdapterInfo;
use std::fs;
use std::time::Duration;

#[derive(Component)]
pub struct FpsCounterText;

#[derive(Component)]
pub struct FpsDetailedOverlay;

#[derive(Component)]
pub struct FpsDetailText;

#[derive(Resource)]
pub struct SystemMonitor {
    pub is_expanded: bool,
    pub timer: Timer,
    pub frame_count: u32,
    pub current_fps: f32,
    pub current_frame_ms: f32,

    // CPU takibi (/proc/stat)
    pub prev_cpu_work: u64,
    pub prev_cpu_total: u64,
    pub cpu_usage_pct: f32,

    // RAM takibi (/proc/self/status & /proc/meminfo)
    pub app_ram_mb: f32,
    pub sys_ram_used_gb: f32,
    pub sys_ram_total_gb: f32,

    // GPU takibi (nvidia-smi / adapter)
    pub gpu_name: String,
    pub gpu_util_pct: Option<f32>,
    pub gpu_vram_used_mb: Option<f32>,
    pub gpu_vram_total_mb: Option<f32>,
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self {
            is_expanded: false,
            timer: Timer::new(Duration::from_millis(350), TimerMode::Repeating),
            frame_count: 0,
            current_fps: 60.0,
            current_frame_ms: 16.6,
            prev_cpu_work: 0,
            prev_cpu_total: 0,
            cpu_usage_pct: 0.0,
            app_ram_mb: 0.0,
            sys_ram_used_gb: 0.0,
            sys_ram_total_gb: 0.0,
            gpu_name: "GPU".to_string(),
            gpu_util_pct: None,
            gpu_vram_used_mb: None,
            gpu_vram_total_mb: None,
        }
    }
}

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SystemMonitor>()
            .add_systems(Startup, setup_fps_counter)
            .add_systems(Update, (toggle_fps_overlay, update_fps_counter));
    }
}

fn setup_fps_counter(mut commands: Commands, fonts: Res<UiFonts>, theme: Res<Theme>) {
    // 1. Ana Kompakt FPS Rozeti
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(theme.spacing.sm),
                right: Val::Px(theme.spacing.md),
                padding: UiRect::axes(Val::Px(theme.spacing.xs), Val::Px(theme.spacing.xxs)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(theme.borders.thin)),
                border_radius: BorderRadius::all(Val::Px(theme.radius.sm)),
                ..default()
            },
            BackgroundColor(theme.colors.surface_overlay),
            BorderColor::all(theme.colors.border_subtle),
            GlobalZIndex(1000),
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("-- FPS"),
                TextFont {
                    font: fonts.heading.clone(),
                    font_size: FontSize::Px(theme.typography.caption),
                    ..default()
                },
                TextColor(theme.colors.success),
                Pickable::IGNORE,
                FpsCounterText,
            ));
        });

    // 2. F12 Detaylı Donanım Monitoring Paneli (Başlangıçta gizli)
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(36.0),
                right: Val::Px(theme.spacing.md),
                width: Val::Px(290.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(theme.radius.md)),
                display: Display::None, // F12 ile açılır
                ..default()
            },
            BackgroundColor(Color::srgba(0.06, 0.08, 0.11, 0.95)), // %95 Opak Obsidyen
            BorderColor::all(Color::srgb(0.78, 0.62, 0.28)),        // Altın Kenarlık
            GlobalZIndex(1001),
            Pickable::IGNORE,
            FpsDetailedOverlay,
        ))
        .with_children(|panel| {
            // Başlık
            panel.spawn((
                Text::new("PERFORMANS VE DONANIM İZLEYİCİ [F12]"),
                TextFont {
                    font: fonts.heading.clone(),
                    font_size: FontSize::Px(theme.typography.label_sm()),
                    ..default()
                },
                TextColor(Color::srgb(0.94, 0.82, 0.45)),
                Pickable::IGNORE,
            ));

            // Ayırıcı çizgi
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    margin: UiRect::vertical(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.40, 0.32, 0.18)),
                Pickable::IGNORE,
            ));

            // Detaylı metin alanı
            panel.spawn((
                Text::new("Hesaplanıyor..."),
                TextFont {
                    font: fonts.medium.clone(),
                    font_size: FontSize::Px(theme.typography.caption),
                    ..default()
                },
                TextColor(theme.colors.text_primary),
                Pickable::IGNORE,
                FpsDetailText,
            ));
        });
}

/// F12 tuşuna basıldığında detaylı performans panelini açıp kapatır
fn toggle_fps_overlay(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut monitor: ResMut<SystemMonitor>,
    mut panel_q: Query<&mut Node, With<FpsDetailedOverlay>>,
) {
    if keyboard.just_pressed(KeyCode::F12) {
        monitor.is_expanded = !monitor.is_expanded;
        for mut node in &mut panel_q {
            node.display = if monitor.is_expanded {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}

fn update_fps_counter(
    time: Res<Time>,
    theme: Res<Theme>,
    adapter_info: Option<Res<RenderAdapterInfo>>,
    mut monitor: ResMut<SystemMonitor>,
    mut text_q: Query<(&mut Text, &mut TextColor), (With<FpsCounterText>, Without<FpsDetailText>)>,
    mut detail_q: Query<&mut Text, (With<FpsDetailText>, Without<FpsCounterText>)>,
) {
    monitor.frame_count += 1;
    monitor.timer.tick(time.delta());

    if monitor.timer.just_finished() {
        let elapsed = monitor.timer.duration().as_secs_f32();
        if elapsed > 0.0 && monitor.frame_count > 0 {
            monitor.current_fps = monitor.frame_count as f32 / elapsed;
            monitor.current_frame_ms = (elapsed / monitor.frame_count as f32) * 1000.0;
        }
        monitor.frame_count = 0;

        // GPU İsmini kaydet
        if let Some(ref adapter) = adapter_info {
            monitor.gpu_name = adapter.name.clone();
        }

        // 1. Kompakt FPS Rozetini güncelle
        for (mut text, mut color) in &mut text_q {
            if monitor.is_expanded {
                text.0 = format!("{:.0} FPS • {:.1}ms [F12: Açık]", monitor.current_fps, monitor.current_frame_ms);
            } else {
                text.0 = format!("{:.0} FPS • {:.1}ms", monitor.current_fps, monitor.current_frame_ms);
            }

            color.0 = if monitor.current_fps >= 55.0 {
                theme.colors.success
            } else if monitor.current_fps >= 30.0 {
                theme.colors.warning
            } else {
                theme.colors.danger
            };
        }

        // 2. Detaylı Panel Açıksa Donanım İstatistiklerini Oku ve Güncelle
        if monitor.is_expanded {
            poll_system_diagnostics(&mut monitor);

            for mut detail_text in &mut detail_q {
                let mut lines = Vec::new();

                // CPU
                lines.push(format!("• CPU Kullanımı:  {:.1}%", monitor.cpu_usage_pct));

                // RAM
                lines.push(format!(
                    "• Oyun Belleği:   {:.1} MB",
                    monitor.app_ram_mb
                ));
                lines.push(format!(
                    "• Sistem RAM:     {:.2} / {:.2} GB",
                    monitor.sys_ram_used_gb, monitor.sys_ram_total_gb
                ));

                // GPU
                lines.push(format!("• Ekran Kartı:   {}", monitor.gpu_name));
                if let (Some(util), Some(used), Some(total)) = (
                    monitor.gpu_util_pct,
                    monitor.gpu_vram_used_mb,
                    monitor.gpu_vram_total_mb,
                ) {
                    lines.push(format!("  └ GPU Yükü:    {:.0}%", util));
                    lines.push(format!("  └ VRAM:        {:.0} / {:.0} MB", used, total));
                }

                detail_text.0 = lines.join("\n");
            }
        }
    }
}

/// Linux /proc dosyalarından ve nvidia-smi'den donanım metriklerini hafifçe okur
fn poll_system_diagnostics(monitor: &mut SystemMonitor) {
    // 1. CPU Kullanımı (/proc/stat)
    if let Ok(stat_content) = fs::read_to_string("/proc/stat") {
        if let Some(first_line) = stat_content.lines().next() {
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() >= 5 {
                let user: u64 = parts[1].parse().unwrap_or(0);
                let nice: u64 = parts[2].parse().unwrap_or(0);
                let system: u64 = parts[3].parse().unwrap_or(0);
                let idle: u64 = parts[4].parse().unwrap_or(0);
                let iowait: u64 = parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(0);

                let work = user + nice + system;
                let total = work + idle + iowait;

                let diff_work = work.saturating_sub(monitor.prev_cpu_work);
                let diff_total = total.saturating_sub(monitor.prev_cpu_total);

                if diff_total > 0 {
                    monitor.cpu_usage_pct = (diff_work as f32 / diff_total as f32) * 100.0;
                }
                monitor.prev_cpu_work = work;
                monitor.prev_cpu_total = total;
            }
        }
    }

    // 2. RAM Kullanımı (/proc/self/status & /proc/meminfo)
    if let Ok(status) = fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(kb) = parts[1].parse::<f32>() {
                        monitor.app_ram_mb = kb / 1024.0;
                    }
                }
                break;
            }
        }
    }

    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        let mut total_kb = 0.0;
        let mut avail_kb = 0.0;
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    total_kb = parts[1].parse().unwrap_or(0.0);
                }
            } else if line.starts_with("MemAvailable:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    avail_kb = parts[1].parse().unwrap_or(0.0);
                }
            }
        }
        if total_kb > 0.0 {
            monitor.sys_ram_total_gb = total_kb / (1024.0 * 1024.0);
            monitor.sys_ram_used_gb = (total_kb - avail_kb) / (1024.0 * 1024.0);
        }
    }

    // 3. GPU Kullanımı (NVIDIA SMI)
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args(["--query-gpu=utilization.gpu,memory.used,memory.total", "--format=csv,noheader,nounits"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = stdout.trim().split(',').map(|s| s.trim()).collect();
            if parts.len() >= 3 {
                monitor.gpu_util_pct = parts[0].parse().ok();
                monitor.gpu_vram_used_mb = parts[1].parse().ok();
                monitor.gpu_vram_total_mb = parts[2].parse().ok();
            }
        }
    }
}
