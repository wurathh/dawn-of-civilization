use crate::map::camera::{MapDragState, WrapCopy};
use crate::map::hud::{
    refresh_province_panel_texts, HudHoverText, ProvincePanelDetails, ProvincePanelRoot,
    ProvincePanelTitle,
};
use crate::map::labels::get_badge_text_width;
use crate::map::types::{
    is_cursor_over_ui, map_pixel_to_world, world_to_map_pixel, ProvinceInfo, MAP_HEIGHT,
    MAP_WIDTH, MAP_WIDTH_F32, WorldMapData,
};
use crate::scenario::{ActiveScenario, CivilizationData};
use crate::state::{AppState, GameSession};
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::PrimaryWindow;
use std::collections::HashSet;

#[derive(Component)]
pub struct HoveredProvinceOverlay;

#[derive(Component)]
pub struct SelectedProvinceOverlay;

#[derive(Component)]
pub struct ProvinceHoverTooltipRoot;

#[derive(Component)]
pub struct ProvinceHoverTooltipBg;

#[derive(Component)]
pub struct ProvinceHoverTooltipBorder;

#[derive(Component)]
pub struct ProvinceHoverTooltipFlag;

#[derive(Component)]
pub struct ProvinceHoverTooltipTitle;

#[derive(Resource, Default)]
pub struct ProvinceHoverTooltipTimer {
    pub hovered_pid: Option<u32>,
    pub elapsed: f32,
}

/// Seçili bir ülkenin tüm eyaletlerini kapsayan ve haritada o ülkeyi vurgulayan maske dokusunu üretir.
pub fn build_country_mask_image(
    map_data: &WorldMapData,
    civ: &CivilizationData,
    fill_rgba: [u8; 4],
    border_rgba: [u8; 4],
) -> Option<(Image, Vec2, Vec2)> {
    if civ.provinces.is_empty() {
        return None;
    }

    let mut relevant_provinces = Vec::new();
    for &pid in &civ.provinces {
        if let Some(p) = map_data.provinces.get(&pid) {
            relevant_provinces.push(p);
        }
    }

    if relevant_provinces.is_empty() {
        return None;
    }

    let mut min_x = u32::MAX;
    let mut min_y = u32::MAX;
    let mut max_x = 0u32;
    let mut max_y = 0u32;

    let prov_set: HashSet<u16> = relevant_provinces.iter().map(|p| p.id as u16).collect();

    for p in &relevant_provinces {
        min_x = min_x.min(p.bbox[0]);
        min_y = min_y.min(p.bbox[1]);
        max_x = max_x.max(p.bbox[2]);
        max_y = max_y.max(p.bbox[3]);
    }

    if min_x > max_x || min_y > max_y {
        return None;
    }

    let orig_w = (max_x - min_x + 1).max(1);
    let orig_h = (max_y - min_y + 1).max(1);

    // Ülke büyüklüğüne göre dinamik adım (step):
    // Küçük ülkelerde 1:1 tam netlik, devasa coğrafyalarda hafif ölçekleme ile anında çizim!
    let total_pixels = orig_w as u64 * orig_h as u64;
    let step: u32 = if total_pixels > 20_000_000 {
        4
    } else if total_pixels > 8_000_000 {
        3
    } else if total_pixels > 1_500_000 {
        2
    } else {
        1
    };

    let w = ((orig_w + step - 1) / step).max(1);
    let h = ((orig_h + step - 1) / step).max(1);

    let mut mask = vec![0u8; (w * h) as usize];

    for (out_y, y) in (min_y..=max_y).step_by(step as usize).enumerate() {
        if out_y >= h as usize {
            break;
        }
        let runs = &map_data.row_runs[y as usize];
        let start_idx = match runs.binary_search_by_key(&(min_x as u16), |&(sx, _)| sx) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };

        let row_offset = out_y * (w as usize);

        for r_idx in start_idx..runs.len() {
            let (run_start, pid) = runs[r_idx];
            let run_start_u32 = run_start as u32;
            if run_start_u32 > max_x {
                break;
            }
            if prov_set.contains(&pid) {
                let run_end_u32 = if r_idx + 1 < runs.len() {
                    (runs[r_idx + 1].0 as u32).saturating_sub(1)
                } else {
                    map_data.width - 1
                };
                let x0 = run_start_u32.max(min_x);
                let x1 = run_end_u32.min(max_x);
                
                let out_x0 = (x0 - min_x) / step;
                let out_x1 = ((x1 - min_x) / step).min(w - 1);
                for ox in out_x0..=out_x1 {
                    mask[row_offset + ox as usize] = 1;
                }
            }
        }
    }

    let mut buf = vec![0u8; (w * h * 4) as usize];
    let w_usize = w as usize;
    for ly in 0..h {
        let ly_usize = ly as usize;
        let row_idx = ly_usize * w_usize;
        for lx in 0..w {
            let lx_usize = lx as usize;
            let idx = row_idx + lx_usize;
            if mask[idx] == 1 {
                let is_edge = lx == 0
                    || lx == w - 1
                    || ly == 0
                    || ly == h - 1
                    || mask[idx - 1] == 0
                    || mask[idx + 1] == 0
                    || mask[idx - w_usize] == 0
                    || mask[idx + w_usize] == 0;

                let c = if is_edge { border_rgba } else { fill_rgba };
                let b = idx * 4;
                buf[b] = c[0];
                buf[b + 1] = c[1];
                buf[b + 2] = c[2];
                buf[b + 3] = c[3];
            }
        }
    }

    let center_px = (min_x as f32 + max_x as f32 + 1.0) * 0.5;
    let center_py = (min_y as f32 + max_y as f32 + 1.0) * 0.5;
    let world_pos = map_pixel_to_world(center_px, center_py);
    let world_size = Vec2::new(orig_w as f32, orig_h as f32);

    let img = Image::new(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        buf,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    Some((img, world_pos, world_size))
}

pub fn build_province_mask_image(
    map_data: &WorldMapData,
    prov: &ProvinceInfo,
    fill_rgba: [u8; 4],
    border_rgba: [u8; 4],
) -> (Image, Vec2) {
    let [min_x, min_y, max_x, max_y] = prov.bbox;
    let w = (max_x - min_x + 1).max(1);
    let h = (max_y - min_y + 1).max(1);

    let mut mask = vec![0u8; (w * h) as usize];
    let target_pid = prov.id as u16;

    for y in min_y..=max_y {
        let runs = &map_data.row_runs[y as usize];
        let start_idx = match runs.binary_search_by_key(&(min_x as u16), |&(sx, _)| sx) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };

        for r_idx in start_idx..runs.len() {
            let (run_start, pid) = runs[r_idx];
            let run_start_u32 = run_start as u32;
            if run_start_u32 > max_x {
                break;
            }
            if pid == target_pid {
                let run_end_u32 = if r_idx + 1 < runs.len() {
                    (runs[r_idx + 1].0 as u32).saturating_sub(1)
                } else {
                    map_data.width - 1
                };
                let x0 = run_start_u32.max(min_x);
                let x1 = run_end_u32.min(max_x);
                let local_y = y - min_y;
                let row_offset = (local_y * w) as usize;
                for x in x0..=x1 {
                    mask[row_offset + (x - min_x) as usize] = 1;
                }
            }
        }
    }

    let mut buf = vec![0u8; (w * h * 4) as usize];
    for ly in 0..h {
        for lx in 0..w {
            let idx = (ly * w + lx) as usize;
            if mask[idx] == 1 {
                let is_edge = lx == 0
                    || lx == w - 1
                    || ly == 0
                    || ly == h - 1
                    || mask[idx - 1] == 0
                    || mask[idx + 1] == 0
                    || mask[idx - w as usize] == 0
                    || mask[idx + w as usize] == 0;

                let c = if is_edge { border_rgba } else { fill_rgba };
                let b = idx * 4;
                buf[b] = c[0];
                buf[b + 1] = c[1];
                buf[b + 2] = c[2];
                buf[b + 3] = c[3];
            }
        }
    }

    let center_px = (min_x as f32 + max_x as f32 + 1.0) * 0.5;
    let center_py = (min_y as f32 + max_y as f32 + 1.0) * 0.5;
    let world_pos = map_pixel_to_world(center_px, center_py);

    let img = Image::new(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        buf,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    (img, world_pos)
}

pub fn province_interaction(
    window_q: Single<&Window, With<PrimaryWindow>>,
    camera_q: Single<(&Camera, &GlobalTransform, &Transform), With<Camera2d>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<MapDragState>,
    mut map_data: ResMut<WorldMapData>,
    scenario: Res<ActiveScenario>,
    mut session: ResMut<GameSession>,
    state: Res<State<AppState>>,
    mut hover_text_q: Query<&mut Text, With<HudHoverText>>,
    mut prov_panel_q: Query<&mut Node, With<ProvincePanelRoot>>,
    mut prov_title_q: Query<
        (&mut Text, &mut TextColor),
        (
            With<ProvincePanelTitle>,
            Without<HudHoverText>,
            Without<ProvincePanelDetails>,
        ),
    >,
    mut prov_details_q: Query<
        &mut Text,
        (
            With<ProvincePanelDetails>,
            Without<HudHoverText>,
            Without<ProvincePanelTitle>,
        ),
    >,
) {
    let win_w = window_q.width();
    let win_h = window_q.height();

    let cursor_opt = window_q.cursor_position();
    let pointer_over_ui = is_cursor_over_ui(
        cursor_opt,
        win_w,
        win_h,
        *state.get(),
        map_data.selected_province.is_some(),
    );

    let (camera, cam_global, cam_tf) = *camera_q;
    let scale = cam_tf.scale.x;
    let is_scenario_select = *state.get() == AppState::ScenarioSelect;
    let allow_hover_highlight = !pointer_over_ui && (is_scenario_select || scale < 1.65);

    let mut cursor_pid = None;
    if !pointer_over_ui {
        if let Some(cursor_pos) = cursor_opt {
            if let Ok(world_pos) = camera.viewport_to_world_2d(cam_global, cursor_pos) {
                if let Some((px, py)) = world_to_map_pixel(world_pos) {
                    let mut pid = map_data.get_province_at(px, py);
                    // Kıyı veya dar sınır hatlarında toleranslı kolay tıklama:
                    if pid == 0 {
                        'snap: for r in 1..=3 {
                            for dy in [-r, 0, r] {
                                for dx in [-r, 0, r] {
                                    let nx = (px as i32 + dx).rem_euclid(MAP_WIDTH as i32) as u32;
                                    let ny = (py as i32 + dy).clamp(0, MAP_HEIGHT as i32 - 1) as u32;
                                    let near_pid = map_data.get_province_at(nx, ny);
                                    if near_pid != 0 {
                                        pid = near_pid;
                                        break 'snap;
                                    }
                                }
                            }
                        }
                    }
                    if pid != 0 {
                        cursor_pid = Some(pid);
                    }
                }
            }
        }
    }

    let new_hovered = if allow_hover_highlight {
        cursor_pid
    } else {
        None
    };

    if map_data.hovered_province != new_hovered {
        map_data.hovered_province = new_hovered;
    }

    if let Some(mut h_text) = hover_text_q.iter_mut().next() {
        if let Some(hid) = cursor_pid {
            if let Some(p) = map_data.provinces.get(&hid) {
                let owner_name = scenario
                    .province_owners
                    .get(&hid)
                    .and_then(|cid| scenario.civs.get(cid))
                    .map(|c| c.name.as_str())
                    .unwrap_or("Tarafsız Bölge");
                h_text.0 = format!(
                    "{} (ID: #{})  •  {}  •  Piksel: [X: {:.0}, Y: {:.0}]",
                    p.name, p.id, owner_name, p.city_px[0], p.city_px[1]
                );
            } else {
                h_text.0 = "Açık Deniz / Okyanus".to_string();
            }
        } else {
            h_text.0 = "Açık Deniz / Okyanus".to_string();
        }
    }

    // Sol tık bırakıldığında: eğer harita sürüklenmediyse (drag <= 5.0px) şehir/ülke seç!
    if mouse_buttons.just_released(MouseButton::Left) {
        let was_clean_click = drag_state.left_press_on_map && drag_state.left_drag_dist <= 5.0;
        drag_state.left_press_on_map = false;
        drag_state.left_drag_dist = 0.0;

        if was_clean_click && !pointer_over_ui {
            if is_scenario_select {
                // Senaryo / Ülke seçim ekranında: İl seçimi yapılmaz!
                // Tıklanan ülke seçilir ve kamera kullanıcı zaten oraya baktığı için hareket etmez.
                map_data.selected_province = None;
                if let Some(pid) = cursor_pid {
                    if let Some(&cid) = scenario.province_owners.get(&pid) {
                        session.player_civ_id = cid;
                    }
                }
            } else {
                // Oyun içinde (InGame): İl seçimi yapılır
                map_data.selected_province = cursor_pid;
                refresh_province_panel_texts(
                    map_data.selected_province,
                    &map_data,
                    &scenario,
                    &mut prov_panel_q,
                    &mut prov_title_q,
                    &mut prov_details_q,
                );
            }
        }
    }
}

pub fn update_province_overlays(
    map_data: Res<WorldMapData>,
    scenario: Res<ActiveScenario>,
    session: Res<GameSession>,
    state: Res<State<AppState>>,
    mut images: ResMut<Assets<Image>>,
    mut hover_q: Query<
        (&mut Sprite, &mut Transform, &mut Visibility, &WrapCopy),
        (With<HoveredProvinceOverlay>, Without<SelectedProvinceOverlay>),
    >,
    mut select_q: Query<
        (&mut Sprite, &mut Transform, &mut Visibility, &WrapCopy),
        (With<SelectedProvinceOverlay>, Without<HoveredProvinceOverlay>),
    >,
    mut prev_hovered: Local<Option<u32>>,
    mut prev_hovered_civ: Local<Option<u32>>,
    mut prev_selected: Local<Option<u32>>,
    mut prev_civ_id: Local<Option<u32>>,
    mut prev_state: Local<Option<AppState>>,
) {
    let cur_state = *state.get();
    let is_scenario_select = cur_state == AppState::ScenarioSelect;

    // 1. Hover Overlay Kontrolü:
    if is_scenario_select {
        // Senaryo / Ülke Seçim Ekranı:
        // Fare hangi ülkenin üzerindeyse uzaktan bakıldığında dahi O ÜLKENİN TAMAMI parıldasın!
        let hovered_cid = map_data
            .hovered_province
            .and_then(|hid| scenario.province_owners.get(&hid).copied());

        let hover_civ_changed = hovered_cid != *prev_hovered_civ;
        let selected_civ_changed = Some(session.player_civ_id) != *prev_civ_id;
        let state_changed = *prev_state != Some(cur_state);

        if hover_civ_changed || selected_civ_changed || state_changed {
            *prev_hovered_civ = hovered_cid;
            if let Some(cid) = hovered_cid {
                if cid != session.player_civ_id {
                    if let Some(civ) = scenario.civs.get(&cid) {
                        if let Some((img, pos, world_size)) = build_country_mask_image(
                            &map_data,
                            civ,
                            [255, 255, 255, 38], // Zarif yarı saydam beyaz/gümüş parıltı
                            [255, 255, 255, 190], // Belirgin dış sınır vurgusu
                        ) {
                            let handle = images.add(img);
                            for (mut h_sprite, mut h_tf, mut h_vis, wrap) in &mut hover_q {
                                h_sprite.image = handle.clone();
                                h_sprite.custom_size = Some(world_size);
                                h_tf.translation.x = pos.x + wrap.0 as f32 * MAP_WIDTH_F32;
                                h_tf.translation.y = pos.y;
                                *h_vis = Visibility::Visible;
                            }
                        }
                    }
                } else {
                    // Halihazırda seçili ülkenin üstündeyken hover parıltısını gizle (çünkü zaten altın renginde yanıyor)
                    for (_, _, mut h_vis, _) in &mut hover_q {
                        *h_vis = Visibility::Hidden;
                    }
                }
            } else {
                for (_, _, mut h_vis, _) in &mut hover_q {
                    *h_vis = Visibility::Hidden;
                }
            }
        }
    } else {
        // Oyun İçi Mod: Tekil eyalet hover vurgusu
        if *prev_hovered != map_data.hovered_province
            || *prev_selected != map_data.selected_province
            || *prev_state != Some(cur_state)
        {
            if let Some(hid) = map_data.hovered_province {
                if Some(hid) != map_data.selected_province {
                    if let Some(prov) = map_data.provinces.get(&hid) {
                        let (img, pos) = build_province_mask_image(
                            &map_data,
                            prov,
                            [255, 245, 215, 45],
                            [255, 248, 220, 140],
                        );
                        let handle = images.add(img);
                        for (mut h_sprite, mut h_tf, mut h_vis, wrap) in &mut hover_q {
                            h_sprite.image = handle.clone();
                            h_sprite.custom_size = None;
                            h_tf.translation.x = pos.x + wrap.0 as f32 * MAP_WIDTH_F32;
                            h_tf.translation.y = pos.y;
                            *h_vis = Visibility::Visible;
                        }
                    }
                } else {
                    for (_, _, mut h_vis, _) in &mut hover_q {
                        *h_vis = Visibility::Hidden;
                    }
                }
            } else {
                for (_, _, mut h_vis, _) in &mut hover_q {
                    *h_vis = Visibility::Hidden;
                }
            }
        }
    }

    // 2. Seçim (Selection) Overlay Kontrolü:
    // Senaryo seçim ekranında (ScenarioSelect) -> Bütün ülkenin sınırları altın rengiyle seçilir!
    // Oyun içinde (InGame) -> Tek bir eyalet seçilir!
    let civ_changed = is_scenario_select && *prev_civ_id != Some(session.player_civ_id);
    let sel_prov_changed = !is_scenario_select && *prev_selected != map_data.selected_province;
    let state_changed = *prev_state != Some(cur_state);

    if civ_changed || sel_prov_changed || state_changed || scenario.is_changed() {
        if is_scenario_select {
            // Senaryo Seçim Modu: Seçili ülkenin tamamını altın sınırla parlat
            if let Some(civ) = scenario.civs.get(&session.player_civ_id) {
                if let Some((img, pos, world_size)) = build_country_mask_image(
                    &map_data,
                    civ,
                    [255, 215, 80, 50],
                    [255, 235, 110, 240],
                ) {
                    let handle = images.add(img);
                    for (mut s_sprite, mut s_tf, mut s_vis, wrap) in &mut select_q {
                        s_sprite.image = handle.clone();
                        s_sprite.custom_size = Some(world_size);
                        s_tf.translation.x = pos.x + wrap.0 as f32 * MAP_WIDTH_F32;
                        s_tf.translation.y = pos.y;
                        *s_vis = Visibility::Visible;
                    }
                } else {
                    for (_, _, mut s_vis, _) in &mut select_q {
                        *s_vis = Visibility::Hidden;
                    }
                }
            } else {
                for (_, _, mut s_vis, _) in &mut select_q {
                    *s_vis = Visibility::Hidden;
                }
            }
        } else {
            // Oyun İçi Mod: Tekil eyalet seçimi
            if let Some(sid) = map_data.selected_province {
                if let Some(prov) = map_data.provinces.get(&sid) {
                    let (img, pos) = build_province_mask_image(
                        &map_data,
                        prov,
                        [245, 200, 50, 95],
                        [255, 235, 95, 230],
                    );
                    let handle = images.add(img);
                    for (mut s_sprite, mut s_tf, mut s_vis, wrap) in &mut select_q {
                        s_sprite.image = handle.clone();
                        s_sprite.custom_size = None;
                        s_tf.translation.x = pos.x + wrap.0 as f32 * MAP_WIDTH_F32;
                        s_tf.translation.y = pos.y;
                        *s_vis = Visibility::Visible;
                    }
                }
            } else {
                for (_, _, mut s_vis, _) in &mut select_q {
                    *s_vis = Visibility::Hidden;
                }
            }
        }
    }

    *prev_hovered = map_data.hovered_province;
    *prev_selected = map_data.selected_province;
    *prev_civ_id = Some(session.player_civ_id);
    *prev_state = Some(cur_state);
}

pub fn update_province_hover_tooltip(
    time: Res<Time>,
    window_q: Single<&Window, With<PrimaryWindow>>,
    camera_q: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    map_data: Res<WorldMapData>,
    scenario: Res<ActiveScenario>,
    asset_server: Res<AssetServer>,
    state: Res<State<AppState>>,
    mut timer: ResMut<ProvinceHoverTooltipTimer>,
    mut tooltip_params: ParamSet<(
        Query<(&mut Transform, &mut Visibility), (With<ProvinceHoverTooltipRoot>, Without<Camera2d>)>,
        Query<&mut Sprite, With<ProvinceHoverTooltipBg>>,
        Query<&mut Transform, (With<ProvinceHoverTooltipBorder>, Without<Camera2d>)>,
        Query<(&mut Sprite, &mut Transform), (With<ProvinceHoverTooltipFlag>, Without<Camera2d>)>,
        Query<(&mut Text2d, &mut Transform), (With<ProvinceHoverTooltipTitle>, Without<Camera2d>)>,
    )>,
) {
    let mut root_q = tooltip_params.p0();
    let Some((_, mut root_vis)) = root_q.iter_mut().next() else {
        return;
    };

    let win_w = window_q.width();
    let win_h = window_q.height();
    let cursor_opt = window_q.cursor_position();

    let pointer_over_ui = is_cursor_over_ui(
        cursor_opt,
        win_w,
        win_h,
        *state.get(),
        map_data.selected_province.is_some(),
    );

    let cur_hovered = if pointer_over_ui {
        None
    } else {
        map_data.hovered_province
    };
    if timer.hovered_pid != cur_hovered {
        timer.hovered_pid = cur_hovered;
        timer.elapsed = 0.0;
    } else if cur_hovered.is_some() {
        timer.elapsed += time.delta_secs();
    }

    let is_scenario_select = *state.get() == AppState::ScenarioSelect;
    // Ülke seçim ekranında gecikme olmaksızın anında belirir, oyun içinde titremeyi önlemek için 0.28s gecikmeli açılır
    let should_show = cur_hovered.is_some() && (is_scenario_select || timer.elapsed >= 0.28);

    if !should_show {
        if *root_vis != Visibility::Hidden {
            *root_vis = Visibility::Hidden;
        }
        return;
    }

    let Some(pid) = cur_hovered else {
        *root_vis = Visibility::Hidden;
        return;
    };

    let Some(prov) = map_data.provinces.get(&pid) else {
        *root_vis = Visibility::Hidden;
        return;
    };

    let Some(cursor_pos) = window_q.cursor_position() else {
        *root_vis = Visibility::Hidden;
        return;
    };

    let (camera, cam_global) = *camera_q;
    let cam_scale = cam_global.compute_transform().scale.x;

    let owner_civ = scenario
        .province_owners
        .get(&pid)
        .and_then(|cid| scenario.civs.get(cid));

    let flag_path = if let Some(civ) = owner_civ {
        ActiveScenario::get_flag_path(&civ.tag, None)
    } else {
        "flags/unkn/0.png".to_string()
    };

    let display_str = if is_scenario_select {
        // Senaryo / Ülke seçim ekranında: Sadece Ülke Adı (ve bayrağı)
        if let Some(civ) = owner_civ {
            civ.name.clone()
        } else {
            "Bilinmeyen Devlet".to_string()
        }
    } else {
        // Oyun içi mod: Sadece İlin Adı (ve bayrağı)
        prov.name.clone()
    };

    // Başkent rozeti mimarisi, bir tık daha büyük ve ferah oranlar:
    let text_w = get_badge_text_width(&display_str) * 1.15;
    let badge_h = 26.0;
    // Sol margin 8px, bayrak 19px, ara boşluk 5px, sağ margin 8px (sol ile milimetrik eşit simetri)
    let badge_w = (text_w + 40.0).max(48.0);
    let flag_center_x = -badge_w * 0.5 + 17.5;
    let text_start_x = -badge_w * 0.5 + 32.0;

    if let Some(mut bg_sprite) = tooltip_params.p1().iter_mut().next() {
        bg_sprite.custom_size = Some(Vec2::new(badge_w, badge_h));
    }

    if let Some(mut border_tf) = tooltip_params.p2().iter_mut().next() {
        border_tf.translation.x = flag_center_x;
    }

    if let Some((mut flag_sprite, mut flag_tf)) = tooltip_params.p3().iter_mut().next() {
        flag_sprite.image = asset_server.load(flag_path);
        flag_tf.translation.x = flag_center_x;
    }

    if let Some((mut title_text, mut title_tf)) = tooltip_params.p4().iter_mut().next() {
        title_text.0 = display_str;
        title_tf.translation.x = text_start_x;
    }

    // Ekran kenarlarına çarpmama sınır hesabı (ekran pikselleriyle):
    let offset_x = 14.0;
    let offset_y = 16.0;
    let mut screen_left = cursor_pos.x + offset_x;
    let mut screen_top = cursor_pos.y + offset_y;

    if screen_left + badge_w > win_w - 6.0 {
        screen_left = (cursor_pos.x - offset_x - badge_w).max(6.0);
    }
    if screen_top + badge_h > win_h - 6.0 {
        screen_top = (cursor_pos.y - offset_y - badge_h).max(6.0);
    }

    let screen_center = Vec2::new(screen_left + badge_w * 0.5, screen_top + badge_h * 0.5);
    if let Ok(world_pos) = camera.viewport_to_world_2d(cam_global, screen_center) {
        let mut root_q = tooltip_params.p0();
        if let Some((mut root_tf, mut root_vis)) = root_q.iter_mut().next() {
            root_tf.translation.x = world_pos.x;
            root_tf.translation.y = world_pos.y;
            root_tf.translation.z = 99.0;
            // Z eksenini 1.0 tutuyoruz ki yakınlaşınca bayrak ve siyah kenarlık z-fighting yaşayıp siyahlaşmasın!
            root_tf.scale = Vec3::new(cam_scale, cam_scale, 1.0);
            *root_vis = Visibility::Visible;
        }
    }
}
