use crate::map::types::{is_cursor_over_ui, MAP_HEIGHT, MAP_WIDTH_F32, WorldMapData};
use crate::state::AppState;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Kamera için pürüzsüz (smooth) yakınlaştırma ve kaydırma hedefleri
#[derive(Resource)]
pub struct CameraRig {
    pub target_pos: Vec2,
    pub target_scale: f32,
}

/// Sol tık ile haritayı sürükleme (pan) ve normal tıklama (eyalet seçimi) ayrımını tutar
#[derive(Resource, Default)]
pub struct MapDragState {
    pub left_press_on_map: bool,
    pub left_drag_dist: f32,
}

/// Yatay sonsuz döngü için harita kopyasının (-1 = Sol, 0 = Merkez, +1 = Sağ) indeksini tutar
#[derive(Component, Clone, Copy)]
pub struct WrapCopy(pub i32);

/// Harita üzerindeki bir öğenin merkez (kopya 0) X koordinatını saklar
#[derive(Component, Clone, Copy, Default)]
pub struct BaseMapX(pub f32);

pub fn camera_controller(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    time: Res<Time>,
    window_q: Single<&Window, With<PrimaryWindow>>,
    state: Res<State<AppState>>,
    map_data: Res<WorldMapData>,
    mut drag_state: ResMut<MapDragState>,
    mut rig: ResMut<CameraRig>,
    mut camera_q: Single<(&Camera, &GlobalTransform, &mut Transform), With<Camera2d>>,
) {
    let (camera, cam_global, ref mut cam_tf) = *camera_q;
    let dt = time.delta_secs().min(0.1);
    let win_w = window_q.width().max(1.0);
    let win_h = window_q.height().max(1.0);
    let state_cursor = window_q.cursor_position();

    let pointer_over_ui = is_cursor_over_ui(
        state_cursor,
        win_w,
        win_h,
        *state.get(),
        map_data.selected_province.is_some(),
    );

    if mouse_buttons.just_pressed(MouseButton::Left) {
        drag_state.left_press_on_map = !pointer_over_ui;
        drag_state.left_drag_dist = 0.0;
    }

    // Sadece dikey eksende maksimum uzaklaştırma sınırı (yatay eksen sonsuz!)
    let max_overscroll_y = 40.0;
    let max_scale_y = (MAP_HEIGHT as f32 + max_overscroll_y * 2.0) / win_h;
    let max_scale = max_scale_y.min(4.0);
    let min_scale = 0.08;

    let mut zoom_delta = 0.0;
    for ev in mouse_wheel.read() {
        if !pointer_over_ui {
            let step = match ev.unit {
                MouseScrollUnit::Line => ev.y * 0.14,
                MouseScrollUnit::Pixel => ev.y * 0.0012,
            };
            zoom_delta += step;
        }
    }

    if zoom_delta != 0.0 {
        let old_target_scale = rig.target_scale;
        let factor = (1.0 - zoom_delta).clamp(0.75, 1.30);
        let new_target_scale = (old_target_scale * factor).clamp(min_scale, max_scale);

        if let Some(cursor_pos) = state_cursor {
            if let Ok(world_before) = camera.viewport_to_world_2d(cam_global, cursor_pos) {
                let scale_ratio = new_target_scale / old_target_scale;
                let offset = world_before - rig.target_pos;
                rig.target_pos = world_before - offset * scale_ratio;
            }
        }
        rig.target_scale = new_target_scale;
    } else if rig.target_scale > max_scale {
        rig.target_scale = max_scale;
    }

    // Sol Tık, Sağ Tık veya Orta Tık ile Sürükleyerek Haritayı Kaydırma (Panning)
    let right_or_mid =
        mouse_buttons.pressed(MouseButton::Right) || mouse_buttons.pressed(MouseButton::Middle);
    let left_held = mouse_buttons.pressed(MouseButton::Left) && drag_state.left_press_on_map;

    if right_or_mid || left_held {
        let mut drag = Vec2::ZERO;
        for ev in mouse_motion.read() {
            drag += ev.delta;
        }
        if left_held {
            drag_state.left_drag_dist += drag.length();
        }
        // Sol tıkta küçük tıklama titremelerini yok saymak için 3.5 piksel eşiği
        let should_pan = right_or_mid || (left_held && drag_state.left_drag_dist > 3.5);
        if should_pan && drag != Vec2::ZERO {
            let current_scale = cam_tf.scale.x;
            rig.target_pos.x -= drag.x * current_scale;
            rig.target_pos.y += drag.y * current_scale;
            cam_tf.translation.x = rig.target_pos.x;
            cam_tf.translation.y = rig.target_pos.y;
        }
    } else {
        mouse_motion.clear();
    }

    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }

    if dir != Vec2::ZERO {
        let speed = 1150.0 * rig.target_scale.max(0.15);
        rig.target_pos += dir.normalize() * speed * dt;
    }

    // DİKEY EKSEN: Haritanın uçlarındaki şehirlere rahatça tıklanabilmesi için
    // dengeli ve ölçülü dikey kaydırma payı (margin)
    let half_map_h = MAP_HEIGHT as f32 * 0.5;
    let half_view_h_target = (win_h * 0.5) * rig.target_scale;
    let margin_y = half_view_h_target * 0.12 + 80.0;
    let limit_y = (half_map_h + margin_y - half_view_h_target).max(0.0);
    rig.target_pos.y = rig.target_pos.y.clamp(-limit_y, limit_y);

    // Pürüzsüz (Smooth) İnterpolasyon (Yatay sonsuz döngüde en kısa mesafeden geçiş):
    let half_map_w = MAP_WIDTH_F32 * 0.5;
    let dx = rig.target_pos.x - cam_tf.translation.x;
    if dx > half_map_w {
        rig.target_pos.x -= MAP_WIDTH_F32;
    } else if dx < -half_map_w {
        rig.target_pos.x += MAP_WIDTH_F32;
    }

    let lerp_speed = (10.0 * dt).clamp(0.0, 0.45);
    let new_scale = cam_tf.scale.x + (rig.target_scale - cam_tf.scale.x) * lerp_speed;
    cam_tf.scale = Vec3::splat(new_scale);

    cam_tf.translation.x += (rig.target_pos.x - cam_tf.translation.x) * lerp_speed;
    cam_tf.translation.y += (rig.target_pos.y - cam_tf.translation.y) * lerp_speed;

    // YATAY EKSEN SONSUZ DÖNGÜ (Infinite Horizontal Wrapping)
    if cam_tf.translation.x < -half_map_w {
        cam_tf.translation.x += MAP_WIDTH_F32;
        rig.target_pos.x += MAP_WIDTH_F32;
    } else if cam_tf.translation.x > half_map_w {
        cam_tf.translation.x -= MAP_WIDTH_F32;
        rig.target_pos.x -= MAP_WIDTH_F32;
    }

    let half_view_h = (win_h * 0.5) * new_scale;
    let cur_margin_y = half_view_h * 0.12 + 80.0;
    let cur_limit_y = (half_map_h + cur_margin_y - half_view_h).max(0.0);
    cam_tf.translation.y = cam_tf.translation.y.clamp(-cur_limit_y, cur_limit_y);
}
