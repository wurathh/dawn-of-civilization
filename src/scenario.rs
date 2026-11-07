use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Clone, Debug)]
pub struct CivilizationData {
    pub id: u32,
    pub tag: String,
    pub name: String,
    pub color: [u8; 3],
    pub capital_province_id: u32,
    pub capital_name: String,
    pub population: u64,
    pub economy: u32,
    pub provinces: Vec<u32>,
    #[serde(default)]
    pub leader: Option<String>,
    #[serde(default)]
    pub government_id: Option<u32>,
    #[serde(default)]
    pub religion_id: Option<u32>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ScenarioFile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub civilizations: Vec<CivilizationData>,
}

#[derive(Resource, Clone, Debug)]
pub struct ActiveScenario {
    pub id: String,
    pub name: String,
    pub description: String,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    /// CivID -> CivilizationData
    pub civs: HashMap<u32, CivilizationData>,
    /// ProvinceID (1-based) -> CivID
    pub province_owners: HashMap<u32, u32>,
    /// Nüfus/büyüklük sırasına göre sıralanmış ülkeler listesi (menüde listelemek için)
    pub sorted_civ_ids: Vec<u32>,
}

impl ActiveScenario {
    pub fn load_from_file(path: &str) -> Self {
        let raw = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Senaryo dosyası okunamadı: {}", path));
        let parsed: ScenarioFile =
            serde_json::from_str(&raw).expect("Senaryo JSON parse edilemedi!");

        let mut civs = HashMap::with_capacity(parsed.civilizations.len());
        let mut province_owners = HashMap::with_capacity(7000);
        let mut sorted_civ_ids = Vec::with_capacity(parsed.civilizations.len());

        for civ in parsed.civilizations {
            if civ.provinces.is_empty() {
                continue;
            }
            for &pid in &civ.provinces {
                province_owners.insert(pid, civ.id);
            }
            sorted_civ_ids.push(civ.id);
            civs.insert(civ.id, civ);
        }

        // Eyalet sayısı ve nüfusa göre sırala (Türkiye en üstte, sonra büyük ülkeler)
        sorted_civ_ids.sort_by(|&a, &b| {
            if a == 1 {
                return std::cmp::Ordering::Less;
            }
            if b == 1 {
                return std::cmp::Ordering::Greater;
            }
            let ca = &civs[&a];
            let cb = &civs[&b];
            cb.provinces
                .len()
                .cmp(&ca.provinces.len())
                .then_with(|| cb.population.cmp(&ca.population))
        });

        Self {
            id: parsed.id,
            name: parsed.name,
            description: parsed.description,
            year: parsed.year,
            month: parsed.month,
            day: parsed.day,
            civs,
            province_owners,
            sorted_civ_ids,
        }
    }

    /// Bayrak yolunu belirler:
    /// 1. `assets/flags/{base_tag}/{ideology}.png` (varsa, örn: 1.png faşizm)
    /// 2. `assets/flags/{base_tag}/0.png` (ana bayrak)
    /// 3. `assets/flags/unkn/0.png` (bilinmeyen / varsayılan bayrak)
    pub fn get_flag_path(tag: &str, ideology_opt: Option<&str>) -> String {
        let base_tag = tag.split('_').next().unwrap_or(tag);

        if let Some(ideology) = ideology_opt {
            let ideo_path = format!("assets/flags/{}/{}.png", base_tag, ideology);
            if std::path::Path::new(&ideo_path).exists() {
                return format!("flags/{}/{}.png", base_tag, ideology);
            }
        }

        let main_path = format!("assets/flags/{}/0.png", base_tag);
        if std::path::Path::new(&main_path).exists() {
            return format!("flags/{}/0.png", base_tag);
        }

        "flags/unkn/0.png".to_string()
    }
}

pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);
    for (count, ch) in s.chars().rev().enumerate() {
        if count > 0 && count % 3 == 0 {
            result.push('.');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}
