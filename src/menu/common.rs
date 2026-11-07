use bevy::color::Alpha;
use bevy::prelude::*;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

/// Menü ve Ayarlar arasında arka plan resminin değişmeden sabit kalmasını sağlayan kaynak.
#[derive(Resource, Default)]
pub struct ActiveMenuBackground(pub Option<Handle<Image>>);

/// Her UI öğesinin özgün (tasarım anındaki) taban alfa değerlerini saklar.
#[derive(Component, Clone, Copy, Debug)]
pub struct UiFadeAlphaTracker {
    pub image: Option<f32>,
    pub text: Option<f32>,
    pub bg: Option<f32>,
    pub border: Option<(f32, f32, f32, f32)>,
}

/// Verilen kök panelin ve tüm alt elemanlarının şeffaflığını (0.0 -> 1.0) gerçek alfa ile çarparak ayarlar.
/// Hiçbir siyah perde veya yapay kutu kullanmaz; elemanlar şeffaflaşarak arka planı gösterir.
pub fn apply_panel_opacity(
    root: Entity,
    opacity: f32,
    commands: &mut Commands,
    children_q: &Query<&Children>,
    fade_q: &Query<&UiFadeAlphaTracker>,
    img_q: &mut Query<&mut ImageNode>,
    text_q: &mut Query<&mut TextColor>,
    bg_q: &mut Query<&mut BackgroundColor>,
    border_q: &mut Query<&mut BorderColor>,
) {
    let mut stack = vec![root];
    while let Some(e) = stack.pop() {
        if let Ok(children) = children_q.get(e) {
            for child in children.iter() {
                stack.push(child);
            }
        }

        // Özgün taban alfayı kaydet veya var olanı oku
        let tracker = if let Ok(existing) = fade_q.get(e) {
            *existing
        } else {
            let base_image = img_q.get(e).ok().map(|i| i.color.to_srgba().alpha);
            let base_text = text_q.get(e).ok().map(|t| t.0.to_srgba().alpha);
            let base_bg = bg_q.get(e).ok().map(|b| b.0.to_srgba().alpha);
            let base_border = border_q.get(e).ok().map(|b| (
                b.top.to_srgba().alpha,
                b.right.to_srgba().alpha,
                b.bottom.to_srgba().alpha,
                b.left.to_srgba().alpha,
            ));

            let new_tracker = UiFadeAlphaTracker {
                image: base_image,
                text: base_text,
                bg: base_bg,
                border: base_border,
            };
            commands.entity(e).insert(new_tracker);
            new_tracker
        };

        // Opaklığı uygula
        if let Some(base) = tracker.image {
            if let Ok(mut img) = img_q.get_mut(e) {
                img.color = img.color.with_alpha(base * opacity);
            }
        }
        if let Some(base) = tracker.text {
            if let Ok(mut text) = text_q.get_mut(e) {
                text.0 = text.0.with_alpha(base * opacity);
            }
        }
        if let Some(base) = tracker.bg {
            if let Ok(mut bg) = bg_q.get_mut(e) {
                bg.0 = bg.0.with_alpha(base * opacity);
            }
        }
        if let Some((top, right, bottom, left)) = tracker.border {
            if let Ok(mut border) = border_q.get_mut(e) {
                border.top = border.top.with_alpha(top * opacity);
                border.right = border.right.with_alpha(right * opacity);
                border.bottom = border.bottom.with_alpha(bottom * opacity);
                border.left = border.left.with_alpha(left * opacity);
            }
        }
    }
}

/// `assets/backgrounds` klasöründeki tüm resimleri (.png, .jpg, .jpeg, .webp) tarar
/// ve her çağrıldığında rastgele bir tanesinin asset yolunu döndürür.
pub fn pick_random_background_image() -> Option<String> {
    let dir = fs::read_dir("assets/backgrounds").ok()?;
    let mut files: Vec<String> = dir
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_file() {
                return None;
            }
            let ext = path.extension()?.to_str()?.to_ascii_lowercase();
            if matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "webp") {
                let fname = path.file_name()?.to_str()?;
                Some(format!("backgrounds/{}", fname))
            } else {
                None
            }
        })
        .collect();

    if files.is_empty() {
        return None;
    }

    files.sort();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as usize + (d.as_secs() as usize) * 31)
        .unwrap_or(0);
    let idx = nanos % files.len();
    Some(files.swap_remove(idx))
}
