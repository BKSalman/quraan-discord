pub mod commands;

use std::{fs::File, io::BufReader, path::PathBuf};

use xml::{reader::XmlEvent, EventReader};

pub const ROOT_DIR: &str = env!("CARGO_MANIFEST_DIR");
const FIRST_PAGE_INDEX: u32 = 2;
const FILE_NAME_PADDING: usize = 3;

pub struct Data {
    pub quraan: Quraan,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, Default)]
pub struct Quraan {
    pub ayat: Vec<Ayat>,
}

#[derive(Debug, Default)]
pub struct Ayat {
    id: usize,
    jozz: u32,
    pub page: u32,
    sura_no: u32,
    pub sura_name_en: String,
    pub sura_name_ar: String,
    line_start: u32,
    line_end: u32,
    aya_no: u32,
    aya_text: String,
    aya_text_emlaey: String,
    aya_tafseer: String,
}

#[derive(Default, Debug)]
struct AyatBuilder {
    id: Option<usize>,
    jozz: Option<u32>,
    page: Option<u32>,
    sura_no: Option<u32>,
    sura_name_en: Option<String>,
    sura_name_ar: Option<String>,
    line_start: Option<u32>,
    line_end: Option<u32>,
    aya_no: Option<u32>,
    aya_text: Option<String>,
    aya_text_emlaey: Option<String>,
    aya_tafseer: Option<String>,
}

impl AyatBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn build(&self) -> Ayat {
        Ayat {
            id: self.id.unwrap(),
            jozz: self.jozz.unwrap(),
            page: self.page.unwrap(),
            sura_no: self.sura_no.unwrap(),
            sura_name_en: self.sura_name_en.clone().unwrap(),
            sura_name_ar: self.sura_name_ar.clone().unwrap(),
            line_start: self.line_start.unwrap(),
            line_end: self.line_end.unwrap(),
            aya_no: self.aya_no.clone().unwrap(),
            aya_text: self.aya_text.clone().unwrap(),
            aya_text_emlaey: self.aya_text_emlaey.clone().unwrap(),
            aya_tafseer: self.aya_tafseer.clone().unwrap(),
        }
    }

    fn id(&mut self, id: usize) {
        self.id = Some(id);
    }

    fn aya_text_emlaey(&mut self, aya_text_emlaey: String) {
        self.aya_text_emlaey = Some(aya_text_emlaey);
    }

    fn aya_text(&mut self, aya_text: String) {
        self.aya_text = Some(aya_text);
    }

    fn aya_no(&mut self, aya_no: u32) {
        self.aya_no = Some(aya_no);
    }

    fn line_end(&mut self, line_end: u32) {
        self.line_end = Some(line_end);
    }

    fn line_start(&mut self, line_start: u32) {
        self.line_start = Some(line_start);
    }

    fn sura_name_ar(&mut self, sura_name_ar: String) {
        self.sura_name_ar = Some(sura_name_ar);
    }

    fn sura_name_en(&mut self, sura_name_en: String) {
        self.sura_name_en = Some(sura_name_en);
    }

    fn sura_no(&mut self, sura_no: u32) {
        self.sura_no = Some(sura_no);
    }

    fn page(&mut self, page: u32) {
        self.page = Some(page);
    }

    fn jozz(&mut self, jozz: u32) {
        self.jozz = Some(jozz);
    }

    fn aya_tafseer(&mut self, aya_tafseer: String) {
        self.aya_tafseer = Some(aya_tafseer);
    }
}

pub fn parse_tafseer_mouaser() -> std::io::Result<Quraan> {
    println!("Started parsing Al Tafseer al Mouaser");
    let data_dir = PathBuf::from(ROOT_DIR).join("data");
    let file = File::open(
        data_dir.join("hafs_tafseerMouaser_v3/hafs_tafseerMouaser_v3_data/tafseerMouaser_v03.xml"),
    )?;
    let file = BufReader::new(file);

    let mut quraan = Quraan { ayat: Vec::new() };

    let parser = EventReader::new(file);
    let mut current_ayah = AyatBuilder::new();
    let mut set_id = false;
    let mut set_jozz = false;
    let mut set_page = false;
    let mut set_sura_no = false;
    let mut set_sura_name_en = false;
    let mut set_sura_name_ar = false;
    let mut set_line_start = false;
    let mut set_line_end = false;
    let mut set_aya_no = false;
    let mut set_aya_text = false;
    let mut set_aya_text_emlaey = false;
    let mut set_aya_tafseer = false;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                let name = name.to_string();
                if name == "ROW" {
                    current_ayah = AyatBuilder::new();
                }
                set_id = name == "id";
                set_jozz = name == "jozz";
                set_page = name == "page";
                set_sura_no = name == "sura_no";
                set_sura_name_en = name == "sura_name_en";
                set_sura_name_ar = name == "sura_name_ar";
                set_line_start = name == "line_start";
                set_line_end = name == "line_end";
                set_aya_no = name == "aya_no";
                set_aya_text = name == "aya_text";
                set_aya_text_emlaey = name == "aya_text_emlaey";
                set_aya_tafseer = name == "aya_tafseer";
            }
            Ok(XmlEvent::Characters(inner_text)) => {
                let inner_text = inner_text.trim().to_string();
                if set_id {
                    current_ayah.id(inner_text.parse().unwrap());
                } else if set_jozz {
                    current_ayah.jozz(inner_text.parse().unwrap());
                } else if set_page {
                    current_ayah.page(inner_text.parse().unwrap());
                } else if set_sura_no {
                    current_ayah.sura_no(inner_text.parse().unwrap());
                } else if set_sura_name_en {
                    current_ayah.sura_name_en(inner_text);
                } else if set_sura_name_ar {
                    let inner_text = remove_surah_name_harakat(&inner_text);
                    current_ayah.sura_name_ar(inner_text);
                } else if set_line_start {
                    current_ayah.line_start(inner_text.parse().unwrap());
                } else if set_line_end {
                    current_ayah.line_end(inner_text.parse().unwrap());
                } else if set_aya_no {
                    current_ayah.aya_no(inner_text.parse().unwrap());
                } else if set_aya_text {
                    current_ayah.aya_text(inner_text);
                } else if set_aya_text_emlaey {
                    current_ayah.aya_text_emlaey(inner_text);
                } else if set_aya_tafseer {
                    current_ayah.aya_tafseer(inner_text);
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.to_string() == "ROW" {
                    quraan.ayat.push(current_ayah.build());
                }
            }
            Err(e) => {
                eprintln!("parse_ar_tafseer: Error: {e}");
                break;
            }
            _ => {}
        }
    }

    println!("Finished parsing Al Tafseer al Mouaser");

    Ok(quraan)
}

fn remove_surah_name_harakat(surah_name: &str) -> String {
    surah_name
        .chars()
        .filter_map(|c| match c {
            // harakat
            '\u{64b}'..='\u{65f}' => None,
            // alif maqsoorah
            '\u{670}' => None,
            _ => Some(c),
        })
        .collect()
}

fn get_page_image_name(page_number: u32) -> String {
    format!(
        "{ROOT_DIR}/data/arabic-quran-images/-{:0width$}.png",
        FIRST_PAGE_INDEX + page_number,
        width = FILE_NAME_PADDING
    )
}
