use crate::map::rendering::InnerBorderLayer;
use bevy::prelude::*;

#[derive(Component)]
pub struct CountryMapLabel {
    pub civ_id: u32,
    pub min_visible_scale: f32,
}

#[derive(Component)]
pub struct CapitalCityBadge {
    pub min_visible_scale: f32,
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

pub fn format_country_map_name(name: &str) -> String {
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

pub fn get_badge_display_name(raw_name: &str) -> String {
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

pub fn update_country_labels_visibility(
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

pub fn update_visual_lod(
    camera_q: Single<&Transform, With<Camera2d>>,
    mut border_q: Query<(&mut Sprite, &mut Visibility), With<InnerBorderLayer>>,
) {
    let scale = camera_q.scale.x;

    let fade_start = 0.50;
    let fade_end = 1.35;
    let t = ((scale - fade_start) / (fade_end - fade_start)).clamp(0.0, 1.0);
    let smooth_t = t * t * (3.0 - 2.0 * t);
    // İç ilçe/eyalet sınırlarını daha yumuşak ve saydam tutarak ülke sınırlarından belirgin biçimde ayırıyoruz
    let border_alpha = (1.0 - smooth_t) * 0.70;

    for (mut sprite, mut vis) in &mut border_q {
        if border_alpha <= 0.01 {
            *vis = Visibility::Hidden;
        } else {
            *vis = Visibility::Inherited;
            sprite.color = Color::srgba(1.0, 1.0, 1.0, border_alpha);
        }
    }
}
