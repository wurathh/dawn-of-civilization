use crate::map::types::{decode_row_to_civ, HALF_W_U32, MAP_HEIGHT, MAP_WIDTH, WorldMapData};
use crate::scenario::ActiveScenario;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Component)]
pub struct InnerBorderLayer;

#[derive(Component)]
pub struct PoliticalMapLayer;

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
            civ_fill[c] = [civ.color[0], civ.color[1], civ.color[2], 148];
            civ_halo[c] = [
                civ.color[0].saturating_add(25),
                civ.color[1].saturating_add(25),
                civ.color[2].saturating_add(25),
                220,
            ];
            // Ülke sınır çizgisini (outer border) ilçe sınırlarından net ayırmak için
            // koyu, belirgin ve opak derin hat
            civ_edge[c] = [
                civ.color[0].saturating_sub(110),
                civ.color[1].saturating_sub(110),
                civ.color[2].saturating_sub(110),
                255,
            ];
        }
    }

    let half_w = HALF_W_U32 as usize;
    let map_w = MAP_WIDTH as usize;
    let mut row_curr = vec![0u16; map_w];
    let mut row_prev = vec![0u16; map_w];
    let mut row_next = vec![0u16; map_w];

    if min_y > 0 {
        decode_row_to_civ(&row_runs[(min_y - 1) as usize], &pid_to_civ, &mut row_prev);
    }
    decode_row_to_civ(&row_runs[min_y as usize], &pid_to_civ, &mut row_curr);

    for y in min_y..=max_y {
        if y + 1 < MAP_HEIGHT {
            decode_row_to_civ(&row_runs[(y + 1) as usize], &pid_to_civ, &mut row_next);
        } else {
            row_next.fill(0);
        }

        let y_usize = y as usize;
        let row_offset = y_usize * half_w;

        for x in min_x..=max_x {
            let x_usize = x as usize;
            let c = row_curr[x_usize] as usize;

            if c == 0 {
                if clear_unowned {
                    let (buf, lx) = if x_usize < half_w {
                        (&mut *west_buf, x_usize)
                    } else {
                        (&mut *east_buf, x_usize - half_w)
                    };
                    let idx = (row_offset + lx) * 4;
                    buf[idx] = 0;
                    buf[idx + 1] = 0;
                    buf[idx + 2] = 0;
                    buf[idx + 3] = 0;
                }
                continue;
            }

            let left_c = if x_usize > 0 { row_curr[x_usize - 1] as usize } else { c };
            let right_c = if x_usize + 1 < map_w { row_curr[x_usize + 1] as usize } else { c };
            let up_c = row_prev[x_usize] as usize;
            let down_c = row_next[x_usize] as usize;

            let left2_c = if x_usize >= 2 { row_curr[x_usize - 2] as usize } else { c };
            let right2_c = if x_usize + 2 < map_w { row_curr[x_usize + 2] as usize } else { c };

            // 1 veya 2 piksel mesafede komşu ülke/boşluk varsa belirgin dış sınır (outer border)
            let is_country_edge = left_c != c || right_c != c || up_c != c || down_c != c;
            let is_near_edge = left2_c != c || right2_c != c;

            let col = if is_country_edge {
                &civ_edge[c]
            } else if is_near_edge {
                &civ_edge[c] // 2px kalınlıkta net sınır çizgisi
            } else {
                let left3 = if x_usize >= 3 { row_curr[x_usize - 3] as usize } else { c };
                let right3 = if x_usize + 3 < map_w { row_curr[x_usize + 3] as usize } else { c };
                if left3 != c || right3 != c {
                    &civ_halo[c]
                } else {
                    &civ_fill[c]
                }
            };

            let (buf, lx) = if x_usize < half_w {
                (&mut *west_buf, x_usize)
            } else {
                (&mut *east_buf, x_usize - half_w)
            };
            let idx = (row_offset + lx) * 4;
            buf[idx] = col[0];
            buf[idx + 1] = col[1];
            buf[idx + 2] = col[2];
            buf[idx + 3] = col[3];
        }

        std::mem::swap(&mut row_prev, &mut row_curr);
        std::mem::swap(&mut row_curr, &mut row_next);
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
        // Batı ve Doğu yarıları için birleşik kutuları hesapla
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

/// Tüm haritayı yeni veya güncellenen senaryoya göre baştan yeniden renklendirir.
pub fn rerender_full_world_map(
    map_data: &WorldMapData,
    scenario: &ActiveScenario,
    images: &mut Assets<Image>,
) {
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
            0,
            0,
            MAP_WIDTH - 1,
            MAP_HEIGHT - 1,
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

/// Antialiased yuvarlak köşeli ve kenarlıklı (border) 2D dikdörtgen dokusu oluşturur.
pub fn generate_rounded_rect_texture(
    w: u32,
    h: u32,
    radius: f32,
    fill_color: [u8; 4],
    border_color: [u8; 4],
    border_width: f32,
) -> Image {
    let mut buf = vec![0u8; (w * h * 4) as usize];
    let hw = w as f32 * 0.5;
    let hh = h as f32 * 0.5;
    let r = radius.min(hw).min(hh);

    for y in 0..h {
        let py = (y as f32 + 0.5) - hh;
        for x in 0..w {
            let px = (x as f32 + 0.5) - hw;
            let qx = px.abs() - (hw - r);
            let qy = py.abs() - (hh - r);
            let dist = (qx.max(0.0).hypot(qy.max(0.0)) + qx.max(qy).min(0.0)) - r;

            let outer_alpha = (0.5 - dist).clamp(0.0, 1.0);
            if outer_alpha <= 0.0 {
                continue;
            }

            let inner_dist = dist + border_width;
            let border_t = (inner_dist + 0.5).clamp(0.0, 1.0);

            let r_col = fill_color[0] as f32 * (1.0 - border_t) + border_color[0] as f32 * border_t;
            let g_col = fill_color[1] as f32 * (1.0 - border_t) + border_color[1] as f32 * border_t;
            let b_col = fill_color[2] as f32 * (1.0 - border_t) + border_color[2] as f32 * border_t;
            let a_base = fill_color[3] as f32 * (1.0 - border_t) + border_color[3] as f32 * border_t;
            let a_col = a_base * outer_alpha;

            let idx = ((y * w + x) * 4) as usize;
            buf[idx] = r_col.round() as u8;
            buf[idx + 1] = g_col.round() as u8;
            buf[idx + 2] = b_col.round() as u8;
            buf[idx + 3] = a_col.round() as u8;
        }
    }

    Image::new(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        buf,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
}
