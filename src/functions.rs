use crate::cm_dtr_wd;
use crate::cm_dtr_wd::Wordle;
use crate::com;
use crate::config::Config;
use crate::data_record;
use crate::entropy;
use crate::use_json;
use crate::use_json::Games;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::collections::BinaryHeap;
use std::collections::HashSet;

pub fn start_a_game(
    used_answers: &mut Vec<String>,
    dtr: &mut data_record::DataRecord,
    com: &com::Com,
    set_num: &mut i32,
    mut games: &mut use_json::State,
) {
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

    wordle.get_correct_answer(used_answers, com, set_num, &w);
    used_answers.push(wordle.correct_answer.clone());
    game_record.answer = wordle.correct_answer.clone();

    while wordle.vec_guess.len() < 6 {
        let guess_answer: String = wordle.get_guess_answer(com);
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
        wordle.print_guess_status();

        if com.entropy_hint {
            let mut entropy: BinaryHeap<entropy::Entro> = BinaryHeap::new();
            entropy::set_entropy(&possible_try, &mut entropy, &wordle.letters_tried);
            entropy::print_entropy_suggest(&entropy);
        }

        if guess_answer == wordle.correct_answer {
            wordle.success = true;
            break;
        }
    }

    if wordle.success {
        dtr.success += 1;
        dtr.success_need += wordle.vec_guess.len() as i32;
    } else {
        dtr.failure += 1;
    }

    wordle.check_final_success();

    games.games.push(game_record);
}

pub fn update_com(com: &mut com::Com, conf: &Config) {
    if com.day == 1 && conf.day != 1 {
        com.day = conf.day;
    }

    if com.seed == 0 && conf.seed != 0 {
        com.seed = conf.seed;
    }

    if com.random == false && conf.random == true {
        com.random = true;
    }

    if com.difficult == false && conf.difficult == true {
        com.difficult = true;
    }

    if com.stats == false && conf.stats == true {
        com.stats = true;
    }

    if com.final_set == "ungiven".to_string()
        && conf.final_set != "ungiven".to_string()
        && !conf.final_set.is_empty()
    {
        com.final_set = conf.final_set.clone();
    }

    if com.acceptable_set == "ungiven".to_string()
        && conf.acceptable_set != "ungiven".to_string()
        && !conf.acceptable_set.is_empty()
    {
        com.acceptable_set = conf.acceptable_set.clone();
    }

    if com.state == "ungiven".to_string()
        && conf.state != "ungiven".to_string()
        && !conf.state.is_empty()
    {
        com.state = conf.state.clone();
    }

    if com.word == "ungiven".to_string()
        && conf.word != "ungiven".to_string()
        && !conf.word.is_empty()
    {
        com.word = conf.word.clone();
    }
}
