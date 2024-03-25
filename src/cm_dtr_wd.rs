use crate::builtin_words;
use crate::com;
use crate::com::Com;
use crate::functions_for_tui;
use console;
use rand::Rng;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub struct Wordle {
    pub is_tty: bool,
    pub correct_answer: String,
    pub success: bool,
    pub letters_guess_right: [bool; 5],
    pub letters_guess_exist: [bool; 5],
    pub letters_tried: [char; 26],
    pub vec_guess: Vec<[char; 5]>,
    pub vec_tried: Vec<[char; 26]>,
    pub final_words: HashSet<String>,
    pub acceptable_words: HashSet<String>,
}

impl Wordle {
    pub fn new() -> Self {
        let is_tty = atty::is(atty::Stream::Stdout);
        let correct_answer: String = String::new();
        let success: bool = false;
        let letters_guess_right: [bool; 5] = [false; 5];
        let letters_guess_exist: [bool; 5] = [false; 5];
        let letters_tried: [char; 26] = ['X'; 26];
        let vec_guess: Vec<[char; 5]> = Vec::new();
        let vec_tried: Vec<[char; 26]> = Vec::new();
        let final_words: HashSet<String> = HashSet::new();
        let acceptable_words: HashSet<String> = HashSet::new();
        Self {
            is_tty,
            correct_answer,
            success,
            letters_guess_right,
            letters_guess_exist,
            letters_tried,
            vec_guess,
            vec_tried,
            final_words,
            acceptable_words,
        }
    }

    //get correct_answer from standard input
    pub fn read_from_standard_input(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        io::stdin().read_line(&mut self.correct_answer)?;
        Ok(())
    }

    //get random answer
    pub fn random_answer(&mut self) {
        let mut vec: Vec<String> = self.final_words.clone().into_iter().collect();
        vec.sort();
        self.correct_answer = vec[rand::thread_rng().gen_range(1..=vec.len())]
            .to_uppercase()
            .trim()
            .to_string();
    }

    //get valid correct_answer
    pub fn get_correct_answer(
        &mut self,
        used_answers: &mut Vec<String>,
        com: &com::Com,
        set_num: &mut i32,
        w: &Vec<i32>,
    ) {
        if com.random {
            if com.seed != 0 {
                let mut vec: Vec<String> = self.final_words.clone().into_iter().collect();
                vec.sort();
                self.correct_answer = vec[w[*set_num as usize - 1] as usize - 1]
                    .to_uppercase()
                    .trim()
                    .to_string();
                *set_num += 1;
                return;
            } else {
                loop {
                    self.random_answer();
                    if !used_answers.contains(&self.correct_answer) {
                        break;
                    }
                }
                return;
            }
        }
        if com.word == "ungiven" {
            if self.is_tty {
                println!(
                    "{}",
                    console::style("Please give the correct answer:")
                        .bold()
                        .red()
                );
            }
            let a = self.read_from_standard_input();
            self.correct_answer = self.correct_answer.to_uppercase().trim().to_string();
            match a {
                Ok(()) => {
                    if !self.final_words.iter().any(|x| *x == self.correct_answer) {
                        println!(
                            "{}",
                            console::style("INVALID ANSWER! PLEASE TRY AGAIN.")
                                .bold()
                                .red()
                        );
                        self.get_correct_answer(used_answers, com, set_num, w)
                    }
                }
                Err(_) => {
                    println!(
                        "{}",
                        console::style("INVALID ANSWER! PLEASE TRY AGAIN.")
                            .bold()
                            .red()
                    );
                    self.get_correct_answer(used_answers, com, set_num, w)
                }
            }
        } else if !self
            .final_words
            .iter()
            .any(|x| *x == com.word.to_uppercase())
        {
            println!(
                "{}",
                console::style("INVALID ANSWER! PLEASE TRY AGAIN.")
                    .bold()
                    .red()
            );
            self.get_correct_answer(used_answers, com, set_num, w)
        } else {
            self.correct_answer = com.word.clone().to_uppercase().trim().to_string();
        }
    }

    pub fn tui_get_answer(
        &mut self,
        com: &com::Com,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        vec_guess: &Vec<[char; 5]>,
        letters_tried: &[char; 26],
        guessed: &Vec<String>,
    ) {
        self.correct_answer =
            functions_for_tui::tui_read(terminal, vec_guess, letters_tried, guessed);
        self.correct_answer = self.correct_answer.to_uppercase().trim().to_string();

        if !self.final_words.iter().any(|x| *x == self.correct_answer) {
            functions_for_tui::print_invalid(terminal, vec_guess, letters_tried, guessed);
            self.tui_get_answer(com, terminal, vec_guess, letters_tried, guessed);
        }
    }

    //get valid&acceptable guess
    pub fn get_guess_answer(&self, com: &com::Com) -> String {
        let mut tmp: String = String::new();
        if self.is_tty {
            println!("{}", console::style("Please guess answer:").bold().blue());
        }
        io::stdin().read_line(&mut tmp).unwrap();
        tmp = tmp.trim().to_string().to_uppercase();
        if self.acceptable_words.iter().any(|x| *x == tmp) {
            if !com.difficult {
                return tmp;
            } else {
                for i in 0..5 {
                    if self.letters_guess_right[i] {
                        if tmp.chars().nth(i) != self.correct_answer.chars().nth(i) {
                            if self.is_tty {
                                println!(
                                    "{}",
                                    console::style("INVALID ANSWER! PLEASE TRY AGAIN.")
                                        .bold()
                                        .red()
                                );
                            } else {
                                println!("INVALID");
                            }
                            tmp = self.get_guess_answer(com);
                            return tmp;
                        }
                    } else if self.letters_guess_exist[i] {
                        if !tmp.contains(self.correct_answer.chars().nth(i).unwrap()) {
                            if self.is_tty {
                                println!(
                                    "{}",
                                    console::style("INVALID ANSWER! PLEASE TRY AGAIN.")
                                        .bold()
                                        .red()
                                );
                            } else {
                                println!("INVALID");
                            }
                            tmp = self.get_guess_answer(com);
                            return tmp;
                        }
                    }
                }
                return tmp;
            }
        }
        if self.is_tty {
            println!(
                "{}",
                console::style("INVALID ANSWER! PLEASE TRY AGAIN.")
                    .bold()
                    .red()
            );
        } else {
            println!("INVALID");
        }
        tmp = self.get_guess_answer(com);
        return tmp;
    }

    //get valid&acceptable guess in tui mode
    pub fn get_guess_answer_tui(
        &self,
        com: &com::Com,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        vec_guess: &Vec<[char; 5]>,
        letters_tried: &[char; 26],
        guessed: &Vec<String>,
    ) -> String {
        let mut tmp: String =
            functions_for_tui::tui_read(terminal, vec_guess, letters_tried, guessed);
        tmp = tmp.trim().to_string().to_uppercase();
        if self.acceptable_words.iter().any(|x| *x == tmp) {
            if !com.difficult {
                return tmp;
            } else {
                for i in 0..5 {
                    if self.letters_guess_right[i] {
                        if tmp.chars().nth(i) != self.correct_answer.chars().nth(i) {
                            functions_for_tui::print_invalid(
                                terminal,
                                vec_guess,
                                letters_tried,
                                guessed,
                            );
                            tmp = self.get_guess_answer_tui(
                                com,
                                terminal,
                                vec_guess,
                                letters_tried,
                                guessed,
                            );
                            tmp = tmp.trim().to_string().to_uppercase();
                            return tmp;
                        }
                    } else if self.letters_guess_exist[i] {
                        if !tmp.contains(self.correct_answer.chars().nth(i).unwrap()) {
                            functions_for_tui::print_invalid(
                                terminal,
                                vec_guess,
                                letters_tried,
                                guessed,
                            );
                            tmp = self.get_guess_answer_tui(
                                com,
                                terminal,
                                vec_guess,
                                letters_tried,
                                guessed,
                            );
                            tmp = tmp.trim().to_string().to_uppercase();
                            return tmp;
                        }
                    }
                }
                return tmp;
            }
        }
        functions_for_tui::print_invalid(terminal, vec_guess, letters_tried, guessed);
        tmp = self.get_guess_answer_tui(com, terminal, vec_guess, letters_tried, guessed);
        tmp = tmp.trim().to_string().to_uppercase();
        return tmp;
    }

    //check if guess_answer is right and set letters_guess/tried and record guess status
    pub fn check_guess_answer(&mut self, guess_answer: &String, letters_guess: &mut [char; 5]) {
        let mut co_a: [bool; 5] = [false; 5];
        for i in 0..guess_answer.len() {
            let chars = guess_answer.chars().nth(i).unwrap();
            if guess_answer.chars().nth(i).unwrap() == self.correct_answer.chars().nth(i).unwrap() {
                letters_guess[i] = 'G';
                self.letters_guess_right[i] = true;
                self.letters_guess_exist[i] = true;
                self.letters_tried[(chars as usize) - ('A' as usize)] = 'G';
                co_a[i] = true;
            } else {
                {
                    if self.letters_tried[(chars as usize) - ('A' as usize)] == 'X' {
                        self.letters_tried[(chars as usize) - ('A' as usize)] = 'R';
                    }
                }
            }
        }
        let mut i: usize = 0;
        for chars in self.correct_answer.chars() {
            if !co_a[i] && guess_answer.contains(chars) {
                if letters_guess[guess_answer.find(chars).unwrap()] != 'G'
                    && letters_guess[guess_answer.find(chars).unwrap()] != 'Y'
                {
                    letters_guess[guess_answer.find(chars).unwrap()] = 'Y';
                    self.letters_guess_exist[i] = true;
                } else if letters_guess[guess_answer.rfind(chars).unwrap()] != 'G' {
                    letters_guess[guess_answer.rfind(chars).unwrap()] = 'Y';
                    self.letters_guess_exist[i] = true;
                }
                if self.letters_tried[(chars as usize) - ('A' as usize)] != 'G' {
                    self.letters_tried[(chars as usize) - ('A' as usize)] = 'Y';
                }
            }
            i += 1;
        }
        i = 0;
        for chars in guess_answer.chars() {
            if !co_a[i] {
                if self.letters_tried[(chars as usize) - ('A' as usize)] == 'X' {
                    self.letters_tried[(chars as usize) - ('A' as usize)] = 'R';
                }
            }
            i += 1;
        }
        self.vec_guess.push(letters_guess.clone());
        self.vec_tried.push(self.letters_tried.clone());
    }

    //guess state output
    pub fn print_guess_status(&self) {
        if self.is_tty {
            self.print_guess_state_tty();
        } else {
            self.print_guess_state_test();
        }
    }

    pub fn print_guess_state_test(&self) {
        let tmp = self.vec_guess.len() - 1;
        for ch in self.vec_guess[tmp] {
            print!("{}", ch);
        }
        print!(" ");
        for ch in self.vec_tried[tmp] {
            print!("{}", ch);
        }
        println!("");
    }

    pub fn print_guess_state_tty(&self) {
        for i in 0..self.vec_guess.len() {
            print!(
                "{}",
                console::style("guess".to_owned() + &(i + 1).to_string() + &": ".to_string())
                    .bold()
            );
            self.print_guess_tty(&self.vec_guess[i], &self.vec_tried[i]);
        }
    }

    pub fn print_guess_tty(&self, letters_guess: &[char; 5], letters_tried: &[char; 26]) {
        for ch in letters_guess.iter() {
            match *ch {
                'R' => print!("{}", console::style("R").red()),
                'G' => print!("{}", console::style("G").green()),
                'Y' => print!("{}", console::style("Y").yellow()),
                'X' => print!("{}", "X"),
                _ => (),
            }
        }
        print!(" ");
        for ch in letters_tried.iter() {
            match *ch {
                'R' => print!("{}", console::style("R").red()),
                'G' => print!("{}", console::style("G").green()),
                'Y' => print!("{}", console::style("Y").yellow()),
                'X' => print!("{}", "X"),
                _ => (),
            }
        }
        println!("");
    }

    //check final success and output
    pub fn check_final_success(&self) {
        match self.success {
            true => {
                if self.is_tty {
                    println!(
                        "{}",
                        console::style("CORRECT ".to_owned() + &self.vec_guess.len().to_string())
                            .bold()
                            .red()
                    );
                } else {
                    println!("CORRECT {}", self.vec_guess.len());
                }
            }
            false => {
                if self.is_tty {
                    println!(
                        "{}",
                        console::style("FAILED ".to_owned() + &self.correct_answer)
                            .bold()
                            .blue()
                    );
                } else {
                    println!("FAILED {}", self.correct_answer);
                }
            }
        }
    }

    // Adapted from https://medium.com/pragmatic-programmers/rustle-5c15d1c153a1 on 2023-07-02
    // get acceptavle words from given file or builtin words
    pub fn get_final_words(&mut self, com: &com::Com) {
        if com.final_set == "ungiven" {
            for item in builtin_words::FINAL.iter() {
                self.final_words.insert((*item).to_string().to_uppercase());
            }
            return;
        } else {
            let tmp: String = fs::read_to_string(&com.final_set).unwrap_or_default();
            for item in tmp.split('\n').into_iter() {
                let it: String = item
                    .trim()
                    .chars()
                    .filter(|c| c.is_ascii_alphabetic())
                    .collect();
                if it.len() == 5 {
                    self.final_words.insert(it.to_uppercase());
                }
            }
            return;
        }
    }

    pub fn get_acceptable_words(&mut self, com: &Com) {
        if com.acceptable_set == "ungiven" {
            for item in builtin_words::ACCEPTABLE.iter() {
                self.acceptable_words
                    .insert((*item).to_string().to_uppercase());
            }
            return;
        } else {
            let tmp: String = fs::read_to_string(&com.acceptable_set).unwrap_or_default();
            for item in tmp.split('\n').into_iter() {
                let it: String = item
                    .trim()
                    .chars()
                    .filter(|c| c.is_ascii_alphabetic())
                    .collect();
                if it.len() == 5 {
                    self.acceptable_words.insert(it.to_uppercase());
                }
            }
            return;
        }
    }

    pub fn sort_answer(&mut self) {
        let mut ve: Vec<String> = Vec::new();
        for item in self.final_words.iter() {
            if !self.acceptable_words.contains(item) {
                ve.push((*item).clone());
            }
        }
        for item in ve.iter() {
            self.final_words.remove(item);
        }
    }

    pub fn print_possible_try(&self, possible_try: &HashSet<String>) -> Result<(), io::Error> {
        let mut file = File::create("possible_try.txt")?;
        for item in possible_try {
            file.write_all(item.as_bytes())?;
            file.write_all(b"\n")?; // 写入换行符
        }

        Ok(())
    }
}

//meant for hint_acceptable
pub fn set_possible_try(
    vec_guess: &Vec<[char; 5]>,
    possible_try: &mut HashSet<String>,
    guess_answer: &String,
    letters_tried: &[char; 26],
) {
    let tmp = possible_try.clone();
    let mut num_ = 0;
    for i in vec_guess[vec_guess.len() - 1].iter() {
        let alph = guess_answer.chars().nth(num_).unwrap();
        if *i == 'Y' {
            for j in tmp.iter() {
                if j.chars().nth(num_).unwrap() == alph.clone() {
                    possible_try.remove(j);
                }
                if !j.contains(alph.clone()) {
                    possible_try.remove(j);
                }
            }
        } else if *i == 'G' {
            for j in tmp.iter() {
                if j.chars().nth(num_).unwrap() != alph.clone() {
                    possible_try.remove(j);
                }
            }
        } else if *i == 'R' {
            let t = alph as usize - 'A' as usize;
            for j in tmp.iter() {
                if j.chars().nth(num_).unwrap() == alph.clone() {
                    possible_try.remove(j);
                }
                if letters_tried[t] == 'R' {
                    if j.contains(alph) {
                        possible_try.remove(j);
                    }
                }
            }
        }
        num_ += 1;
    }
}
