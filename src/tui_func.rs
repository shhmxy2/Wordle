use crate::com;
use crate::data_record;
use crate::functions_for_tui;
use crate::use_json;

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::thread;
use std::time::Duration;
use tui::{backend::CrosstermBackend, Terminal};

pub fn tui(
    com: &com::Com,
    games: &mut use_json::State,
    used_answers: &mut Vec<String>,
    dtr: &mut data_record::DataRecord,
    set_num: &mut i32,
) -> Result<(), io::Error> {
    // Copied from https://docs.rs/tui/latest/tui/ on 2023-07-06
    // the five line below; setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        functions_for_tui::draw_terminal_blank(&mut terminal);

        if com.state != "ungiven" {
            com.read_from_exist_json(&com.state, games);
        }

        functions_for_tui::start_a_game_tui(used_answers, dtr, &com, set_num, games, &mut terminal);

        if com.state != "ungiven" {
            let t = com.write_to_json(&com.state, &games);
            t.unwrap();
        }

        if com.stats {
            dtr.calc();
            let out: String = "Success: ".to_owned()
                + &dtr.success.to_string()
                + &"  Failure :".to_string()
                + &dtr.failure.to_string()
                + &"  Success_on_average: ".to_string()
                + &dtr.success_on_average.to_string();
            functions_for_tui::easy_out(&mut terminal, out);
            thread::sleep(Duration::from_millis(1000));
            let num: usize = dtr.guessed_answers.len();
            let mut out2 = String::new();
            if num < 5 {
                for i in 0..num {
                    out2 = out2.to_owned()
                        + &dtr.guessed_answers[i].0
                        + &" ".to_string()
                        + &dtr.guessed_answers[i].1.to_string()
                        + &" ".to_string();
                }
            } else {
                for i in 0..5 {
                    out2 = out2.to_owned()
                        + &dtr.guessed_answers[i].0
                        + &" ".to_string()
                        + &dtr.guessed_answers[i].1.to_string()
                        + &" ".to_string();
                }
            }
            functions_for_tui::easy_out(&mut terminal, out2);
            thread::sleep(Duration::from_millis(1000));
        }
        if com.word == "ungiven" {
            let yes_or_no: String =
                functions_for_tui::easy_read(&mut terminal, "Start a new game.(Y/N)".to_string());
            if yes_or_no != "Y" {
                break;
            }
        } else {
            break;
        }
    }

    // Copied from https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs on 2023-07-06
    // the five line below; restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
