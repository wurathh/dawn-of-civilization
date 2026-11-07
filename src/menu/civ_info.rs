// ============================================================================
// DETAYLI ÜLKE BİLGİ VE JEOFİZİK VERİLERİ (Dossier Helpers)
// ============================================================================

pub fn get_civ_leader_info(tag: &str) -> (&'static str, &'static str) {
    let t = tag.split('_').next().unwrap_or(tag).to_lowercase();
    match t.as_str() {
        "tur" => ("Recep Tayyip Erdoğan", "Cumhurbaşkanı"),
        "usa" => ("Joe Biden", "Başkan"),
        "rus" => ("Vladimir Putin", "Devlet Başkanı"),
        "chi" | "chn" => ("Xi Jinping", "Devlet Başkanı"),
        "ger" | "deu" => ("Olaf Scholz", "Federal Şansölye"),
        "uni" | "gbr" => ("Keir Starmer", "Başbakan"),
        "fra" => ("Emmanuel Macron", "Cumhurbaşkanı"),
        "ita" => ("Giorgia Meloni", "Başbakan"),
        "jap" => ("Shigeru Ishiba", "Başbakan"),
        "ind" => ("Narendra Modi", "Başbakan"),
        "aze" => ("İlham Aliyev", "Cumhurbaşkanı"),
        "ukr" => ("Volodimir Zelenski", "Devlet Başkanı"),
        "irn" => ("Mesud Pezeşkiyan", "Cumhurbaşkanı"),
        "sau" => ("Muhammed bin Selman", "Veliaht Prens & Başbakan"),
        "egy" => ("Abdülfettah es-Sisi", "Cumhurbaşkanı"),
        "bra" => ("Luiz Inácio Lula da Silva", "Devlet Başkanı"),
        "can" => ("Justin Trudeau", "Başbakan"),
        "aus" => ("Anthony Albanese", "Başbakan"),
        "spa" | "esp" => ("Pedro Sánchez", "Başbakan"),
        "sok" => ("Yoon Suk Yeol", "Devlet Başkanı"),
        "prk" => ("Kim Jong-un", "Yüce Lider"),
        "isr" => ("Binyamin Netanyahu", "Başbakan"),
        "pal" => ("Mahmud Abbas", "Devlet Başkanı"),
        "gre" => ("Kiryakos Miçotakis", "Başbakan"),
        "pol" => ("Donald Tusk", "Başbakan"),
        "kaz" => ("Kasım Cömert Tokayev", "Cumhurbaşkanı"),
        "uzb" => ("Şevket Mirziyoyev", "Cumhurbaşkanı"),
        "pak" => ("Şahbaz Şerif", "Başbakan"),
        "alg" => ("Abdülmecid Tebbun", "Cumhurbaşkanı"),
        "mor" => ("VI. Muhammed", "Kral"),
        "syr" => ("Beşar Esad", "Devlet Başkanı"),
        "irq" => ("Muhammed Şiya es-Sudani", "Başbakan"),
        "cyn" => ("Ersin Tatar", "Cumhurbaşkanı"),
        "cpr" => ("Nikos Hristodulidis", "Cumhurbaşkanı"),
        "mex" => ("Claudia Sheinbaum", "Devlet Başkanı"),
        "arg" => ("Javier Milei", "Devlet Başkanı"),
        "swe" => ("Ulf Kristersson", "Başbakan"),
        "nor" => ("Jonas Gahr Støre", "Başbakan"),
        "fin" => ("Alexander Stubb", "Cumhurbaşkanı"),
        "nld" => ("Dick Schoof", "Başbakan"),
        "bel" => ("Alexander De Croo", "Başbakan"),
        _ => ("Devlet Başkanı", "Hükümet Lideri"),
    }
}

pub fn get_government_by_id(id: u32) -> &'static str {
    match id {
        1 => "City State",
        2 => "Communism",
        3 => "Constitutional Monarchy",
        4 => "Daimyo",
        5 => "Democracy",
        6 => "Fascism",
        7 => "Horde",
        8 => "Merchant Republic",
        9 => "Monarchy",
        10 => "Republic",
        11 => "Theocracy",
        12 => "Tribal",
        _ => "Republic",
    }
}

pub fn get_religion_by_id(id: u32) -> &'static str {
    match id {
        1 => "Animism",
        2 => "Atheism",
        3 => "Buddhism",
        4 => "Catholicism",
        5 => "Coptic Christianity",
        6 => "Confucianism",
        7 => "Gelugpa",
        8 => "Hellenic",
        9 => "Hinduism",
        10 => "Hussitism",
        11 => "Ibadi",
        12 => "Islam",
        13 => "Jainism",
        14 => "Judaism",
        15 => "Mahayana",
        16 => "Maya",
        17 => "Norse",
        18 => "Orthodoxy",
        19 => "Paganism",
        20 => "Protestantism",
        21 => "Shia",
        22 => "Shinto",
        23 => "Sikhism",
        24 => "Sunni",
        25 => "Taoism",
        26 => "Tengriism",
        27 => "Theravada",
        28 => "Totemism",
        29 => "Vajrayana",
        30 => "Zoroastrianism",
        _ => "Seküler",
    }
}

pub fn get_civ_government(tag: &str) -> &'static str {
    let t = tag.split('_').next().unwrap_or(tag).to_lowercase();
    match t.as_str() {
        "tur" => "Cumhurbaşkanlığı Hükümet Sistemi",
        "usa" => "Federal Başkanlık Cumhuriyeti",
        "rus" => "Federal Yarı Başkanlık Cumhuriyeti",
        "chi" | "chn" => "Tek Partili Sosyalist Cumhuriyet",
        "uni" | "gbr" | "jap" | "spa" | "esp" | "nld" | "bel" | "swe" | "nor" => {
            "Meşruti Monarşi (Demokrasi)"
        }
        "sau" | "uae" | "oma" | "qat" | "kwt" | "bhr" => "Monarşi / Emirlik",
        "irn" => "İslam Cumhuriyeti",
        "prk" => "Tek Partili Sosyalist Diktatörlük",
        "ger" | "deu" | "ita" | "ind" | "isr" => "Federal / Parlamenter Cumhuriyet",
        "cyn" => "Yarı Başkanlık Cumhuriyeti",
        _ => "Üniter Cumhuriyet",
    }
}

pub fn get_civ_religion(tag: &str) -> &'static str {
    let t = tag.split('_').next().unwrap_or(tag).to_lowercase();
    match t.as_str() {
        "tur" | "aze" | "sau" | "egy" | "alg" | "mor" | "pak" | "tun" | "lby" | "jor"
        | "syr" | "irq" | "cyn" => "İslam (Sünni / Çoğulcu)",
        "irn" => "İslam (Caferi Şiilik)",
        "isr" => "Musevilik (Yahudilik)",
        "rus" | "gre" | "geo" | "srb" | "bul" | "rom" | "ukr" | "cpr" | "blr" => {
            "Hristiyanlık (Ortodoks)"
        }
        "ita" | "spa" | "esp" | "fra" | "pol" | "bra" | "mex" | "arg" | "prt" | "por"
        | "irl" | "aut" | "hrv" => "Hristiyanlık (Katolik)",
        "usa" | "uni" | "gbr" | "ger" | "deu" | "swe" | "nor" | "fin" | "dnk" | "can"
        | "aus" | "nzl" => "Hristiyanlık (Protestan / Çoğulcu)",
        "chi" | "chn" | "jap" | "vie" | "tha" | "mmr" | "sok" => "Budizm / Şintoizm & Seküler",
        "ind" => "Hinduizm (%80) & İslam (%14)",
        _ => "Seküler / Çoğulcu",
    }
}

pub fn get_civ_technology(tag: &str, economy: u32) -> &'static str {
    let t = tag.split('_').next().unwrap_or(tag).to_lowercase();
    match t.as_str() {
        "usa" | "jap" | "ger" | "deu" => "Seviye 2.45 (Küresel İleri Teknoloji & Yapay Zeka)",
        "chi" | "chn" | "uni" | "gbr" | "fra" | "sok" => "Seviye 2.30 (Yüksek Teknoloji & Nükleer)",
        "tur" | "rus" | "ita" | "isr" | "can" | "swe" => "Seviye 2.10 (Modern Sanayi & Savunma)",
        "ind" | "bra" | "spa" | "esp" | "aus" | "sau" | "pol" => "Seviye 1.90 (Gelişmiş Ağır Sanayi)",
        _ => {
            if economy > 1000 {
                "Seviye 1.65 (Gelişmekte Olan Sanayi)"
            } else {
                "Seviye 1.30 (Standart Sanayi)"
            }
        }
    }
}

pub fn get_civ_diplomacy(tag: &str) -> (&'static str, &'static str) {
    let t = tag.split('_').next().unwrap_or(tag).to_lowercase();
    match t.as_str() {
        "tur" => (
            "Azerbaycan, KKTC, Pakistan, Katar",
            "Yunanistan, GKRY, Suriye",
        ),
        "usa" => (
            "NATO, Birleşik Krallık, İsrail, Japonya, G. Kore",
            "Rusya, Çin, İran, K. Kore",
        ),
        "rus" => (
            "Belarus, Kazakistan, Çin, İran, K. Kore",
            "ABD, NATO, Ukrayna",
        ),
        "chi" | "chn" => (
            "Rusya, Pakistan, K. Kore, Kamboçya",
            "ABD, Tayvan, Hindistan, Japonya",
        ),
        "ger" | "deu" => ("Fransa, AB Müttefikleri, NATO, ABD", "Rusya, K. Kore"),
        "uni" | "gbr" => (
            "ABD, İngiliz Milletler Topluluğu, NATO",
            "Rusya, Çin, İran",
        ),
        "fra" => ("Almanya, AB Müttefikleri, NATO, ABD", "Rusya"),
        "aze" => ("Türkiye, İsrail, Pakistan", "Ermenistan, İran"),
        "arm" => ("Yunanistan, Fransa, Rusya", "Azerbaycan, Türkiye"),
        "isr" => (
            "ABD, İngiltere, Almanya",
            "İran, Filistin, Suriye, Lübnan",
        ),
        "pal" => ("Arap Birliği, Türkiye, İran", "İsrail, ABD"),
        "gre" => ("GKRY, Fransa, ABD, İsrail", "Türkiye, KKTC"),
        "cyn" => ("Türkiye", "GKRY, Yunanistan"),
        "cpr" => ("Yunanistan, Fransa, İsrail", "KKTC, Türkiye"),
        "ukr" => ("NATO, ABD, Polonya, İngiltere", "Rusya, Belarus"),
        "irn" => (
            "Suriye, Rusya, Çin, Husiler",
            "ABD, İsrail, Suudi Arabistan",
        ),
        "sau" => ("B.A.E., Bahreyn, ABD, Mısır", "İran, Yemen (Husiler)"),
        "ind" => ("Rusya, ABD, Fransa, Japonya", "Pakistan, Çin"),
        "pak" => ("Çin, Türkiye, Suudi Arabistan", "Hindistan"),
        _ => ("Bölgesel Komşular, BM Ortakları", "Jeopolitik Rakipler"),
    }
}
