use crate::map::camera::{BaseMapX, WrapCopy};
use crate::map::interaction::{
    HoveredProvinceOverlay, ProvinceHoverTooltipBg, ProvinceHoverTooltipBorder,
    ProvinceHoverTooltipFlag, ProvinceHoverTooltipRoot, ProvinceHoverTooltipTitle,
    SelectedProvinceOverlay,
};
use crate::map::labels::{
    char_width_ratio, format_country_map_name, get_badge_display_name, get_badge_text_width,
    get_country_label_color, BadgeAabb, CapitalCityBadge, CountryMapLabel, PlacedCharInfo,
};
use crate::map::rendering::{
    render_political_rect_to_buffers, InnerBorderLayer, PoliticalMapLayer,
};
use crate::map::types::{
    load_rle_rows, map_pixel_to_world, ProvincesFile, HALF_W, HALF_W_U32, MAP_HEIGHT, MAP_WIDTH,
    MAP_WIDTH_F32, WorldMapData,
};
use crate::scenario::ActiveScenario;
use crate::state::UiFonts;
use crate::ui::{Theme, Ui};
use bevy::asset::RenderAssetUsages;
use bevy::picking::prelude::*;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::{Anchor, SpriteImageMode};
use std::collections::HashMap;
use std::fs;

pub fn setup_world_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scenario: Res<ActiveScenario>,
    mut images: ResMut<Assets<Image>>,
    fonts: Res<UiFonts>,
    _theme: Res<Theme>,
    ui: Res<Ui>,
) {
    commands.spawn((
        Camera2d,
        Msaa::Sample4,
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

        // Taşmayı KESİNLİKLE engellemek için güvenli çekirdek alanı
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

        let text_color = get_country_label_color(civ.color);

        for wrap_idx in [-1i32, 0, 1] {
            let wrap_offset_x = wrap_idx as f32 * MAP_WIDTH_F32;

            // 1. Uzaktan görünen büyük harita ülke adı
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

        // 2. Başkent Rozeti
        let cap_pos_opt = cap_coords.or_else(|| mainland_provinces.first().map(|p| p.city_px));

        if let Some(c_px) = cap_pos_opt {
            let c_world = map_pixel_to_world(c_px[0], c_px[1]);

            let badge_title = get_badge_display_name(&civ.name);
            let text_w = get_badge_text_width(&badge_title);
            let real_text_w = text_w * 0.94;
            let badge_w = (real_text_w + 33.0).max(38.0);
            let badge_h = 20.0;

            let mut candidate_offsets: Vec<(f32, f32)> = Vec::with_capacity(128);

            // Mikro-bölgesel yön tercihleri
            if civ.tag == "cyn" {
                candidate_offsets.push((0.0, 6.0));
            } else if civ.tag == "cpr" {
                candidate_offsets.push((0.0, -7.0));
            } else if civ.tag == "jor" {
                candidate_offsets.push((16.0, 0.0));
            } else if civ.tag == "isr" {
                candidate_offsets.push((-15.0, 9.0));
            } else if civ.tag == "pal" {
                candidate_offsets.push((0.0, -14.0));
            } else if civ.tag == "vat" {
                candidate_offsets.push((-8.0, 8.0));
            } else if civ.tag == "mco" {
                candidate_offsets.push((8.0, -8.0));
            } else if civ.tag == "smr" {
                candidate_offsets.push((0.0, -8.0));
            } else if civ.tag == "cog" {
                candidate_offsets.push((-12.0, 10.0));
            } else if civ.tag == "drc_rel1_r" {
                candidate_offsets.push((12.0, -10.0));
            }

            if !candidate_offsets.contains(&(0.0, 0.0)) {
                candidate_offsets.push((0.0, 0.0));
            }

            // Global Eliptik Spiral Arama
            let aspect = (badge_w / badge_h).clamp(1.5, 3.2);
            for step_r in 1..=24 {
                let r = step_r as f32 * 7.5 + 8.0;
                let rx = r * aspect * 0.52;
                let ry = r;

                candidate_offsets.push((0.0, ry));
                candidate_offsets.push((0.0, -ry));
                candidate_offsets.push((rx, 0.0));
                candidate_offsets.push((-rx, 0.0));

                for ang_deg in [30.0, 45.0, 60.0, 120.0, 135.0, 150.0, 210.0, 225.0, 240.0, 300.0, 315.0, 330.0] {
                    let rad = ang_deg * std::f32::consts::PI / 180.0;
                    candidate_offsets.push((rx * rad.cos(), ry * rad.sin()));
                }
            }

            let mut best_ox = 0.0;
            let mut best_oy = 0.0;
            let mut found_non_overlapping = false;

            for (ox, oy) in candidate_offsets {
                let cand_min_x = c_world.x + ox - badge_w * 0.5;
                let cand_max_x = c_world.x + ox + badge_w * 0.5;
                let cand_min_y = c_world.y + oy - badge_h * 0.5;
                let cand_max_y = c_world.y + oy + badge_h * 0.5;

                let has_conflict = placed_badge_boxes.iter().any(|bb| {
                    bb.overlaps(cand_min_x, cand_max_x, cand_min_y, cand_max_y, 1.5)
                        || bb.overlaps(cand_min_x + MAP_WIDTH_F32, cand_max_x + MAP_WIDTH_F32, cand_min_y, cand_max_y, 1.5)
                        || bb.overlaps(cand_min_x - MAP_WIDTH_F32, cand_max_x - MAP_WIDTH_F32, cand_min_y, cand_max_y, 1.5)
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
                best_oy = (placed_badge_boxes.len() as f32 + 1.0) * (badge_h + 3.0);
                placed_badge_boxes.push(BadgeAabb {
                    min_x: c_world.x - badge_w * 0.5,
                    max_x: c_world.x + badge_w * 0.5,
                    min_y: c_world.y + best_oy - badge_h * 0.5,
                    max_y: c_world.y + best_oy + badge_h * 0.5,
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

            let flag_center_x = -badge_w * 0.5 + 14.0;
            let text_start_x = -badge_w * 0.5 + 26.0;

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
                        // 1. Tooltip.png Dokusu ile 9-slice Başkent Çerçevesi (Tooltip ile aynı %92 opaklık)
                        parent.spawn((
                            Sprite {
                                image: ui.tooltip.clone(),
                                image_mode: SpriteImageMode::Sliced(TextureSlicer {
                                    border: BorderRect::all(8.0),
                                    center_scale_mode: SliceScaleMode::Stretch,
                                    sides_scale_mode: SliceScaleMode::Stretch,
                                    max_corner_scale: 1.0,
                                }),
                                color: Color::srgba(1.0, 1.0, 1.0, 0.92),
                                custom_size: Some(Vec2::new(badge_w, badge_h)),
                                ..default()
                            },
                            Transform::from_xyz(0.0, 0.0, 0.0),
                            Pickable::IGNORE,
                        ));

                        // 2. Bayrak Arkası İnce Kenarlık
                        parent.spawn((
                            Sprite {
                                color: Color::srgba(0.08, 0.08, 0.10, 0.95),
                                custom_size: Some(Vec2::new(16.0, 11.5)),
                                ..default()
                            },
                            Transform::from_xyz(flag_center_x, 0.0, 0.05),
                            Pickable::IGNORE,
                        ));

                        // 3. Başkent Ülke Bayrağı
                        parent.spawn((
                            Sprite {
                                image: flag_handle.clone(),
                                custom_size: Some(Vec2::new(14.0, 9.5)),
                                ..default()
                            },
                            Transform::from_xyz(flag_center_x, 0.0, 0.1),
                            Pickable::IGNORE,
                        ));

                        // 4. Başkent Şehir Adı
                        parent.spawn((
                            Text2d::new(badge_title.clone()),
                            TextFont {
                                font: fonts.heading.clone(),
                                font_size: FontSize::Px(20.0),
                                ..default()
                            },
                            Anchor::CENTER_LEFT,
                            TextColor(Color::srgb(0.92, 0.90, 0.83)),
                            Transform::from_xyz(text_start_x, 0.0, 0.1).with_scale(Vec3::splat(0.5)),
                            Pickable::IGNORE,
                        ));
                    });
            }
        }
    }

    // 5. Şehir / Ülke Üzerine Gelince (Hover) Beliren Dinamik Fare Tooltip Kartı (Başkent Rozeti ile Birebir Aynı 2D Sprite Mimarisi)
    commands
        .spawn((
            Transform::from_xyz(0.0, 0.0, 99.0),
            Visibility::Hidden,
            Pickable::IGNORE,
            ProvinceHoverTooltipRoot,
        ))
        .with_children(|root| {
            // 1. Tooltip.png Dokusu ile 9-slice Çerçeve (%92 opaklık)
            root.spawn((
                Sprite {
                    image: ui.tooltip.clone(),
                    image_mode: SpriteImageMode::Sliced(TextureSlicer {
                        border: BorderRect::all(8.0),
                        center_scale_mode: SliceScaleMode::Stretch,
                        sides_scale_mode: SliceScaleMode::Stretch,
                        max_corner_scale: 1.0,
                    }),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.92),
                    custom_size: Some(Vec2::new(48.0, 26.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
                Pickable::IGNORE,
                ProvinceHoverTooltipBg,
            ));

            // 2. Bayrak Arkası İnce Kenarlık
            root.spawn((
                Sprite {
                    color: Color::srgba(0.08, 0.08, 0.10, 0.95),
                    custom_size: Some(Vec2::new(19.0, 13.5)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 1.0),
                Pickable::IGNORE,
                ProvinceHoverTooltipBorder,
            ));

            // 3. Bayrak
            root.spawn((
                Sprite {
                    image: asset_server.load("flags/unkn/0.png"),
                    custom_size: Some(Vec2::new(17.0, 11.5)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 2.0),
                Pickable::IGNORE,
                ProvinceHoverTooltipFlag,
            ));

            // 4. Şehir / Ülke Adı (Bir tık daha büyük ve net 11.5px font)
            root.spawn((
                Text2d::new(""),
                TextFont {
                    font: fonts.heading.clone(),
                    font_size: FontSize::Px(23.0),
                    ..default()
                },
                Anchor::CENTER_LEFT,
                TextColor(Color::srgb(0.94, 0.92, 0.86)),
                Transform::from_xyz(0.0, 0.0, 2.0).with_scale(Vec3::splat(0.5)),
                Pickable::IGNORE,
                ProvinceHoverTooltipTitle,
            ));
        });

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
