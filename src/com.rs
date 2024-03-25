use crate::use_json;
use crate::use_json::State;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Com {
    #[arg(short = 'w', long, default_value_t =  ("ungiven").to_string())]
    pub word: String,

    #[arg(short = 'r', long, default_value_t = false)]
    pub random: bool,

    #[arg(short = 'D', long, default_value_t = false)]
    pub difficult: bool,

    #[arg(short = 't', long, default_value_t = false)]
    pub stats: bool,

    #[arg(short = 'd', long, default_value_t = 1)]
    pub day: i32,

    #[arg(short = 's', long, default_value_t = 0)]
    pub seed: u64,

    #[arg(short = 'f', long, default_value_t = ("ungiven").to_string())]
    pub final_set: String,

    #[arg(short = 'a', long, default_value_t = ("ungiven").to_string())]
    pub acceptable_set: String,

    #[arg(short = 'S', long, default_value_t = ("ungiven").to_string())]
    pub state: String,

    #[arg(short = 'c', long, default_value_t = ("ungiven").to_string())]
    pub config: String,

    #[arg(short = 'H', long, default_value_t = false)]
    pub hint_acceptable: bool,

    #[arg(short = 'e', long, default_value_t = false)]
    pub entropy_hint: bool,

    #[arg(short = 'u', long, default_value_t = false)]
    pub tui: bool,
}

impl Com {
    pub fn check(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.random && self.word != "ungiven".to_string() {
            Err(Box::<dyn std::error::Error>::from("-r while -w"))
        } else if self.word != "ungiven".to_string() && (self.day != 1 || self.seed != 0) {
            Err(Box::<dyn std::error::Error>::from("-w while -d/-s"))
        } else {
            Ok(())
        }
    }
    pub fn read_from_exist_json(&self, file_path: &str, games: &mut use_json::State) {
        if Path::new(file_path).exists() {
            let file = File::open(file_path).expect("creation-goes-wrong");
            let reader: BufReader<File> = BufReader::new(file);
            *games = serde_json::from_reader(reader).expect("reader-goes-wrong");
        } else {
            let file1 = File::create(file_path).expect("creation-goes-wrong");
            serde_json::to_writer_pretty(&file1, games).expect("nothing_written");
        }
    }

    pub fn write_to_json(
        &self,
        path: &str,
        game_stats: &State,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(&file, game_stats).unwrap();
        Ok(())
    }
}
