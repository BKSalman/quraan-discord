use std::collections::HashMap;
use std::io::BufReader;
use std::{fs::File, path::PathBuf};

use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Default)]
pub struct Quraan {
    /// سور
    pub suar: Vec<Surah>,
}

#[derive(Debug, Default)]
pub struct Surah {
    pub name: String,
    pub index: usize,
    /// آيات
    pub ayat: Vec<Ayah>,
}

#[derive(Debug, Default)]
pub struct Ayah {
    pub index: usize,
    pub text: String,
    pub tafseer: HashMap<String, String>,
}

pub fn parse_xml() -> std::io::Result<Quraan> {
    let root = env!("CARGO_MANIFEST_DIR");
    let data_dir = PathBuf::from(root).join("data");

    let file = File::open(data_dir.join("quran-simple.xml"))?;
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut quraan = Quraan::default();
    let mut current_surah = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.to_string() == "sura" {
                    let surah_index = attributes
                        .iter()
                        .find(|a| a.name.to_string() == "index")
                        .expect("surah should have index field")
                        .value
                        .parse::<usize>()
                        .expect("index should be a number");
                    let surah_name = attributes
                        .iter()
                        .find(|a| a.name.to_string() == "name")
                        .cloned()
                        .expect("surah should have name field")
                        .value;

                    let surah = Surah {
                        name: surah_name,
                        index: surah_index,
                        ayat: Vec::new(),
                    };

                    quraan.suar.push(surah);
                    current_surah = surah_index - 1;
                } else if name.to_string() == "aya" {
                    let ayah_index = attributes
                        .iter()
                        .find(|a| a.name.to_string() == "index")
                        .expect("surah should have index field")
                        .value
                        .parse::<usize>()
                        .expect("index should be a number");
                    let ayah_text = attributes
                        .iter()
                        .find(|a| a.name.to_string() == "text")
                        .cloned()
                        .expect("surah should have name field")
                        .value;

                    let ayah = Ayah {
                        text: ayah_text,
                        index: ayah_index,
                        tafseer: HashMap::new(),
                    };

                    quraan.suar[current_surah].ayat.push(ayah);
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            _ => {}
        }
    }

    parse_ar_tafseer(&mut quraan, data_dir)?;

    Ok(quraan)
}

fn parse_ar_tafseer(quraan: &mut Quraan, data_dir: PathBuf) -> std::io::Result<()> {
    let file = File::open(data_dir.join("ar.muyassar.xml"))?;
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut current_surah = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.to_string() == "sura" {
                    let surah_index = attributes
                        .iter()
                        .find(|a| a.name.to_string() == "index")
                        .expect("surah should have index field")
                        .value
                        .parse::<usize>()
                        .expect("index should be a number");

                    if quraan
                        .suar
                        .iter()
                        .find(|s| s.index == surah_index)
                        .is_some()
                    {
                        current_surah = surah_index - 1;
                    }
                } else if name.to_string() == "aya" {
                    let ayah_index = attributes
                        .iter()
                        .find(|a| a.name.to_string() == "index")
                        .expect("surah should have index field")
                        .value
                        .parse::<usize>()
                        .expect("index should be a number");
                    let ayah_tafseer = attributes
                        .iter()
                        .find(|a| a.name.to_string() == "text")
                        .cloned()
                        .expect("surah should have index field")
                        .value;

                    if let Some(ayah) = quraan.suar[current_surah]
                        .ayat
                        .iter_mut()
                        .find(|a| a.index == ayah_index)
                    {
                        ayah.tafseer.insert(String::from("ar"), ayah_tafseer);
                    }
                }
            }
            Err(e) => {
                eprintln!("parse_ar_tafseer: Error: {e}");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
