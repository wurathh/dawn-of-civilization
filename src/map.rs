use crate::scenario::{format_number, ActiveScenario};
use crate::state::{AppState, GameSession, UiFonts};
use bevy::asset::RenderAssetUsages;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::picking::prelude::*;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

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
struct ProvincesFile {
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

#[derive(Component)]
pub struct InnerBorderLayer;

#[derive(Component)]
pub struct PoliticalMapLayer;

#[derive(Component)]
pub struct HoveredProvinceOverlay;

#[derive(Component)]
pub struct SelectedProvinceOverlay;

#[derive(Component)]
pub struct InGameHudRoot;

#[derive(Component)]
pub struct HudPlayerTitleText;

#[derive(Component)]
pub struct HudPlayerColorBadge;

#[derive(Component)]
pub struct HudPlayerFlagNode;

#[derive(Component)]
pub struct HudHoverText;

#[derive(Component)]
pub struct ProvincePanelRoot;

#[derive(Component)]
pub struct ProvincePanelTitle;

#[derive(Component)]
pub struct ProvincePanelDetails;

#[derive(Component)]
pub struct BackToMenuButton;

#[derive(Component)]
pub struct CountryMapLabel {
    pub civ_id: u32,
    pub min_visible_scale: f32,
}

#[derive(Component)]
pub struct CapitalCityBadge {
    pub min_visible_scale: f32,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let scenario = ActiveScenario::load_from_file("assets/scenarios/modernworld/scenario.json");
        app.insert_resource(scenario)
            .insert_resource(CameraRig {
                target_pos: Vec2::new(850.0, 560.0),
                target_scale: 0.85,
            })
            .init_resource::<MapDragState>()
            .add_systems(Startup, setup_world_map)
            .add_systems(OnEnter(AppState::InGame), setup_ingame_hud)
            .add_systems(OnExit(AppState::InGame), cleanup_ingame_hud)
            .add_systems(
                Update,
                (
                    camera_controller.run_if(is_map_active),
                    update_visual_lod,
                    province_interaction.run_if(is_map_active),
                    update_province_overlays,
                    update_country_labels_visibility.run_if(is_map_active),
                )
                    .chain(),
            );
    }
}

fn is_map_active(state: Res<State<AppState>>) -> bool {
    matches!(state.get(), AppState::ScenarioSelect | AppState::InGame)
}

fn load_rle_rows(path: &str, width: u32, height: u32) -> Vec<Vec<(u16, u16)>> {
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
fn decode_row_to_civ(runs: &[(u16, u16)], pid_to_civ: &[u16], out: &mut [u16]) {
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

/// Yazılım üzerinden (CPU'da dinamik olarak) belirtilen dikdörtgen bölgenin politik ülke renklerini
/// ve ülke dış sınırlarını `west_buf` ve `east_buf` RGBA tamponlarına işler.
pub fn render_political_rect_to_buffers(
    row_runs: &[Vec<(u16, u16)>],
    scenario: &ActiveScenario,
    west_buf: &mut [u8],
    east_buf: &mut [u8],
    min_x: u32,
    min_y: u32,
    max_x: u32,
    max_y: u32,
    clear_unowned: bool,
) {
    let min_x = min_x.saturating_sub(2);
    let min_y = min_y.saturating_sub(2);
    let max_x = (max_x + 2).min(MAP_WIDTH - 1);
    let max_y = (max_y + 2).min(MAP_HEIGHT - 1);

    let max_pid = 7000usize;
    let mut pid_to_civ = vec![0u16; max_pid];
    let mut max_cid = 0u32;
    for (&pid, &cid) in &scenario.province_owners {
        if (pid as usize) < max_pid {
            pid_to_civ[pid as usize] = cid as u16;
            if cid > max_cid {
                max_cid = cid;
            }
        }
    }

    let table_len = (max_cid as usize) + 2;
    let mut civ_fill = vec![[0u8; 4]; table_len];
    let mut civ_halo = vec![[0u8; 4]; table_len];
    let mut civ_edge = vec![[0u8; 4]; table_len];

    for (&cid, civ) in &scenario.civs {
        let c = cid as usize;
        if c < table_len {
            let [r, g, b] = civ.color;
            // İç dolgu: alttaki arazi dokusunu (dağlar, çöller) gösteren yarı saydam renk
            civ_fill[c] = [r, g, b, 92];
            // Sınır iç halesi (2. piksel): daha doygun geçiş
            civ_halo[c] = [
                ((r as u16 * 76) / 100) as u8,
                ((g as u16 * 76) / 100) as u8,
                ((b as u16 * 76) / 100) as u8,
                160,
            ];
            // Ülke dış sınırı (1. piksel): belirgin koyu sınır çizgisi
            civ_edge[c] = [
                ((r as u16 * 40) / 100) as u8,
                ((g as u16 * 40) / 100) as u8,
                ((b as u16 * 40) / 100) as u8,
                235,
            ];
        }
    }

    // 5 satırlık kayan pencere (sliding ring buffer): y-2, y-1, y, y+1, y+2
    let mut ring = vec![vec![0u16; MAP_WIDTH as usize]; 5];
    let mut ring_y = [u32::MAX; 5];

    for y in min_y..=max_y {
        let ym2 = y.saturating_sub(2);
        let ym1 = y.saturating_sub(1);
        let yp1 = (y + 1).min(MAP_HEIGHT - 1);
        let yp2 = (y + 2).min(MAP_HEIGHT - 1);

        for &ry in &[ym2, ym1, y, yp1, yp2] {
            let slot = (ry as usize) % 5;
            if ring_y[slot] != ry {
                decode_row_to_civ(&row_runs[ry as usize], &pid_to_civ, &mut ring[slot]);
                ring_y[slot] = ry;
            }
        }

        let s_m2 = (ym2 as usize) % 5;
        let s_m1 = (ym1 as usize) % 5;
        let s_cur = (y as usize) % 5;
        let s_p1 = (yp1 as usize) % 5;
        let s_p2 = (yp2 as usize) % 5;

        let r_m2 = &ring[s_m2];
        let r_m1 = &ring[s_m1];
        let r_cur = &ring[s_cur];
        let r_p1 = &ring[s_p1];
        let r_p2 = &ring[s_p2];

        let row_offset_half = (y * HALF_W_U32) as usize * 4;
        let runs = &row_runs[y as usize];
        let r_len = runs.len();

        for i in 0..r_len {
            let run_start = runs[i].0 as u32;
            if run_start > max_x {
                break;
            }
            let run_end = if i + 1 < r_len {
                (runs[i + 1].0 as u32).saturating_sub(1)
            } else {
                MAP_WIDTH - 1
            };
            if run_end < min_x {
                continue;
            }

            let pid = runs[i].1 as usize;
            let cid = if pid < max_pid { pid_to_civ[pid] } else { 0 };

            let x0 = run_start.max(min_x);
            let x1 = run_end.min(max_x);

            if cid == 0 {
                if clear_unowned {
                    for x in x0..=x1 {
                        if x < HALF_W_U32 {
                            let idx = row_offset_half + (x as usize) * 4;
                            west_buf[idx..idx + 4].copy_from_slice(&[0, 0, 0, 0]);
                        } else {
                            let idx = row_offset_half + ((x - HALF_W_U32) as usize) * 4;
                            east_buf[idx..idx + 4].copy_from_slice(&[0, 0, 0, 0]);
                        }
                    }
                }
                continue;
            }

            let fill = civ_fill[cid as usize];
            let halo = civ_halo[cid as usize];
            let edge = civ_edge[cid as usize];

            for x in x0..=x1 {
                let xu = x as usize;
                let xm1 = xu.saturating_sub(1);
                let xp1 = (xu + 1).min(MAP_WIDTH as usize - 1);

                let rgba =
                    if r_cur[xm1] != cid || r_cur[xp1] != cid || r_m1[xu] != cid || r_p1[xu] != cid
                    {
                        edge
                    } else {
                        let xm2 = xu.saturating_sub(2);
                        let xp2 = (xu + 2).min(MAP_WIDTH as usize - 1);
                        if r_cur[xm2] != cid
                            || r_cur[xp2] != cid
                            || r_m2[xu] != cid
                            || r_p2[xu] != cid
                        {
                            halo
                        } else {
                            fill
                        }
                    };

                if x < HALF_W_U32 {
                    let idx = row_offset_half + xu * 4;
                    west_buf[idx..idx + 4].copy_from_slice(&rgba);
                } else {
                    let idx = row_offset_half + (xu - HALF_W_U32 as usize) * 4;
                    east_buf[idx..idx + 4].copy_from_slice(&rgba);
                }
            }
        }
    }
}

/// Tek bir eyaletin (veya fethedilen şehrin) politik harita dokusunu anında günceller.
pub fn recolor_province_on_map(
    map_data: &WorldMapData,
    scenario: &ActiveScenario,
    images: &mut Assets<Image>,
    province_id: u32,
) {
    let Some(prov) = map_data.provinces.get(&province_id) else {
        return;
    };
    let [min_x, min_y, max_x, max_y] = prov.bbox;

    let mut west_buf = images
        .get_mut(&map_data.pol_west_handle)
        .and_then(|mut img| img.data.take())
        .unwrap_or_default();
    let mut east_buf = images
        .get_mut(&map_data.pol_east_handle)
        .and_then(|mut img| img.data.take())
        .unwrap_or_default();

    if !west_buf.is_empty() && !east_buf.is_empty() {
        render_political_rect_to_buffers(
            &map_data.row_runs,
            scenario,
            &mut west_buf,
            &mut east_buf,
            min_x,
            min_y,
            max_x,
            max_y,
            true,
        );
    }

    if let Some(mut img) = images.get_mut(&map_data.pol_west_handle) {
        img.data = Some(west_buf);
    }
    if let Some(mut img) = images.get_mut(&map_data.pol_east_handle) {
        img.data = Some(east_buf);
    }
}

/// Bir ülkenin tüm eyaletlerini yazılım üzerinden anında yeniden renklendirir.
pub fn recolor_country_on_map(
    map_data: &WorldMapData,
    scenario: &ActiveScenario,
    images: &mut Assets<Image>,
    civ_id: u32,
) {
    let Some(civ) = scenario.civs.get(&civ_id) else {
        return;
    };
    if civ.provinces.is_empty() {
        return;
    }

    let mut west_buf = images
        .get_mut(&map_data.pol_west_handle)
        .and_then(|mut img| img.data.take())
        .unwrap_or_default();
    let mut east_buf = images
        .get_mut(&map_data.pol_east_handle)
        .and_then(|mut img| img.data.take())
        .unwrap_or_default();

    if !west_buf.is_empty() && !east_buf.is_empty() {
        // Batı ve Doğu yarıları için birleşik kutuları hesapla (okyanus aşırı ülkelerde hızlı güncelleme için)
        let mut west_bbox: Option<[u32; 4]> = None;
        let mut east_bbox: Option<[u32; 4]> = None;

        for &pid in &civ.provinces {
            if let Some(p) = map_data.provinces.get(&pid) {
                let [x0, y0, x1, y1] = p.bbox;
                if x0 < HALF_W_U32 {
                    let b = west_bbox.get_or_insert([x0, y0, x1.min(HALF_W_U32 - 1), y1]);
                    b[0] = b[0].min(x0);
                    b[1] = b[1].min(y0);
                    b[2] = b[2].max(x1.min(HALF_W_U32 - 1));
                    b[3] = b[3].max(y1);
                }
                if x1 >= HALF_W_U32 {
                    let b = east_bbox.get_or_insert([x0.max(HALF_W_U32), y0, x1, y1]);
                    b[0] = b[0].min(x0.max(HALF_W_U32));
                    b[1] = b[1].min(y0);
                    b[2] = b[2].max(x1);
                    b[3] = b[3].max(y1);
                }
            }
        }

        if let Some([x0, y0, x1, y1]) = west_bbox {
            render_political_rect_to_buffers(
                &map_data.row_runs,
                scenario,
                &mut west_buf,
                &mut east_buf,
                x0,
                y0,
                x1,
                y1,
                false,
            );
        }
        if let Some([x0, y0, x1, y1]) = east_bbox {
            render_political_rect_to_buffers(
                &map_data.row_runs,
                scenario,
                &mut west_buf,
                &mut east_buf,
                x0,
                y0,
                x1,
                y1,
                false,
            );
        }
    }

    if let Some(mut img) = images.get_mut(&map_data.pol_west_handle) {
        img.data = Some(west_buf);
    }
    if let Some(mut img) = images.get_mut(&map_data.pol_east_handle) {
        img.data = Some(east_buf);
    }
}

/// Rastgele canlı ama strateji haritasına uygun bir ülke rengi üretir.
pub fn generate_random_civ_color(seed_mix: u32) -> [u8; 3] {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() ^ (d.as_secs() as u32).wrapping_mul(2654435761))
        .unwrap_or(123456789)
        .wrapping_add(seed_mix.wrapping_mul(747796405));

    let r = (45 + (nanos & 0xFF) % 185) as u8;
    let g = (45 + ((nanos >> 8) & 0xFF) % 185) as u8;
    let b = (45 + ((nanos >> 16) & 0xFF) % 185) as u8;
    [r, g, b]
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
        AppState::MainMenu | AppState::ScenarioCatalog => true,
        AppState::ScenarioSelect => {
            // Sol tarafta ülke bilgi kartı (genişlik 440px)
            // Alt barda önerilen ülkeler ve butonlar (y > win_h - 100.0)
            // Sağ üstte ayarlar butonu (x > win_w - 220.0 && y < 75.0)
            cp.x < 440.0 || cp.y > win_h - 100.0 || (cp.x > win_w - 220.0 && cp.y < 75.0)
        }
        AppState::InGame => {
            cp.y < 64.0 || (has_selected_province && cp.x < 520.0 && cp.y > win_h - 235.0)
        }
    }
}

fn setup_world_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scenario: Res<ActiveScenario>,
    mut images: ResMut<Assets<Image>>,
    fonts: Res<UiFonts>,
) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(850.0, 560.0, 0.0).with_scale(Vec3::splat(0.85)),
    ));

    // 1. Eyalet / Şehir JSON verisini oku
    let json_raw = fs::read_to_string("assets/map/provinces.json")
        .expect("assets/map/provinces.json okunamadı!");
    let parsed: ProvincesFile =
        serde_json::from_str(&json_raw).expect("provinces.json parse edilemedi!");

    let mut provinces = HashMap::with_capacity(parsed.provinces.len());
    for prov in parsed.provinces {
        provinces.insert(prov.id, prov);
    }

    // 2. RLE sıkıştırmalı 11600x4218 eyalet tablosunu yükle
    let row_runs = load_rle_rows("assets/map/provinces.rle", MAP_WIDTH, MAP_HEIGHT);

    // 3. Politik harita katmanını statik PNG yerine doğrudan yazılım üzerinden dinamik oluştur!
    let half_bytes = (HALF_W_U32 * MAP_HEIGHT * 4) as usize;
    let mut west_buf = vec![0u8; half_bytes];
    let mut east_buf = vec![0u8; half_bytes];
    render_political_rect_to_buffers(
        &row_runs,
        &scenario,
        &mut west_buf,
        &mut east_buf,
        0,
        0,
        MAP_WIDTH - 1,
        MAP_HEIGHT - 1,
        false,
    );

    let pol_extent = Extent3d {
        width: HALF_W_U32,
        height: MAP_HEIGHT,
        depth_or_array_layers: 1,
    };
    let tex_pol_west = images.add(Image::new(
        pol_extent,
        TextureDimension::D2,
        west_buf,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    ));
    let tex_pol_east = images.add(Image::new(
        pol_extent,
        TextureDimension::D2,
        east_buf,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    ));

    // Doku Handle'larını bir kez al, 3 yatay döngü kopyasında (-1, 0, +1) ortak kullan (0 ekstra VRAM maliyeti!)
    let tex_world_west: Handle<Image> = asset_server.load("map/world_west.png");
    let tex_world_east: Handle<Image> = asset_server.load("map/world_east.png");
    let tex_bord_west: Handle<Image> = asset_server.load("map/borders_west.png");
    let tex_bord_east: Handle<Image> = asset_server.load("map/borders_east.png");

    let empty_img = Image::new(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    let hover_tex_handle = images.add(empty_img.clone());
    let select_tex_handle = images.add(empty_img);

    // Yatayda sonsuz dünya (Infinite Horizontal Wrap) için -1 (Sol), 0 (Merkez), +1 (Sağ) kopyalarını oluştur
    for wrap_idx in [-1i32, 0, 1] {
        let wrap_offset_x = wrap_idx as f32 * MAP_WIDTH_F32;

        // Katman 0 (Z = 0.0): Saf, sınırsız, kabartmalı arazi haritası
        for (handle, local_x) in [
            (tex_world_west.clone(), -HALF_W * 0.5),
            (tex_world_east.clone(), HALF_W * 0.5),
        ] {
            commands.spawn((
                Sprite::from_image(handle),
                Transform::from_xyz(local_x + wrap_offset_x, 0.0, 0.0),
                WrapCopy(wrap_idx),
            ));
        }

        // Katman 0.5 (Z = 0.5): Yazılım Üzerinden Dinamik Üretilen Politik Renk & Ülke Dış Sınırları Katmanı
        for (handle, local_x) in [
            (tex_pol_west.clone(), -HALF_W * 0.5),
            (tex_pol_east.clone(), HALF_W * 0.5),
        ] {
            commands.spawn((
                Sprite::from_image(handle),
                Transform::from_xyz(local_x + wrap_offset_x, 0.0, 0.5),
                PoliticalMapLayer,
                WrapCopy(wrap_idx),
            ));
        }

        // Katman 1.0 (Z = 1.0): Şehir/Eyalet iç sınırları katmanı (Politik katmanın üstünde net görünür)
        for (handle, local_x) in [
            (tex_bord_west.clone(), -HALF_W * 0.5),
            (tex_bord_east.clone(), HALF_W * 0.5),
        ] {
            commands.spawn((
                Sprite::from_image(handle),
                Transform::from_xyz(local_x + wrap_offset_x, 0.0, 1.0),
                InnerBorderLayer,
                WrapCopy(wrap_idx),
            ));
        }

        // Katman 2.0: Üzerine gelinen (Hover) eyalet katmanı
        commands.spawn((
            Sprite::from_image(hover_tex_handle.clone()),
            Transform::from_xyz(wrap_offset_x, 0.0, 2.0),
            Visibility::Hidden,
            HoveredProvinceOverlay,
            WrapCopy(wrap_idx),
            BaseMapX(0.0),
        ));

        // Katman 3.0: Seçilen (Selected) eyalet katmanı
        commands.spawn((
            Sprite::from_image(select_tex_handle.clone()),
            Transform::from_xyz(wrap_offset_x, 0.0, 3.0),
            Visibility::Hidden,
            SelectedProvinceOverlay,
            WrapCopy(wrap_idx),
            BaseMapX(0.0),
        ));
    }

    // 4. Harita Üzerindeki Ülke İsimleri ve Başkent Rozetleri (Age of History 3 Stili Dinamik Entegre Etiketler)
    let mut civ_list: Vec<_> = scenario
        .civs
        .values()
        .filter(|c| !c.provinces.is_empty())
        .collect();
    // Büyük ülkeleri önce yerleştir ki küçük/komşu ülkeler onlara göre konumlanabilsin
    civ_list.sort_by(|a, b| {
        b.provinces
            .len()
            .cmp(&a.provinces.len())
            .then_with(|| b.population.cmp(&a.population))
    });

    let mut placed_labels: Vec<(f32, f32)> = Vec::new();
    let mut placed_badge_boxes: Vec<BadgeAabb> = Vec::new();

    for civ in civ_list {

        let cap_info = provinces.get(&civ.capital_province_id);
        let cap_coords = cap_info.map(|p| p.city_px);
        let cap_continent = cap_info.map(|p| p.continent.as_str()).unwrap_or("");

        // Sömürgeci / denizaşırı toprakları olan ülkelerde farklı kıtadaki adaları/sömürgeleri filtrele
        let is_colonial = matches!(
            civ.tag.as_str(),
            "fra" | "gbr" | "nld" | "den_rel3_n" | "esp" | "prt" | "por" | "usa" | "chl"
        );

        let mut mainland_provinces = Vec::new();
        for &pid in &civ.provinces {
            if let Some(p) = provinces.get(&pid) {
                // Fransa'nın Guyana'sı veya Danimarka'nın Grönland'ı anakarayı denize çekmesin
                if is_colonial && !cap_continent.is_empty() && p.continent != cap_continent {
                    continue;
                }

                let (dist, dx, dy) = if let Some(cap) = cap_coords {
                    (
                        ((p.city_px[0] - cap[0]).powi(2) + (p.city_px[1] - cap[1]).powi(2)).sqrt(),
                        (p.city_px[0] - cap[0]).abs(),
                        (p.city_px[1] - cap[1]).abs(),
                    )
                } else {
                    (0.0, 0.0, 0.0)
                };

                // Şili'nin Paskalya adası (Pasifik'te binlerce km uzakta) anakarayı bozmasın
                if civ.tag == "chl" && dx > 350.0 {
                    continue;
                }
                // Portekiz'in Azor ve Madeira adaları (Atlantik'te) anakarayı bozmasın
                if (civ.tag == "por" || civ.tag == "prt") && (dx > 300.0 || dy > 300.0) {
                    continue;
                }

                // Rusya dışındaki ülkelerde başkentten aşırı uzak (1800 px+) adaları hariç tut
                if civ.tag != "rus" && dist > 1800.0 {
                    continue;
                }

                mainland_provinces.push(p);
            }
        }

        if mainland_provinces.is_empty() {
            if let Some(c) = cap_info {
                mainland_provinces.push(c);
            } else if let Some(first_p) = civ.provinces.first().and_then(|id| provinces.get(id)) {
                mainland_provinces.push(first_p);
            } else {
                continue;
            }
        }

        // Anakara ağırlık merkezi ve bounding box hesabı
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut total_w = 0.0;
        let mut min_x = u32::MAX;
        let mut max_x = 0u32;
        let mut min_y = u32::MAX;
        let mut max_y = 0u32;

        for p in &mainland_provinces {
            let w = (p.pixel_area as f32).max(15.0);
            sum_x += p.city_px[0] * w;
            sum_y += p.city_px[1] * w;
            total_w += w;

            min_x = min_x.min(p.bbox[0]);
            min_y = min_y.min(p.bbox[1]);
            max_x = max_x.max(p.bbox[2]);
            max_y = max_y.max(p.bbox[3]);
        }

        let center_px = sum_x / total_w;
        let center_py = sum_y / total_w;

        let span_w = (max_x.saturating_sub(min_x) as f32).max(35.0);
        let span_h = (max_y.saturating_sub(min_y) as f32).max(25.0);

        // Bir ülkenin dikey şerit (Kuzey-Güney) olup olmadığını belirle
        let is_vertical = civ.tag == "chl"
            || civ.tag == "por"
            || civ.tag == "prt"
            || civ.tag == "nor_rel3_n"
            || civ.tag == "vie_rel10_c"
            || (span_h > span_w * 1.35 && span_w < 250.0);

        // Ülke anakara çekirdek yarıçapları (standart sapma / kovaryans özdeğerleri):
        let mut cov_xy = 0.0;
        let mut var_x = 0.0;
        let mut var_y = 0.0;
        for p in &mainland_provinces {
            let dx = p.city_px[0] - center_px;
            let dy = p.city_px[1] - center_py;
            let w = (p.pixel_area as f32).max(15.0);
            cov_xy += dx * dy * w;
            var_x += dx * dx * w;
            var_y += dy * dy * w;
        }
        let norm_w = total_w.max(1.0);
        let c_cov_xy = cov_xy / norm_w;
        let c_var_x = var_x / norm_w;
        let c_var_y = var_y / norm_w;

        let diff = c_var_x - c_var_y;
        let disc = (diff * diff + 4.0 * c_cov_xy * c_cov_xy).sqrt();
        let lambda1 = ((c_var_x + c_var_y + disc) * 0.5).max(0.0);
        let lambda2 = ((c_var_x + c_var_y - disc) * 0.5).max(0.0);
        let sigma_major = lambda1.sqrt();
        let sigma_minor = lambda2.sqrt();

        // Açı (Rotasyon) ve Coğrafi Merkez Ayarı:
        let (adj_center_px, adj_center_py, angle) = if civ.tag == "zam" {
            (6680.0, 3365.0, -0.38) // Zambiya: Kongo girintisinin altından eğimli (-22°)
        } else if civ.tag == "mai" {
            (5720.0, 2240.0, 0.35) // Mali: Moritanya sınırından uzak, geniş Sahra/Timbuktu merkezine (+20°)
        } else if civ.tag == "wes_rel0_r" {
            (5365.0, 2060.0, 0.60) // Batı Sahra: Kıyı ve çöl şeridi boyunca doğal diagonal akış (+34.4°)
        } else if civ.tag == "chl" {
            (center_px, center_py, 1.45) // Şili: And Dağları kıyısı boyunca DİKEY (+83°)
        } else if civ.tag == "por" || civ.tag == "prt" {
            (center_px, center_py, 1.40) // Portekiz: Lizbon-Porto kıyısı boyunca DİKEY (+80°)
        } else if civ.tag == "nor_rel3_n" {
            (center_px, center_py, 0.75) // Norveç: Fiyortlar boyunca eğik (+43°)
        } else if civ.tag == "vie_rel10_c" {
            (center_px, center_py, 1.35) // Vietnam: Kıyı boyunca DİKEY (+77°)
        } else if civ.tag == "ita" {
            (center_px, center_py, -0.52) // İtalya: Çizme eğimi boyunca (-30°)
        } else if civ.tag == "gre" {
            (center_px, center_py, -0.42) // Yunanistan: (-24°)
        } else if civ.tag == "egy" {
            (center_px, center_py, -0.35) // Mısır: (-20°)
        } else if is_vertical {
            (center_px, center_py, 1.40) // Diğer dikey ülkeler: DİKEY (+80°)
        } else {
            // Doğal kartografik eğiklik (-22° .. +22°)
            let raw_ang = if (var_x - var_y).abs() > 20.0 || cov_xy.abs() > 20.0 {
                0.5 * (-2.0 * cov_xy).atan2(var_x - var_y)
            } else {
                0.0
            };
            let clamped = raw_ang.clamp(-0.38, 0.38);
            (center_px, center_py, clamped)
        };

        let world_pos = map_pixel_to_world(adj_center_px, adj_center_py);

        // Dikey ve boyuna omurgadaki gerçek uzunluk ve kalınlığı hesapla
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let mut u_min = f32::MAX;
        let mut u_max = f32::MIN;
        let mut v_min = f32::MAX;
        let mut v_max = f32::MIN;

        for p in &mainland_provinces {
            let corners = [
                [p.bbox[0] as f32, p.bbox[1] as f32],
                [p.bbox[2] as f32, p.bbox[1] as f32],
                [p.bbox[0] as f32, p.bbox[3] as f32],
                [p.bbox[2] as f32, p.bbox[3] as f32],
            ];
            for &[cx, cy] in &corners {
                let dx = cx - adj_center_px;
                let dy = -(cy - adj_center_py);
                let u = dx * cos_a + dy * sin_a;
                let v = -dx * sin_a + dy * cos_a;
                u_min = u_min.min(u);
                u_max = u_max.max(u);
                v_min = v_min.min(v);
                v_max = v_max.max(v);
            }
        }
        let spine_len = (u_max - u_min).max(30.0);
        let spine_thick = (v_max - v_min).max(20.0);

        // Dikey ülkeler tek satırda boyuna uzanır; diğerleri kompakt formatlanır
        let label_text = if is_vertical {
            civ.name
                .chars()
                .map(|c| match c {
                    'i' => 'İ',
                    'ı' => 'I',
                    _ => c.to_uppercase().next().unwrap_or(c),
                })
                .collect()
        } else {
            format_country_map_name(&civ.name)
        };

        let lines: Vec<&str> = label_text.lines().collect();
        let line_count = lines.len().max(1) as f32;
        let max_line_chars = lines
            .iter()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(3)
            .max(2) as f32;

        // Taşmayı KESİNLİKLE engellemek için güvenli çekirdek alanı:
        // Hem omurga sınırları hem de kütle çekirdeği (2.15 * sigma) kontrol edilir.
        // Geniş alana sahip ülkelerde (Rusya, Kanada, Avustralya, Türkiye, Mısır vb.)
        // yazıların ferah ve okunaklı şekilde büyümesine imkan tanınır.
        // Eyalet sayısı çok az olan ülkelerde (örn: Batı Sahra) 1D çizgi dejenerasyonu önlenir.
        let n_provs = mainland_provinces.len();
        let safe_len = (spine_len * 0.60)
            .min(if n_provs <= 6 { spine_len * 0.55 } else { sigma_major * 2.15 })
            .max(24.0);
        let safe_thick = (spine_thick * 0.54)
            .min(if n_provs <= 6 { spine_thick * 0.40 } else { sigma_minor * 1.75 })
            .max(18.0);

        // Font boyutu hem uzunluk hem kalınlık ile kesin sınırlandırılır:
        let font_by_len = safe_len / (max_line_chars * 0.62);
        let font_by_thick = safe_thick / (line_count * 1.16);
        let mut font_size = font_by_len.min(font_by_thick);

        // Dinamik alan tavanı: Ülke yüzölçümüne orantılı limit.
        // Yer varsa büyük ülkelerin isimleri heybetli ve kartografik olarak büyüyecek,
        // dar yerlerde ise safe_len / safe_thick sınırları sayesinde asla taşmayacaktır.
        let area_cap = (total_w.sqrt() * 0.140).clamp(8.0, 145.0);
        font_size = font_size.clamp(4.5, area_cap);

        // Kavis (Eğimli / Dairesel Yay) Hesabı:
        let mut kappa: f32 = match civ.tag.as_str() {
            "wes_rel0_r" => 0.0,    // Batı Sahra: Düz hat boyunca doğal diagonal akış
            "rus" => 0.000045,      // Rusya: Sibirya boyunca yay
            "can" => 0.000095,      // Kanada: ABD sınırı üzerinden kavis
            "usa" => 0.000050,      // ABD: Kıtalararası dairesel yay
            "bra" => 0.000070,      // Brezilya: Amazon-Atlantik yay
            "aus" => 0.000065,      // Avustralya: Kıtalararası kavis
            "tur" => 0.000085,      // Türkiye: Anadolu yarımadası kavis
            "alg" => 0.000075,      // Cezayir: Sahra boyunca doğal kavis
            "kaz" => 0.000070,      // Kazakistan: Bozkır boyunca kavis
            "chn" => 0.000055,      // Çin: Asya anakarası kavis
            "ind" => 0.000065,      // Hindistan: Yarımada kavis
            "egy" => 0.000080,      // Mısır: Nil & çöl ekseninde kavis
            "ita" => 0.000250,      // İtalya: Çizme boyunca kavis
            "vie_rel10_c" => -0.000220, // Vietnam: Kıyı boyunca kavis
            "nor_rel3_n" => 0.000160,   // Norveç: Fiyortlar boyunca kavis
            "chl" => 0.000040,      // Şili: Andlar boyu kavis
            "idn" => 0.000050,      // Endonezya: Adalar boyu kavis
            "mex" => 0.000080,      // Meksika: Kavisli duruş
            "arg" => 0.000055,      // Arjantin: Patagonya boyu kavis
            _ => {
                if spine_len > 180.0 {
                    let mut num = 0.0;
                    let mut denom = 0.0;
                    for p in &mainland_provinces {
                        let dx = p.city_px[0] - adj_center_px;
                        let dy = -(p.city_px[1] - adj_center_py);
                        let u = dx * cos_a + dy * sin_a;
                        let v = -dx * sin_a + dy * cos_a;
                        let w = (p.pixel_area as f32).max(10.0);
                        num += w * u * u * v;
                        denom += w * u * u * u * u;
                    }
                    if denom > 1.0 {
                        let k_raw = -num / denom;
                        k_raw.clamp(-0.0003, 0.0003)
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            }
        };

        // Kavisin ülke sınırlarını delip taşmasını engelleyen sagitta sınırlayıcı:
        let half_target_len = safe_len * 0.5;
        let max_allowed_sagitta = safe_thick * 0.12;
        if kappa.abs() * half_target_len * half_target_len > max_allowed_sagitta {
            kappa = kappa.signum() * (max_allowed_sagitta / (half_target_len * half_target_len));
        }
        if kappa.abs() * half_target_len * half_target_len < 1.0 {
            kappa = 0.0;
        }

        // Harflerin yay/eğri boyunca matematiksel yerleşimi
        let mut placed_chars = Vec::new();
        let line_h = font_size * 1.15;
        for (l_idx, line_str) in lines.iter().enumerate() {
            let d_l = ((line_count - 1.0) * 0.5 - l_idx as f32) * line_h;
            let line_chars: Vec<char> = line_str.chars().collect();
            let mut raw_chars_w = 0.0;
            for &c in &line_chars {
                raw_chars_w += char_width_ratio(c) * font_size;
            }
            let gaps = line_chars.len().saturating_sub(1);
            let target_line_w = safe_len.min(raw_chars_w + gaps as f32 * font_size * 0.20);
            let spacing = if gaps > 0 && target_line_w > raw_chars_w {
                ((target_line_w - raw_chars_w) / gaps as f32).min(font_size * 0.20)
            } else {
                0.0
            };
            let actual_line_w = raw_chars_w + gaps as f32 * spacing;
            let mut cur_u = -actual_line_w * 0.5;

            for &c in &line_chars {
                let cw = char_width_ratio(c) * font_size;
                let u_center = cur_u + cw * 0.5;
                if c != ' ' {
                    let v_center = -kappa * u_center * u_center;
                    let slope = -2.0 * kappa * u_center;
                    let norm_len = (1.0 + slope * slope).sqrt();
                    let norm_x = -slope / norm_len;
                    let norm_y = 1.0 / norm_len;
                    let local_rot = slope.atan();
                    let local_x = u_center + d_l * norm_x;
                    let local_y = v_center + d_l * norm_y;

                    placed_chars.push(PlacedCharInfo {
                        text: c.to_string(),
                        local_x,
                        local_y,
                        local_rot,
                    });
                }
                cur_u += cw + spacing;
            }
        }

        // Zoom görünürlük kademesi (LOD): Küçük ülkelerin birbiri üstüne binmesini önleme
        let mut min_visible_scale: f32 = if font_size >= 40.0 {
            100.0 // Dev ülkeler her zaman görünür
        } else if font_size >= 24.0 {
            4.0 // Büyük ve orta ülkeler dünya zoomunda görünür
        } else if font_size >= 16.0 {
            2.2 // Bölgesel zoomda görünür
        } else if font_size >= 12.0 {
            1.6 // Yakınlaşınca belirir
        } else {
            1.35 // Küçük ülkeler sadece harita makul yakınlıktayken görünür
        };

        let mut label_overlap = false;
        for &(lx, ly) in &placed_labels {
            let dist = ((world_pos.x - lx).powi(2) + (world_pos.y - ly).powi(2)).sqrt();
            if dist < 65.0 {
                label_overlap = true;
                break;
            }
        }
        if label_overlap {
            min_visible_scale = min_visible_scale.min(1.40);
        }
        placed_labels.push((world_pos.x, world_pos.y));

        // Büyük harita ülke adı rengi (Cezayir yeşiline mavi ton atma hatası giderildi; zeminle tam uyumlu!)
        let text_color = get_country_label_color(civ.color);

        for wrap_idx in [-1i32, 0, 1] {
            let wrap_offset_x = wrap_idx as f32 * MAP_WIDTH_F32;

            // 1. Uzaktan görünen büyük harita ülke adı (Kavisli / Dairesel, organik duruşlu)
            commands
                .spawn((
                    Transform::from_xyz(world_pos.x + wrap_offset_x, world_pos.y, 4.2)
                        .with_rotation(Quat::from_rotation_z(angle)),
                    Visibility::Inherited,
                    CountryMapLabel {
                        civ_id: civ.id,
                        min_visible_scale,
                    },
                    WrapCopy(wrap_idx),
                    BaseMapX(world_pos.x),
                    Pickable::IGNORE,
                ))
                .with_children(|parent| {
                    for ch_info in &placed_chars {
                        parent.spawn((
                            Text2d::new(ch_info.text.clone()),
                            TextFont {
                                font: fonts.map_country.clone(),
                                font_size: FontSize::Px(font_size),
                                ..default()
                            },
                            TextColor(text_color),
                            Transform::from_xyz(ch_info.local_x, ch_info.local_y, 0.0)
                                .with_rotation(Quat::from_rotation_z(ch_info.local_rot)),
                            Pickable::IGNORE,
                        ));
                    }
                });
        }

        // 2. Başkent Rozeti (Kamera yakınlaşınca başkentin üstünde beliren tek parça bayraklı rozet)
        let cap_pos_opt = cap_coords.or_else(|| mainland_provinces.first().map(|p| p.city_px));

        if let Some(c_px) = cap_pos_opt {
            let c_world = map_pixel_to_world(c_px[0], c_px[1]);

            let badge_title = get_badge_display_name(&civ.name);
            let text_w = get_badge_text_width(&badge_title);
            // Kutu rozet metnine ve bayrağa tam oturur, metin asla dışarı taşmaz veya arkaplan geride kalmaz:
            let badge_w = (30.5 + text_w).max(36.0);
            let badge_h = 16.0;

            // Kesin Çakışma Önleme (AABB Anti-Collision Engine):
            // Başkent rozetleri şehirlerin tam üstünde kalır; denize veya komşu ülkelere ASLA kaymaz!
            let mut candidate_offsets: Vec<(f32, f32)> = Vec::with_capacity(16);

            // Bitişik mikro-komşu başkentler için yerinde mikro ayrışma:
            if civ.tag == "cyn" {
                // KKTC: Kıbrıs adasının kuzey yarısında (+6px yukarı, ada üzerinde kalır, asla Türkiye'ye uzanmaz)
                candidate_offsets.push((0.0, 6.0));
                candidate_offsets.push((4.0, 6.0));
                candidate_offsets.push((-4.0, 6.0));
            } else if civ.tag == "cpr" {
                // Güney Kıbrıs: Kıbrıs adasının güney yarısında (-7px aşağı, ada üzerinde kalır)
                candidate_offsets.push((0.0, -7.0));
                candidate_offsets.push((-4.0, -7.0));
                candidate_offsets.push((4.0, -7.0));
            } else if civ.tag == "jor" {
                // Ürdün: Amman'ın doğu ekseninde (+14px doğu - Ürdün içine yayılır, Kudüs'ü tıkamaz)
                candidate_offsets.push((14.0, 0.0));
                candidate_offsets.push((16.0, 2.0));
                candidate_offsets.push((12.0, -2.0));
                candidate_offsets.push((0.0, 0.0));
            } else if civ.tag == "isr" {
                // İsrail: Tel Aviv - Kudüs iç koridorunda (-14px batı, +10px kuzey - 100% İsrail toprağında, denizde değil!)
                candidate_offsets.push((-14.0, 10.0));
                candidate_offsets.push((-16.0, 12.0));
                candidate_offsets.push((-12.0, 8.0));
                candidate_offsets.push((-10.0, 14.0));
            } else if civ.tag == "pal" {
                // Filistin: Batı Şeria güneyinde (0px x, -14px güney - 100% Batı Şeria içinde, asla Mısır'a taşmaz!)
                candidate_offsets.push((0.0, -14.0));
                candidate_offsets.push((2.0, -16.0));
                candidate_offsets.push((-2.0, -14.0));
                candidate_offsets.push((4.0, -12.0));
            } else if civ.tag == "vat" {
                candidate_offsets.push((-6.0, 6.0));
            } else if civ.tag == "mco" {
                candidate_offsets.push((6.0, -6.0));
            } else if civ.tag == "smr" {
                candidate_offsets.push((0.0, -6.0));
            } else if civ.tag == "cog" {
                candidate_offsets.push((-6.0, 6.0));
            } else if civ.tag == "drc_rel1_r" {
                candidate_offsets.push((6.0, -6.0));
            } else {
                // Tüm kıyı, ada ve normal ülkeler için BİRİNCİ ÖNCELİK:
                // Şehrin tam koordinatı! (Denize veya başka ülkelere kaymayı %100 engeller)
                candidate_offsets.push((0.0, 0.0));
                // Yanında çok yakın başka bir başkent varsa yalnızca küçük mikro adımlarla ayrışma:
                candidate_offsets.push((0.0, 6.0));
                candidate_offsets.push((0.0, -6.0));
                candidate_offsets.push((6.0, 0.0));
                candidate_offsets.push((-6.0, 0.0));
                candidate_offsets.push((6.0, 6.0));
                candidate_offsets.push((-6.0, 6.0));
                candidate_offsets.push((6.0, -6.0));
                candidate_offsets.push((-6.0, -6.0));
                candidate_offsets.push((0.0, 10.0));
                candidate_offsets.push((0.0, -10.0));
            }

            let mut best_ox = candidate_offsets.first().map(|c| c.0).unwrap_or(0.0);
            let mut best_oy = candidate_offsets.first().map(|c| c.1).unwrap_or(0.0);
            let mut found_non_overlapping = false;

            for (ox, oy) in candidate_offsets {
                let cand_min_x = c_world.x + ox - badge_w * 0.5;
                let cand_max_x = c_world.x + ox + badge_w * 0.5;
                let cand_min_y = c_world.y + oy - badge_h * 0.5;
                let cand_max_y = c_world.y + oy + badge_h * 0.5;

                let has_conflict = placed_badge_boxes.iter().any(|bb| {
                    bb.overlaps(cand_min_x, cand_max_x, cand_min_y, cand_max_y, 1.2)
                });

                if !has_conflict {
                    best_ox = ox;
                    best_oy = oy;
                    found_non_overlapping = true;
                    placed_badge_boxes.push(BadgeAabb {
                        min_x: cand_min_x,
                        max_x: cand_max_x,
                        min_y: cand_min_y,
                        max_y: cand_max_y,
                    });
                    break;
                }
            }

            if !found_non_overlapping {
                best_ox = 0.0;
                best_oy = 0.0;
                placed_badge_boxes.push(BadgeAabb {
                    min_x: c_world.x - badge_w * 0.5,
                    max_x: c_world.x + badge_w * 0.5,
                    min_y: c_world.y - badge_h * 0.5,
                    max_y: c_world.y + badge_h * 0.5,
                });
            }

            let badge_base_x = c_world.x + best_ox;
            let badge_y = c_world.y + best_oy;

            let min_badge_scale = if matches!(civ.tag.as_str(), "pal" | "isr" | "cyn" | "cpr" | "vat" | "smr" | "mco") {
                0.85
            } else {
                1.25
            };

            let flag_path = ActiveScenario::get_flag_path(&civ.tag, None);
            let flag_handle = asset_server.load(flag_path);

            let flag_center_x = -badge_w * 0.5 + 12.0;
            let text_start_x = -badge_w * 0.5 + 24.0;

            for wrap_idx in [-1i32, 0, 1] {
                let wrap_offset_x = wrap_idx as f32 * MAP_WIDTH_F32;
                let badge_x = badge_base_x + wrap_offset_x;

                commands
                    .spawn((
                        Transform::from_xyz(badge_x, badge_y, 4.4),
                        Visibility::Hidden,
                        CapitalCityBadge {
                            min_visible_scale: min_badge_scale,
                        },
                        WrapCopy(wrap_idx),
                        BaseMapX(badge_base_x),
                        Pickable::IGNORE,
                    ))
                    .with_children(|parent| {
                        // Koyu yarı saydam kompakt arka plan kutucuğu (Pill) - tam oturan boyut
                        parent.spawn((
                            Sprite::from_color(
                                Color::srgba(0.04, 0.06, 0.10, 0.88),
                                Vec2::new(badge_w, badge_h),
                            ),
                            Transform::from_xyz(0.0, 0.0, 0.0),
                            Pickable::IGNORE,
                        ));

                        // Ülke bayrağı
                        parent.spawn((
                            Sprite {
                                image: flag_handle.clone(),
                                custom_size: Some(Vec2::new(14.0, 9.5)),
                                ..default()
                            },
                            Transform::from_xyz(flag_center_x, 0.0, 0.1),
                            Pickable::IGNORE,
                        ));

                        // Ülke adı metni (BOLD ve okunaklı)
                        parent.spawn((
                            Text2d::new(badge_title.clone()),
                            TextFont {
                                font: fonts.heading.clone(),
                                font_size: FontSize::Px(9.5),
                                ..default()
                            },
                            Anchor::CENTER_LEFT,
                            TextColor(Color::srgb(0.80, 0.79, 0.75)),
                            Transform::from_xyz(text_start_x, 0.0, 0.1),
                            Pickable::IGNORE,
                        ));
                    });
            }
        }
    }

    commands.insert_resource(WorldMapData {
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
        row_runs,
        provinces,
        hovered_province: None,
        selected_province: None,
        pol_west_handle: tex_pol_west,
        pol_east_handle: tex_pol_east,
    });
}

fn format_country_map_name(name: &str) -> String {
    let custom = match name {
        "Amerika Birleşik Devletleri" => Some("AMERİKA\nBİRLEŞİK DEVLETLERİ"),
        "Kuzey Kıbrıs Türk Cumhuriyeti" => Some("KUZEY\nKIBRIS"),
        "Kıbrıs Cumhuriyeti" => Some("KIBRIS"),
        "Birleşik Krallık" => Some("BİRLEŞİK\nKRALLIK"),
        "Makedonya Cumhuriyeti" => Some("MAKEDONYA"),
        "Filistin Devleti" => Some("FİLİSTİN"),
        "İsrail" => Some("İSRAİL"),
        "Demokratik Kongo Cumhuriyeti" | "Kongo Demokratik Cumhuriyeti" => Some("DEMOKRATİK\nKONGO"),
        "Kongo Cumhuriyeti" => Some("KONGO"),
        "Orta Afrika Cumhuriyeti" => Some("ORTA AFRİKA"),
        "Bosna-Hersek" => Some("BOSNA"),
        "Birleşik Arap Emirlikleri" => Some("B.A.E."),
        "Suudi Arabistan" => Some("SUUDİ\nARABİSTAN"),
        "Papua Yeni Gine" => Some("PAPUA\nYENİ GİNE"),
        "Dominik Cumhuriyeti" => Some("DOMİNİK"),
        "Çek Cumhuriyeti" => Some("ÇEKYA"),
        "Güney Afrika Cumhuriyeti" => Some("GÜNEY\nAFRİKA"),
        "Yeni Zelanda" => Some("YENİ\nZELANDA"),
        "Fildişi Sahili" => Some("FİLDİŞİ\nSAHİLİ"),
        "Ekvator Ginesi" => Some("EKVATOR\nGİNESİ"),
        "Kosta Rika" => Some("KOSTA\nRİKA"),
        "Porto Riko" => Some("PORTO\nRİKO"),
        "El Salvador" => Some("EL\nSALVADOR"),
        "Sri Lanka" => Some("SRİ\nLANKA"),
        "Sierra Leone" => Some("SİERRA\nLEONE"),
        "Burkina Faso" => Some("BURKİNA\nFASO"),
        "Trinidad ve Tobago" => Some("TRİNİDAD\nVE TOBAGO"),
        "Afganistan Emirliği" => Some("AFGANİSTAN"),
        "Yeşil Burun Adaları" => Some("YEŞİL\nBURUN"),
        "Sahra Demokratik Arap Cumhuriyeti" => Some("BATI SAHRA"),
        "Güney Georgia ve Güney Sandwich Adaları" => Some("GÜNEY\nGEORGIA"),
        "Mikronezya Federal Devletleri" => Some("MİKRONEZYA"),
        _ => None,
    };

    if let Some(s) = custom {
        return s.to_string();
    }

    let upper: String = name
        .chars()
        .map(|c| match c {
            'i' => 'İ',
            'ı' => 'I',
            _ => c.to_uppercase().next().unwrap_or(c),
        })
        .collect();

    let words: Vec<&str> = upper.split_whitespace().collect();
    if words.len() == 2 {
        format!("{}\n{}", words[0], words[1])
    } else if words.len() >= 3 {
        let mid = words.len() / 2;
        format!("{}\n{}", words[..mid].join(" "), words[mid..].join(" "))
    } else {
        upper
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BadgeAabb {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl BadgeAabb {
    #[inline]
    pub fn overlaps(&self, other_min_x: f32, other_max_x: f32, other_min_y: f32, other_max_y: f32, pad: f32) -> bool {
        self.min_x - pad < other_max_x
            && self.max_x + pad > other_min_x
            && self.min_y - pad < other_max_y
            && self.max_y + pad > other_min_y
    }
}

pub struct PlacedCharInfo {
    pub text: String,
    pub local_x: f32,
    pub local_y: f32,
    pub local_rot: f32,
}

/// Harita yazı fontu (EBGaramond-ExtraBold) için bağıl karakter genişliği
pub fn char_width_ratio(c: char) -> f32 {
    match c {
        'I' | 'İ' | 'l' | '|' | '!' | '\'' | '.' | ':' | ';' => 0.28,
        'J' | 'f' | 'r' | 't' | '(' | ')' | '[' | ']' => 0.40,
        'E' | 'F' | 'L' | 'S' | 'Ş' | 'Z' | 'c' | 's' | 'z' => 0.54,
        'A' | 'B' | 'C' | 'Ç' | 'D' | 'G' | 'Ğ' | 'H' | 'K' | 'N' | 'O' | 'Ö' | 'P' | 'R' | 'T' | 'U' | 'Ü' | 'V' | 'X' | 'Y' => 0.66,
        'M' | 'W' | 'm' | 'w' => 0.86,
        ' ' => 0.42,
        _ => 0.58,
    }
}

/// Başkent rozetleri (Inter-Bold 9.5px) için gerçek piksel metin genişliği hesabı
pub fn get_badge_text_width(text: &str) -> f32 {
    let mut w = 0.0;
    for ch in text.chars() {
        w += match ch {
            '\'' => 2.0,
            ' ' => 2.3,
            'i' | 'ı' | 'l' | 'j' | 'I' | 'İ' => 2.7,
            '.' | ',' | ':' | ';' | '!' => 2.9,
            'f' | 't' | 'r' | '/' => 3.8,
            '(' | ')' | '-' => 4.2,
            '1' => 4.7,
            's' | 'ş' | 'z' | 'x' | 'k' | 'v' | 'y' => 5.5,
            'c' | 'ç' | 'a' | 'e' | 'L' | 'F' => 5.7,
            'o' | 'ö' | 'u' | 'ü' | 'n' | 'h' | 'd' | 'b' | 'p' | 'q' | 'g' | 'ğ' | 'E' | '7' => 6.0,
            '2' | '3' | '5' | '6' | '8' | '9' | '0' | 'B' | 'P' | 'R' | 'S' | 'Ş' | 'T' | 'Z' => 6.3,
            'K' | 'X' | 'Y' | 'H' | 'U' | 'Ü' | 'N' | 'D' => 7.0,
            'A' | 'C' | 'Ç' | 'G' | 'Ğ' | 'O' | 'Ö' | 'Q' | 'V' => 7.3,
            'w' => 8.1,
            'm' | 'M' => 8.7,
            'W' => 9.9,
            _ => 6.2,
        };
    }
    w
}

/// Büyük ülke adı metin rengini belirler:
/// Kullanıcının kesin tercihi: Tüm ülkelerde istisnasız net, klasik ve asil kartografik SİYAH.
pub fn get_country_label_color(_country_rgb: [u8; 3]) -> Color {
    Color::srgba(0.08, 0.08, 0.10, 0.92)
}

fn get_badge_display_name(raw_name: &str) -> String {
    let s = match raw_name {
        "Amerika Birleşik Devletleri" => "ABD",
        "Kuzey Kıbrıs Türk Cumhuriyeti" => "KKTC",
        "Kıbrıs Cumhuriyeti" => "Kıbrıs",
        "Birleşik Krallık" => "İngiltere",
        "Makedonya Cumhuriyeti" => "Makedonya",
        "Filistin Devleti" | "Filistin" => "Filistin",
        "İsrail" => "İsrail",
        "Demokratik Kongo Cumhuriyeti" | "Kongo Demokratik Cumhuriyeti" => "D. Kongo",
        "Kongo Cumhuriyeti" => "Kongo",
        "Orta Afrika Cumhuriyeti" => "Orta Afrika",
        "Bosna-Hersek" => "Bosna",
        "Birleşik Arap Emirlikleri" => "B.A.E.",
        "Suudi Arabistan" => "S. Arabistan",
        "Papua Yeni Gine" => "Papua Y.G.",
        "Dominik Cumhuriyeti" => "Dominik",
        "Çek Cumhuriyeti" => "Çekya",
        "Güney Afrika Cumhuriyeti" => "G. Afrika",
        "Yeni Zelanda" => "Y. Zelanda",
        "Fildişi Sahili" => "Fildişi S.",
        "Ekvator Ginesi" => "Ekvator G.",
        "Trinidad ve Tobago" => "Trinidad",
        "Saint Vincent ve Grenadinler" => "St. Vincent",
        "Antigua ve Barbuda" => "Antigua",
        "Saint Kitts ve Nevis" => "St. Kitts",
        "São Tomé ve Príncipe" => "Sao Tome",
        "Afganistan Emirliği" => "Afganistan",
        "Yeşil Burun Adaları" => "Yeşil Burun",
        "Sahra Demokratik Arap Cumhuriyeti" => "Batı Sahra",
        "Güney Georgia ve Güney Sandwich Adaları" => "G. Georgia",
        "Mikronezya Federal Devletleri" => "Mikronezya",
        "Marshall Adaları" => "Marshall",
        "Falkland Adaları" => "Falkland",
        other => other,
    };
    s.to_string()
}

fn update_country_labels_visibility(
    camera_q: Single<&Transform, With<Camera2d>>,
    mut label_q: Query<(&CountryMapLabel, &mut Visibility), Without<CapitalCityBadge>>,
    mut badge_q: Query<
        (&CapitalCityBadge, &mut Transform, &mut Visibility),
        Without<Camera2d>,
    >,
) {
    let cam_scale = camera_q.scale.x;

    // Yakınlaşınca (cam_scale <= 1.25): Büyük ülke adları kaybolur, başkentin üstünde bayraklı rozetler belirir
    // Uzaklaşınca (cam_scale > 1.25): Büyük ülke adları görünür, rozetler gizlenir
    let show_labels = cam_scale > 1.25;

    for (label, mut vis) in &mut label_q {
        let should_be_visible = show_labels && (cam_scale <= label.min_visible_scale);
        let target_vis = if should_be_visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *vis != target_vis {
            *vis = target_vis;
        }
    }

    // Kamera yakınlaştıkça başkent rozetleri haritada dev gibi kalmaz, orantılı küçülür
    let badge_scale_factor = (cam_scale * 0.70 + 0.30).clamp(0.40, 1.0);

    for (badge, mut tf, mut vis) in &mut badge_q {
        let show_badge = cam_scale <= badge.min_visible_scale;
        let target_vis = if show_badge {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *vis != target_vis {
            *vis = target_vis;
        }

        if show_badge {
            let cur_scale = tf.scale.x;
            if (cur_scale - badge_scale_factor).abs() > 0.005 {
                tf.scale = Vec3::splat(badge_scale_factor);
            }
        }
    }
}

fn camera_controller(
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

    // DİKEY EKSEN: Haritanın üst ve alt sınırlarında durdur
    let half_map_h = MAP_HEIGHT as f32 * 0.5;
    let half_view_h_target = (win_h * 0.5) * rig.target_scale;
    let margin_y = half_view_h_target * 0.05;
    let limit_y = (half_map_h + margin_y - half_view_h_target).max(0.0);
    rig.target_pos.y = rig.target_pos.y.clamp(-limit_y, limit_y);

    // Pürüzsüz (Smooth) İnterpolasyon:
    let lerp_speed = (16.0 * dt).clamp(0.0, 1.0);
    let new_scale = cam_tf.scale.x + (rig.target_scale - cam_tf.scale.x) * lerp_speed;
    cam_tf.scale = Vec3::splat(new_scale);

    cam_tf.translation.x += (rig.target_pos.x - cam_tf.translation.x) * lerp_speed;
    cam_tf.translation.y += (rig.target_pos.y - cam_tf.translation.y) * lerp_speed;

    // YATAY EKSEN SONSUZ DÖNGÜ (Infinite Horizontal Wrapping)
    let half_map_w = MAP_WIDTH_F32 * 0.5;
    if cam_tf.translation.x < -half_map_w {
        cam_tf.translation.x += MAP_WIDTH_F32;
        rig.target_pos.x += MAP_WIDTH_F32;
    } else if cam_tf.translation.x > half_map_w {
        cam_tf.translation.x -= MAP_WIDTH_F32;
        rig.target_pos.x -= MAP_WIDTH_F32;
    }

    let half_view_h = (win_h * 0.5) * new_scale;
    let cur_limit_y = (half_map_h + half_view_h * 0.05 - half_view_h).max(0.0);
    cam_tf.translation.y = cam_tf.translation.y.clamp(-cur_limit_y, cur_limit_y);
}

fn update_visual_lod(
    camera_q: Single<&Transform, With<Camera2d>>,
    mut border_q: Query<(&mut Sprite, &mut Visibility), With<InnerBorderLayer>>,
) {
    let scale = camera_q.scale.x;

    let fade_start = 0.65;
    let fade_end = 1.55;
    let t = ((scale - fade_start) / (fade_end - fade_start)).clamp(0.0, 1.0);
    let smooth_t = t * t * (3.0 - 2.0 * t);
    let border_alpha = 1.0 - smooth_t;

    for (mut sprite, mut vis) in &mut border_q {
        if border_alpha <= 0.01 {
            *vis = Visibility::Hidden;
        } else {
            *vis = Visibility::Inherited;
            sprite.color = Color::srgba(1.0, 1.0, 1.0, border_alpha);
        }
    }
}

pub fn refresh_province_panel_texts(
    selected_province: Option<u32>,
    map_data: &WorldMapData,
    scenario: &ActiveScenario,
    prov_panel_q: &mut Query<&mut Node, With<ProvincePanelRoot>>,
    prov_title_q: &mut Query<
        (&mut Text, &mut TextColor),
        (
            With<ProvincePanelTitle>,
            Without<HudHoverText>,
            Without<ProvincePanelDetails>,
        ),
    >,
    prov_details_q: &mut Query<
        &mut Text,
        (
            With<ProvincePanelDetails>,
            Without<HudHoverText>,
            Without<ProvincePanelTitle>,
        ),
    >,
) {
    let Some(mut panel_node) = prov_panel_q.iter_mut().next() else {
        return;
    };
    if let Some(sel_id) = selected_province {
        if let Some(p) = map_data.provinces.get(&sel_id) {
            panel_node.display = Display::Flex;
            let civ_opt = scenario
                .province_owners
                .get(&sel_id)
                .and_then(|cid| scenario.civs.get(cid));

            if let Some((mut title_txt, mut title_col)) = prov_title_q.iter_mut().next() {
                if let Some(civ) = civ_opt {
                    title_txt.0 = format!("{} (ID: #{})  —  {}", p.name, p.id, civ.name);
                    title_col.0 = Color::srgb_u8(
                        civ.color[0].saturating_add(45),
                        civ.color[1].saturating_add(45),
                        civ.color[2].saturating_add(45),
                    );
                } else {
                    title_txt.0 = format!("{} (ID: #{}) — Tarafsız", p.name, p.id);
                    title_col.0 = Color::srgb(0.92, 0.90, 0.85);
                }
            }

            if let Some(mut det_txt) = prov_details_q.iter_mut().next() {
                let (cap_str, pop_str, eco_str) = if let Some(civ) = civ_opt {
                    (
                        civ.capital_name.as_str(),
                        format_number(civ.population),
                        format_number(civ.economy as u64),
                    )
                } else {
                    ("-", "-".to_string(), "-".to_string())
                };
                det_txt.0 = format!(
                    "Kıta: {}   •   Kalkınma: %{}   •   Koordinat: [X: {:.0}, Y: {:.0}]\nBaşkent: {}   •   Ülke Nüfusu: {}   •   Ekonomi: {}",
                    p.continent, p.growth_rate, p.city_px[0], p.city_px[1], cap_str, pop_str, eco_str
                );
            }
        }
    } else {
        panel_node.display = Display::None;
    }
}

fn province_interaction(
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
    let allow_hover_highlight = scale < 1.65 && !pointer_over_ui;

    let mut cursor_pid = None;
    if !pointer_over_ui {
        if let Some(cursor_pos) = cursor_opt {
            if let Ok(world_pos) = camera.viewport_to_world_2d(cam_global, cursor_pos) {
                if let Some((px, py)) = world_to_map_pixel(world_pos) {
                    let pid = map_data.get_province_at(px, py);
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
            map_data.selected_province = cursor_pid;

            if let Some(sel_id) = map_data.selected_province {
                if *state.get() == AppState::ScenarioSelect {
                    if let Some(&cid) = scenario.province_owners.get(&sel_id) {
                        session.player_civ_id = cid;
                    }
                }
            }

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

fn build_province_mask_image(
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

fn update_province_overlays(
    map_data: Res<WorldMapData>,
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
    mut prev_selected: Local<Option<u32>>,
) {
    if *prev_hovered != map_data.hovered_province || *prev_selected != map_data.selected_province {
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

    if *prev_selected != map_data.selected_province {
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

    *prev_hovered = map_data.hovered_province;
    *prev_selected = map_data.selected_province;
}

fn setup_ingame_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scenario: Res<ActiveScenario>,
    session: Res<GameSession>,
    fonts: Res<UiFonts>,
) {
    let player_civ = scenario.civs.get(&session.player_civ_id);
    let (civ_name, civ_color, flag_path) = if let Some(c) = player_civ {
        (
            c.name.clone(),
            Color::srgb_u8(c.color[0], c.color[1], c.color[2]),
            ActiveScenario::get_flag_path(&c.tag, None),
        )
    } else {
        (
            "Bilinmiyor".to_string(),
            Color::srgb(0.8, 0.2, 0.2),
            "flags/unkn/0.png".to_string(),
        )
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            Pickable::IGNORE,
            InGameHudRoot,
        ))
        .with_children(|root| {
            // ÜST BAR
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(58.0),
                    padding: UiRect {
                        left: Val::Px(22.0),
                        right: Val::Px(150.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    border: UiRect::bottom(Val::Px(1.5)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.06, 0.08, 0.13, 0.94)),
                BorderColor::all(Color::srgba(0.76, 0.64, 0.38, 0.65)),
            ))
            .with_children(|topbar| {
                // Sol: Oyuncu ülkesi bayrağı, renk rozeti ve adı
                topbar
                    .spawn(Node {
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..default()
                    })
                    .with_children(|left| {
                        // Bayrak ve dalgalanma efekti kutusu
                        left.spawn(Node {
                            width: Val::Px(42.0),
                            height: Val::Px(28.0),
                            position_type: PositionType::Relative,
                            border: UiRect::all(Val::Px(1.5)),
                            border_radius: BorderRadius::all(Val::Px(5.0)),
                            overflow: Overflow::clip(),
                            ..default()
                        })
                        .with_children(|flag_box| {
                            flag_box.spawn((
                                ImageNode {
                                    image: asset_server.load(flag_path),
                                    ..default()
                                },
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                HudPlayerFlagNode,
                            ));

                            flag_box.spawn((
                                ImageNode {
                                    image: asset_server.load("flags/flagOver.png"),
                                    ..default()
                                },
                                Node {
                                    position_type: PositionType::Absolute,
                                    top: Val::Px(0.0),
                                    left: Val::Px(0.0),
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                Pickable::IGNORE,
                            ));
                        });

                        left.spawn((
                            Node {
                                width: Val::Px(14.0),
                                height: Val::Px(14.0),
                                border_radius: BorderRadius::all(Val::Px(7.0)),
                                ..default()
                            },
                            BackgroundColor(civ_color),
                            HudPlayerColorBadge,
                        ));

                        left.spawn((
                            Text::new(format!(
                                "{}   •   29 Ekim 2026   •   Tur {}",
                                civ_name, session.turn
                            )),
                            TextFont {
                                font: fonts.heading.clone(),
                                font_size: FontSize::Px(16.5),
                                ..default()
                            },
                            TextColor(Color::srgb(0.96, 0.94, 0.88)),
                            HudPlayerTitleText,
                        ));
                    });

                // Orta: Fare altındaki şehir/ülke bilgisi
                topbar.spawn((
                    Text::new("Haritada bir şehrin üzerine gelin veya tıklayın"),
                    TextFont {
                        font: fonts.medium.clone(),
                        font_size: FontSize::Px(14.5),
                        ..default()
                    },
                    TextColor(Color::srgb(0.75, 0.79, 0.86)),
                    HudHoverText,
                ));

                // Sağ: Ana Menüye Dön butonu
                topbar
                    .spawn((
                        Node {
                            padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                            border: UiRect::all(Val::Px(1.5)),
                            border_radius: BorderRadius::all(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.14, 0.17, 0.24, 0.95)),
                        BorderColor::all(Color::srgba(0.72, 0.60, 0.36, 0.80)),
                        BackToMenuButton,
                    ))
                    .observe(
                        |ev: On<PointerOver>, mut bg_q: Query<&mut BackgroundColor>| {
                            if let Ok(mut bg) = bg_q.get_mut(ev.entity) {
                                bg.0 = Color::srgba(0.22, 0.27, 0.38, 0.98);
                            }
                        },
                    )
                    .observe(
                        |ev: On<PointerOut>, mut bg_q: Query<&mut BackgroundColor>| {
                            if let Ok(mut bg) = bg_q.get_mut(ev.entity) {
                                bg.0 = Color::srgba(0.14, 0.17, 0.24, 0.95);
                            }
                        },
                    )
                    .observe(
                        |_: On<PointerClick>, mut next_state: ResMut<NextState<AppState>>| {
                            next_state.set(AppState::MainMenu);
                        },
                    )
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("Ana Menü"),
                            TextFont {
                                font: fonts.body.clone(),
                                font_size: FontSize::Px(14.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.95, 0.92, 0.82)),
                            Pickable::IGNORE,
                        ));
                    });
            });

            // ALT SOL: Seçili Şehir/Eyalet Detay Paneli + Dinamik Renk & Fetih Aksiyonları
            root.spawn((
                Node {
                    display: Display::None,
                    width: Val::Px(495.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    padding: UiRect::all(Val::Px(18.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    border: UiRect::all(Val::Px(1.5)),
                    border_radius: BorderRadius::all(Val::Px(14.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.06, 0.08, 0.13, 0.95)),
                BorderColor::all(Color::srgba(0.80, 0.68, 0.38, 0.88)),
                ProvincePanelRoot,
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new("Seçili Şehir"),
                    TextFont {
                        font: fonts.heading.clone(),
                        font_size: FontSize::Px(19.5),
                        ..default()
                    },
                    TextColor(Color::srgb(0.96, 0.88, 0.64)),
                    ProvincePanelTitle,
                ));

                panel.spawn((
                    Text::new("Detaylar"),
                    TextFont {
                        font: fonts.medium.clone(),
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.82, 0.85, 0.90)),
                    ProvincePanelDetails,
                ));

                // Aksiyon Butonları: Ülke Rengini Değiştir & Şehri Fethet
                panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        margin: UiRect::top(Val::Px(4.0)),
                        ..default()
                    })
                    .with_children(|actions| {
                        // 1. Ülke Rengini Değiştir Butonu
                        actions
                            .spawn((
                                Node {
                                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                                    border: UiRect::all(Val::Px(1.5)),
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.14, 0.19, 0.29, 0.95)),
                                BorderColor::all(Color::srgba(0.55, 0.68, 0.88, 0.85)),
                            ))
                            .observe(
                                |ev: On<PointerOver>, mut bg_q: Query<&mut BackgroundColor>| {
                                    if let Ok(mut bg) = bg_q.get_mut(ev.entity) {
                                        bg.0 = Color::srgba(0.22, 0.29, 0.42, 0.99);
                                    }
                                },
                            )
                            .observe(
                                |ev: On<PointerOut>, mut bg_q: Query<&mut BackgroundColor>| {
                                    if let Ok(mut bg) = bg_q.get_mut(ev.entity) {
                                        bg.0 = Color::srgba(0.14, 0.19, 0.29, 0.95);
                                    }
                                },
                            )
                            .observe(
                                |_: On<PointerClick>,
                                 map_data: Res<WorldMapData>,
                                 session: Res<GameSession>,
                                 mut scenario: ResMut<ActiveScenario>,
                                 mut images: ResMut<Assets<Image>>,
                                 mut badge_q: Query<
                                    &mut BackgroundColor,
                                    With<HudPlayerColorBadge>,
                                 >,
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
                                 label_parent_q: Query<(&CountryMapLabel, &Children)>,
                                 mut text_col_q: Query<
                                    &mut TextColor,
                                    (
                                        Without<ProvincePanelTitle>,
                                        Without<ProvincePanelDetails>,
                                        Without<HudHoverText>,
                                    ),
                                 >| {
                                    let Some(sel_pid) = map_data.selected_province else {
                                        return;
                                    };
                                    let Some(&cid) = scenario.province_owners.get(&sel_pid) else {
                                        return;
                                    };
                                    let new_col = generate_random_civ_color(cid.wrapping_add(sel_pid));
                                    if let Some(civ) = scenario.civs.get_mut(&cid) {
                                        civ.color = new_col;
                                    }
                                    recolor_country_on_map(&map_data, &scenario, &mut images, cid);
                                    let new_label_col = get_country_label_color(new_col);
                                    for (lbl, children) in &label_parent_q {
                                        if lbl.civ_id == cid {
                                            for child in children.iter() {
                                                if let Ok(mut tc) = text_col_q.get_mut(child) {
                                                    tc.0 = new_label_col;
                                                }
                                            }
                                        }
                                    }
                                    if cid == session.player_civ_id {
                                        for mut badge_bg in &mut badge_q {
                                            badge_bg.0 =
                                                Color::srgb_u8(new_col[0], new_col[1], new_col[2]);
                                        }
                                    }
                                    refresh_province_panel_texts(
                                        map_data.selected_province,
                                        &map_data,
                                        &scenario,
                                        &mut prov_panel_q,
                                        &mut prov_title_q,
                                        &mut prov_details_q,
                                    );
                                },
                            )
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("🎨 Ülke Rengini Değiştir"),
                                    TextFont {
                                        font: fonts.body.clone(),
                                        font_size: FontSize::Px(13.0),
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.92, 0.95, 1.0)),
                                    Pickable::IGNORE,
                                ));
                            });

                        // 2. Şehri Fethet (Oyuncu Ülkesine Kat) Butonu
                        actions
                            .spawn((
                                Node {
                                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                                    border: UiRect::all(Val::Px(1.5)),
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.28, 0.14, 0.14, 0.95)),
                                BorderColor::all(Color::srgba(0.88, 0.52, 0.45, 0.85)),
                            ))
                            .observe(
                                |ev: On<PointerOver>, mut bg_q: Query<&mut BackgroundColor>| {
                                    if let Ok(mut bg) = bg_q.get_mut(ev.entity) {
                                        bg.0 = Color::srgba(0.40, 0.18, 0.18, 0.99);
                                    }
                                },
                            )
                            .observe(
                                |ev: On<PointerOut>, mut bg_q: Query<&mut BackgroundColor>| {
                                    if let Ok(mut bg) = bg_q.get_mut(ev.entity) {
                                        bg.0 = Color::srgba(0.28, 0.14, 0.14, 0.95);
                                    }
                                },
                            )
                            .observe(
                                |_: On<PointerClick>,
                                 map_data: Res<WorldMapData>,
                                 session: Res<GameSession>,
                                 mut scenario: ResMut<ActiveScenario>,
                                 mut images: ResMut<Assets<Image>>,
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
                                 >| {
                                    let Some(sel_pid) = map_data.selected_province else {
                                        return;
                                    };
                                    let player_cid = session.player_civ_id;
                                    let old_cid = scenario.province_owners.get(&sel_pid).copied();
                                    if old_cid == Some(player_cid) {
                                        return;
                                    }
                                    if let Some(oc) = old_cid {
                                        if let Some(old_civ) = scenario.civs.get_mut(&oc) {
                                            old_civ.provinces.retain(|&p| p != sel_pid);
                                        }
                                    }
                                    if let Some(player_civ) = scenario.civs.get_mut(&player_cid) {
                                        if !player_civ.provinces.contains(&sel_pid) {
                                            player_civ.provinces.push(sel_pid);
                                        }
                                    }
                                    scenario.province_owners.insert(sel_pid, player_cid);
                                    recolor_province_on_map(
                                        &map_data,
                                        &scenario,
                                        &mut images,
                                        sel_pid,
                                    );
                                    refresh_province_panel_texts(
                                        map_data.selected_province,
                                        &map_data,
                                        &scenario,
                                        &mut prov_panel_q,
                                        &mut prov_title_q,
                                        &mut prov_details_q,
                                    );
                                },
                            )
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("⚔ Şehri Ülkeme Kat"),
                                    TextFont {
                                        font: fonts.body.clone(),
                                        font_size: FontSize::Px(13.0),
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 0.90, 0.88)),
                                    Pickable::IGNORE,
                                ));
                            });
                    });
            });
        });
}

fn cleanup_ingame_hud(mut commands: Commands, q: Query<Entity, With<InGameHudRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
