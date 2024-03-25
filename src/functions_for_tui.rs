use crate::cm_dtr_wd;
use crate::cm_dtr_wd::Wordle;
use crate::com;
use crate::data_record;
use crate::entropy;
use crate::use_json;
use crate::use_json::Games;

use crossterm::event;
use crossterm::event::{Event, KeyCode};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::collections::{BinaryHeap, HashSet};
use std::io;
use std::thread;
use std::time::Duration;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

pub fn draw_terminal_blank(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) {
    // Adapted from https://docs.rs/tui/latest/tui/ on 2023-07-06
    // draw blocks and paragraphs
    terminal
        .draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(3),
                        Constraint::Percentage(60),
                        Constraint::Percentage(7),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let block = Block::default()
                .title("Wordle")
                .borders(Borders::TOP)
                .style(
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                )
                .title_alignment(Alignment::Center);
            f.render_widget(block, chunks[0]);

            let block = Block::default().title("keyboard").borders(Borders::ALL);
            f.render_widget(block, chunks[3]);

            let chunks_row = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(8),
                        Constraint::Percentage(8),
                        Constraint::Percentage(8),
                        Constraint::Percentage(8),
                        Constraint::Percentage(8),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(chunks[1]);

            let mut chunks_block: Vec<Vec<tui::layout::Rect>> = Vec::new();

            for i in 0..5 {
                let row = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(3),
                            Constraint::Percentage(16),
                            Constraint::Percentage(16),
                            Constraint::Percentage(16),
                            Constraint::Percentage(16),
                            Constraint::Percentage(16),
                            Constraint::Percentage(16),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_row[i + 1]);

                chunks_block.push(row);
            }

            for i in 0..5 {
                for j in 0..6 {
                    let block = Block::default().borders(Borders::ALL);
                    f.render_widget(block, chunks_block[i][j + 1]);
                }
            }

            let chunks_keyboard_row = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(35),
                        Constraint::Percentage(35),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(chunks[3]);

            let mut chunks_keyboard_block: Vec<Vec<tui::layout::Rect>> = Vec::new();
            for i in 0..3 {
                let row = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(27),
                            Constraint::Percentage(5),
                            Constraint::Percentage(5),
                            Constraint::Percentage(5),
                            Constraint::Percentage(5),
                            Constraint::Percentage(5),
                            Constraint::Percentage(5),
                            Constraint::Percentage(5),
                            Constraint::Percentage(5),
                            Constraint::Percentage(5),
                            Constraint::Percentage(27),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_keyboard_row[i]);

                chunks_keyboard_block.push(row);
            }

            for i in 0..9 {
                for j in 0..3 {
                    let m = i + 9 * j;
                    if m == 26 {
                        break;
                    }
                    let character = (m as u8 + 'A' as u8) as char;
                    let block = Block::default().borders(Borders::ALL);
                    f.render_widget(block, chunks_keyboard_block[j][i + 1]);
                    let text =
                        Text::styled(character.to_string(), Style::default().fg(Color::Black));
                    let paragraph =
                        Paragraph::new(text).block(Block::default().borders(Borders::ALL));
                    f.render_widget(paragraph, chunks_keyboard_block[j][i + 1]);
                }
                let chunks_read = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(3),
                            Constraint::Percentage(59),
                            Constraint::Percentage(9),
                            Constraint::Percentage(29),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                let block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::LightBlue));
                f.render_widget(block, chunks_read[2]);
            }
        })
        .unwrap();
}

pub fn draw_inst(
    f: &mut tui::Frame<'_, CrosstermBackend<io::Stdout>>,
    vec_guess: &Vec<[char; 5]>,
    letters_tried: &[char; 26],
    guessed: &Vec<String>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(3),
                Constraint::Percentage(60),
                Constraint::Percentage(7),
                Constraint::Percentage(30),
            ]
            .as_ref(),
        )
        .split(f.size());

    let block = Block::default()
        .title("Wordle")
        .borders(Borders::TOP)
        .style(
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .title_alignment(Alignment::Center);
    f.render_widget(block, chunks[0]);

    let block = Block::default().title("keyboard").borders(Borders::ALL);
    f.render_widget(block, chunks[3]);

    let chunks_row = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(8),
                Constraint::Percentage(8),
                Constraint::Percentage(8),
                Constraint::Percentage(8),
                Constraint::Percentage(8),
                Constraint::Percentage(30),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let mut chunks_block: Vec<Vec<tui::layout::Rect>> = Vec::new();

    for i in 0..5 {
        let row = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(3),
                    Constraint::Percentage(16),
                    Constraint::Percentage(16),
                    Constraint::Percentage(16),
                    Constraint::Percentage(16),
                    Constraint::Percentage(16),
                    Constraint::Percentage(16),
                ]
                .as_ref(),
            )
            .split(chunks_row[i + 1]);

        chunks_block.push(row);
    }

    let mut num = 0;
    for i in guessed.iter() {
        for j in 0..5 {
            let character = i.chars().nth(j).unwrap();
            if vec_guess[num][j] == 'G' {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Green));
                f.render_widget(block, chunks_block[j][num + 1]);
            } else if vec_guess[num][j] == 'R' {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Red));
                f.render_widget(block, chunks_block[j][num + 1]);
            } else if vec_guess[num][j] == 'Y' {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Yellow));
                f.render_widget(block, chunks_block[j][num + 1]);
            }
            let text = Text::styled(character.to_string(), Style::default().fg(Color::Black));
            let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
            f.render_widget(paragraph, chunks_block[j][num + 1]);
        }
        num += 1;
    }
    if num != 0 {
        num -= 1;
    }

    for i in 0..5 {
        for j in num..7 {
            let block = Block::default().borders(Borders::ALL);
            f.render_widget(block, chunks_block[i][j]);
        }
    }

    let chunks_keyboard_row = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(35),
                Constraint::Percentage(35),
                Constraint::Percentage(30),
            ]
            .as_ref(),
        )
        .split(chunks[3]);

    let mut chunks_keyboard_block: Vec<Vec<tui::layout::Rect>> = Vec::new();
    for i in 0..3 {
        let row = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(27),
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                    Constraint::Percentage(27),
                ]
                .as_ref(),
            )
            .split(chunks_keyboard_row[i]);

        chunks_keyboard_block.push(row);
    }

    for i in 0..9 {
        for j in 0..3 {
            let m = i + 9 * j;
            if m == 26 {
                break;
            }
            let character = (m as u8 + 'A' as u8) as char;
            if letters_tried[m] == 'G' {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Green));
                f.render_widget(block, chunks_keyboard_block[j][i + 1]);
            } else if letters_tried[m] == 'R' {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Red));
                f.render_widget(block, chunks_keyboard_block[j][i + 1]);
            } else if letters_tried[m] == 'Y' {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Yellow));
                f.render_widget(block, chunks_keyboard_block[j][i + 1]);
            } else if letters_tried[m] == 'X' {
                let block = Block::default().borders(Borders::ALL);
                f.render_widget(block, chunks_keyboard_block[j][i + 1]);
            }
            let text = Text::styled(character.to_string(), Style::default().fg(Color::Black));
            let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
            f.render_widget(paragraph, chunks_keyboard_block[j][i + 1]);

            let chunks_read = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(3),
                        Constraint::Percentage(59),
                        Constraint::Percentage(9),
                        Constraint::Percentage(29),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::LightBlue));
            f.render_widget(block, chunks_read[2]);
        }
    }
}

pub fn draw_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    vec_guess: &Vec<[char; 5]>,
    letters_tried: &[char; 26],
    guessed: &Vec<String>,
) {
    terminal
        .draw(|f: &mut tui::Frame<'_, CrosstermBackend<io::Stdout>>| {
            draw_inst(f, vec_guess, letters_tried, guessed);
        })
        .unwrap();
}

pub fn easy_read(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, txt: String) -> String {
    let mut input: String = String::new();
    terminal
        .draw(|f: &mut tui::Frame<'_, CrosstermBackend<io::Stdout>>| {
            let chunks_read = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(3),
                        Constraint::Percentage(59),
                        Constraint::Percentage(9),
                        Constraint::Percentage(29),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            let text = Text::styled(txt, Style::default().fg(Color::Black));
            let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
            f.render_widget(paragraph, chunks_read[2]);
        })
        .unwrap();
    // Adapted from https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs on 2023-07-06
    // read keys from keyboard
    if let Event::Key(key) = event::read().unwrap() {
        match key.code {
            KeyCode::Char(c) => {
                input.push(c);
            }
            _ => {}
        }
    }
    return input;
}

pub fn tui_read(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    vec_guess: &Vec<[char; 5]>,
    letters_tried: &[char; 26],
    guessed: &Vec<String>,
) -> String {
    let mut input: String = String::new();
    loop {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Char(c) => {
                    input.push(c);
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                _ => {}
            }
        }
        terminal
            .draw(|f: &mut tui::Frame<'_, CrosstermBackend<io::Stdout>>| {
                draw_inst(f, vec_guess, letters_tried, guessed);

                let chunks_read = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(3),
                            Constraint::Percentage(59),
                            Constraint::Percentage(9),
                            Constraint::Percentage(29),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                let text = Text::styled(input.to_string(), Style::default().fg(Color::Black));
                let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
                f.render_widget(paragraph, chunks_read[2]);
            })
            .unwrap();
    }
    input = input.to_uppercase();
    return input;
}

pub fn print_invalid(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    vec_guess: &Vec<[char; 5]>,
    letters_tried: &[char; 26],
    guessed: &Vec<String>,
) {
    terminal
        .draw(|f| {
            draw_inst(f, vec_guess, letters_tried, guessed);

            let chunks_invalid_1 = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(20),
                        Constraint::Percentage(40),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let chunks_invalid = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(20),
                        Constraint::Percentage(40),
                    ]
                    .as_ref(),
                )
                .split(chunks_invalid_1[1]);

            let block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            f.render_widget(block, chunks_invalid[1]);
            let text = Text::styled("INVALID".to_string(), Style::default().fg(Color::Black));
            let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
            f.render_widget(paragraph, chunks_invalid[1]);
        })
        .unwrap();
    thread::sleep(Duration::from_millis(1500));
    draw_terminal(terminal, vec_guess, letters_tried, guessed);
}

pub fn easy_out(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, txt: String) {
    terminal
        .draw(|f: &mut tui::Frame<'_, CrosstermBackend<io::Stdout>>| {
            let chunks_read = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(3),
                        Constraint::Percentage(59),
                        Constraint::Percentage(9),
                        Constraint::Percentage(29),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            let text = Text::styled(txt, Style::default().fg(Color::Green));
            let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
            f.render_widget(paragraph, chunks_read[2]);
        })
        .unwrap();

    thread::sleep(Duration::from_millis(100));
}

pub fn check_final_success(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    correct_answer: &String,
    success: bool,
    times: usize,
) {
    match success {
        true => {
            easy_out(terminal, "CORRECT ".to_string() + &times.to_string());
        }
        false => {
            easy_out(terminal, "FAILURE ".to_string() + correct_answer);
        }
    }
}

pub fn start_a_game_tui(
    used_answers: &mut Vec<String>,
    dtr: &mut data_record::DataRecord,
    com: &com::Com,
    set_num: &mut i32,
    mut games: &mut use_json::State,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) {
    draw_terminal_blank(terminal);
    games.total_rounds += 1;
    let mut game_record: Games = Games::new();

    let mut wordle = Wordle::new();
    wordle.get_final_words(com);
    wordle.get_acceptable_words(com);
    wordle.sort_answer();

    let mut possible_try: HashSet<String> = wordle.acceptable_words.clone();
    let num = wordle.final_words.len();
    let mut w: Vec<i32> = (1..=(num as i32)).collect();
    if com.random && com.seed != 0 {
        let mut rng: StdRng = SeedableRng::seed_from_u64(com.seed);
        w.shuffle(&mut rng);
    }

    let mut guessed: Vec<String> = Vec::new();

    if com.random || com.word != "ungiven" {
        wordle.get_correct_answer(used_answers, com, set_num, &w);
    } else {
        let tmp: Vec<[char; 5]> = Vec::new();
        let lt: [char; 26] = ['X'; 26];
        wordle.tui_get_answer(com, terminal, &tmp, &lt, &guessed);
        draw_terminal_blank(terminal);
    }

    used_answers.push(wordle.correct_answer.clone());
    game_record.answer = wordle.correct_answer.clone();

    while wordle.vec_guess.len() < 6 {
        let guess_answer: String = wordle.get_guess_answer_tui(
            com,
            terminal,
            &wordle.vec_guess,
            &wordle.letters_tried,
            &guessed,
        );
        guessed.push(guess_answer.clone());
        game_record.guesses.push(guess_answer.clone());

        if let Some(index) = dtr
            .guessed_answers
            .iter()
            .position(|(answer, _)| answer == &guess_answer)
        {
            if let Some(entry) = dtr.guessed_answers.get_mut(index) {
                entry.1 += 1;
            }
        } else {
            dtr.guessed_answers.push((guess_answer.clone(), 1));
        }

        let mut letters_guess: [char; 5] = ['R'; 5];
        wordle.check_guess_answer(&guess_answer, &mut letters_guess);
        cm_dtr_wd::set_possible_try(
            &wordle.vec_guess,
            &mut possible_try,
            &guess_answer,
            &wordle.letters_tried,
        );
        if com.hint_acceptable {
            let t = wordle.print_possible_try(&possible_try);
            match t {
                Ok(()) => (),
                Err(_) => println!("Fail to print possible try"),
            }
        }

        draw_terminal(terminal, &wordle.vec_guess, &wordle.letters_tried, &guessed);

        if com.entropy_hint {
            let mut entropy: BinaryHeap<entropy::Entro> = BinaryHeap::new();
            entropy::set_entropy(&possible_try, &mut entropy, &wordle.letters_tried);
            entropy::print_entropy_suggest(&entropy);
        }

        if guess_answer == wordle.correct_answer {
            wordle.success = true;
            thread::sleep(Duration::from_millis(100));
            break;
        }

        if wordle.vec_guess.len() == 6 {
            thread::sleep(Duration::from_millis(100));
        }
    }

    if wordle.success {
        dtr.success += 1;
        dtr.success_need += wordle.vec_guess.len() as i32;
    } else {
        dtr.failure += 1;
    }

    check_final_success(
        terminal,
        &wordle.correct_answer,
        wordle.success,
        wordle.vec_guess.len(),
    );

    games.games.push(game_record);
    thread::sleep(Duration::from_millis(1500));
}
