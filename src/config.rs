use std::{fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub random: bool,

    #[serde(default)]
    pub difficult: bool,

    #[serde(default)]
    pub stats: bool,

    #[serde(default)]
    pub day: i32,

    #[serde(default)]
    pub seed: u64,

    #[serde(default)]
    pub final_set: String,

    #[serde(default)]
    pub acceptable_set: String,

    #[serde(default)]
    pub state: String,

    #[serde(default)]
    pub word: String,
}

impl Config {
    pub fn new() -> Self {
        let random = false;
        let difficult = false;
        let stats = false;
        let day = 1;
        let seed = 0;
        let final_set = "ungiven".to_string();
        let acceptable_set = "ungiven".to_string();
        let state = "ungiven".to_string();
        let word = "ungiven".to_string();
        Self {
            random,
            difficult,
            stats,
            day,
            seed,
            final_set,
            acceptable_set,
            state,
            word,
        }
    }
}

pub fn read_from_exist_json(file_path: &str, conf: &mut Config) {
    if Path::new(file_path).exists() {
        let file = File::open(file_path).expect("creation-goes-wrong");
        let reader: BufReader<File> = BufReader::new(file);
        *conf = serde_json::from_reader(reader).expect("reader-goes-wrong");
    }
}
