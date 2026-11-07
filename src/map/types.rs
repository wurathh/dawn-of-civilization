use crate::state::AppState;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

pub const MAP_WIDTH: u32 = 11600;
pub const MAP_HEIGHT: u32 = 4218;
pub const HALF_W_U32: u32 = 5800;
pub const HALF_W: f32 = 5800.0;
pub const MAP_WIDTH_F32: f32 = 11600.0;

#[derive(Deserialize, Clone, Debug)]
pub struct ProvinceInfo {
    pub id: u32,
    pub name: String,
    pub continent: String,
    pub growth_rate: f32,
    pub city_px: [f32; 2],
    pub bbox: [u32; 4], // [min_x, min_y, max_x, max_y]
    pub pixel_area: u32,
}

#[derive(Deserialize)]
pub struct ProvincesFile {
    pub provinces: Vec<ProvinceInfo>,
}

/// RLE formatındaki eyalet tablosu ve dinamik yazılım tabanlı politik harita doku referansları.
#[derive(Resource)]
pub struct WorldMapData {
    pub width: u32,
    pub height: u32,
    /// Her satır için (başlangıç_x, province_id) çiftleri
    pub row_runs: Vec<Vec<(u16, u16)>>,
    pub provinces: HashMap<u32, ProvinceInfo>,
    pub hovered_province: Option<u32>,
    pub selected_province: Option<u32>,
    pub pol_west_handle: Handle<Image>,
    pub pol_east_handle: Handle<Image>,
}

impl WorldMapData {
    #[inline]
    pub fn get_province_at(&self, px: u32, py: u32) -> u32 {
        if px >= self.width || py >= self.height {
            return 0;
        }
        let runs = &self.row_runs[py as usize];
        let x16 = px as u16;
        match runs.binary_search_by_key(&x16, |&(start_x, _)| start_x) {
            Ok(i) => runs[i].1 as u32,
            Err(i) => {
                if i > 0 {
                    runs[i - 1].1 as u32
                } else {
                    0
                }
            }
        }
    }
}

pub fn is_map_active(state: Res<State<AppState>>) -> bool {
    matches!(state.get(), AppState::ScenarioSelect | AppState::InGame)
}

pub fn load_rle_rows(path: &str, width: u32, height: u32) -> Vec<Vec<(u16, u16)>> {
    let bytes = fs::read(path).expect("assets/map/provinces.rle okunamadı!");
    assert!(&bytes[0..4] == b"PRLE", "Geçersiz RLE başlığı!");

    let mut row_runs: Vec<Vec<(u16, u16)>> = vec![Vec::with_capacity(24); height as usize];
    let mut cur_x: u32 = 0;
    let mut cur_y: u32 = 0;

    let data = &bytes[12..];
    for chunk in data.chunks_exact(4) {
        let pid = u16::from_le_bytes([chunk[0], chunk[1]]);
        let mut rem = u16::from_le_bytes([chunk[2], chunk[3]]) as u32;

        while rem > 0 && cur_y < height {
            let space_in_row = width - cur_x;
            let take = rem.min(space_in_row);
            row_runs[cur_y as usize].push((cur_x as u16, pid));

            cur_x += take;
            rem -= take;
            if cur_x == width {
                cur_x = 0;
                cur_y += 1;
            }
        }
    }
    row_runs
}

/// RLE satırını ülke ID'si (`civ_id`) dizisine açar.
#[inline]
pub fn decode_row_to_civ(runs: &[(u16, u16)], pid_to_civ: &[u16], out: &mut [u16]) {
    let len = runs.len();
    for i in 0..len {
        let start = runs[i].0 as usize;
        let end = if i + 1 < len {
            runs[i + 1].0 as usize
        } else {
            MAP_WIDTH as usize
        };
        let pid = runs[i].1 as usize;
        let cid = if pid < pid_to_civ.len() {
            pid_to_civ[pid]
        } else {
            0
        };
        out[start..end].fill(cid);
    }
}

/// Dünya koordinatını (X ekseninde sonsuz döngü `rem_euclid` ile) harita pikseline çevirir!
pub fn world_to_map_pixel(world_pos: Vec2) -> Option<(u32, u32)> {
    let half_w = MAP_WIDTH_F32 * 0.5;
    let half_h = MAP_HEIGHT as f32 * 0.5;

    let py_f = half_h - world_pos.y;
    if py_f < 0.0 || py_f >= MAP_HEIGHT as f32 {
        return None;
    }

    // Yatayda sonsuz döngü (modulo MAP_WIDTH)
    let px_f = (world_pos.x + half_w).rem_euclid(MAP_WIDTH_F32);
    let px = (px_f as u32).min(MAP_WIDTH - 1);
    let py = (py_f as u32).min(MAP_HEIGHT - 1);
    Some((px, py))
}

pub fn map_pixel_to_world(px: f32, py: f32) -> Vec2 {
    let half_w = MAP_WIDTH_F32 * 0.5;
    let half_h = MAP_HEIGHT as f32 * 0.5;
    Vec2::new(px - half_w, half_h - py)
}

/// Ülkenin anakara sınırlarına ve başkentine göre kamera hedef pozisyonunu (X, Y)
/// ve ülkenin coğrafi büyüklüğüne göre dinamik Z (yakınlık/zoom) ölçeğini hesaplar.
pub fn calculate_civ_camera_target(
    civ: &crate::scenario::CivilizationData,
    map_data: &WorldMapData,
) -> (Vec2, f32) {
    if civ.provinces.is_empty() {
        return (Vec2::ZERO, 0.65);
    }

    let cap_info = map_data.provinces.get(&civ.capital_province_id);
    let cap_coords = cap_info.map(|p| p.city_px);
    let cap_continent = cap_info.map(|p| p.continent.as_str()).unwrap_or("");

    let is_colonial = matches!(
        civ.tag.as_str(),
        "fra" | "gbr" | "uni_m" | "nld" | "den_rel3_n" | "esp" | "prt" | "por" | "usa" | "chl" | "nez_m"
    );

    let mut relevant = Vec::new();
    for &pid in &civ.provinces {
        if let Some(p) = map_data.provinces.get(&pid) {
            // Sömürge veya denizaşırı toprakların kıta filtrelemesi
            if is_colonial && !cap_continent.is_empty() && p.continent != cap_continent {
                continue;
            }
            if civ.tag == "usa" && p.city_px[0] < 1600.0 {
                // ABD için uzak kutup eyaletlerini (Alaska) anakaradan ayır
                continue;
            }
            if let Some(cap) = cap_coords {
                let dx = (p.city_px[0] - cap[0]).abs();
                if dx > HALF_W {
                    continue;
                }
                if is_colonial && dx > 3000.0 {
                    continue;
                }
            }
            relevant.push(p);
        }
    }

    if relevant.is_empty() {
        for &pid in &civ.provinces {
            if let Some(p) = map_data.provinces.get(&pid) {
                relevant.push(p);
            }
        }
    }

    let mut min_x = u32::MAX;
    let mut min_y = u32::MAX;
    let mut max_x = 0u32;
    let mut max_y = 0u32;

    for p in &relevant {
        min_x = min_x.min(p.bbox[0]);
        min_y = min_y.min(p.bbox[1]);
        max_x = max_x.max(p.bbox[2]);
        max_y = max_y.max(p.bbox[3]);
    }

    // Ülkenin sadece başkentine değil, tüm anakara sınırlarının ve en uç noktalarının
    // (bounding box) coğrafi merkezine odaklan:
    let center_px = if min_x <= max_x && min_y <= max_y {
        [(min_x + max_x) as f32 * 0.5, (min_y + max_y) as f32 * 0.5]
    } else if let Some(cap) = cap_coords {
        cap
    } else {
        [HALF_W, MAP_HEIGHT as f32 * 0.5]
    };

    let orig_w = if max_x >= min_x { max_x - min_x + 1 } else { 100 };
    let orig_h = if max_y >= min_y { max_y - min_y + 1 } else { 100 };
    let span_x = (orig_w as f32).max(60.0);
    let span_y = (orig_h as f32).max(60.0);

    // Ülkenin ekranda boğulmadan, rahat ve geniş bir nefes payıyla (%55-%65 kadraj)
    // görünmesi için optimize edilmiş zoom ölçeği:
    let scale_by_w = span_x / 880.0;
    let scale_by_h = span_y / 500.0;
    let target_scale = (scale_by_w.max(scale_by_h) * 1.05).clamp(0.42, 2.80);

    // Sol panel (440px) açıkken haritanın açık alanının ortasına oturması için dengeli ofset:
    let center_world = map_pixel_to_world(center_px[0], center_px[1]);
    let target_pos = Vec2::new(
        center_world.x - 140.0 * target_scale,
        center_world.y,
    );

    (target_pos, target_scale)
}

pub fn is_cursor_over_ui(
    cursor_opt: Option<Vec2>,
    win_w: f32,
    win_h: f32,
    state: AppState,
    has_selected_province: bool,
) -> bool {
    let Some(cp) = cursor_opt else {
        return false;
    };
    match state {
        AppState::MainMenu | AppState::ScenarioCatalog | AppState::Settings => true,
        AppState::ScenarioSelect => {
            // Sol tarafta ülke bilgi kartı (genişlik 440px)
            // Alt barda önerilen ülkeler ve butonlar (y > win_h - 100.0)
            // Sağ üstte ayarlar butonu (x > win_w - 220.0 && y < 75.0)
            // Üst ortada senaryo değiştirme paneli (genişlik 440px, yükseklik 100px)
            let half_w = win_w * 0.5;
            let over_top_panel = cp.y < 100.0 && cp.x >= half_w - 220.0 && cp.x <= half_w + 220.0;
            cp.x < 440.0 || cp.y > win_h - 100.0 || (cp.x > win_w - 220.0 && cp.y < 75.0) || over_top_panel
        }
        AppState::InGame => {
            let half_w = win_w * 0.5;
            let over_top_ribbon = cp.x < 780.0 && cp.y < 68.0;
            let over_left_vbar = cp.x < 68.0 && cp.y < 430.0;
            let over_top_right_btn = cp.x > win_w - 170.0 && cp.y < 60.0;
            let over_bottom_speed =
                cp.y > win_h - 85.0 && cp.x >= half_w - 155.0 && cp.x <= half_w + 155.0;
            let over_prov_panel =
                has_selected_province && cp.x < 575.0 && cp.y > win_h - 290.0;
            over_top_ribbon
                || over_left_vbar
                || over_top_right_btn
                || over_bottom_speed
                || over_prov_panel
        }
    }
}
