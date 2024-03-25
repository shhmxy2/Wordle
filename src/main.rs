use clap::Parser;
mod basic_func;
mod builtin_words;
mod cm_dtr_wd;
mod com;
mod config;
mod data_record;
mod entropy;
mod functions;
mod functions_for_tui;
mod tui_func;
mod use_json;

/// The main function for the Wordle game
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);
    let mut com = com::Com::parse();
    let mut games = use_json::State::new();

    if com.config != "ungiven" {
        let mut conf = config::Config::new();
        config::read_from_exist_json(&com.config, &mut conf);
        functions::update_com(&mut com, &conf);
    }

    let check = com.check();
    match check {
        Ok(()) => (),
        Err(_) => return Err(Box::<dyn std::error::Error>::from("check_wrong")),
    }

    let mut dtr = data_record::DataRecord::new(&is_tty);
    if com.state != "ungiven" {
        com.read_from_exist_json(&com.state, &mut games);
    } else {
        dtr.failure -= 1;
    }
    data_record::update_dtr(&mut dtr, &games);
    let mut used_answers: Vec<String> = Vec::new();
    let mut set_num = com.day;
    if com.tui {
        tui_func::tui(&com, &mut games, &mut used_answers, &mut dtr, &mut set_num)?;
    } else {
        basic_func::basic(
            &com,
            &mut games,
            &mut used_answers,
            &mut dtr,
            &mut set_num,
            is_tty,
        );
    }

    Ok(())
}
