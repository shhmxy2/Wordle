use crate::use_json::State;

pub struct DataRecord {
    pub is_tty: bool,
    pub success: i32,
    pub failure: i32,
    pub success_need: i32,
    pub success_on_average: f64,
    pub guessed_answers: Vec<(String, i32)>,
}

impl DataRecord {
    pub fn new(is_tty_: &bool) -> Self {
        let is_tty = *is_tty_;
        let success: i32 = 0;
        let failure: i32 = 0;
        let success_need: i32 = 0;
        let success_on_average: f64 = 0.00;
        let guessed_answers: Vec<(String, i32)> = Vec::new();
        Self {
            is_tty,
            success,
            failure,
            success_need,
            success_on_average,
            guessed_answers,
        }
    }
    pub fn calc(&mut self) {
        if self.success != 0 {
            self.success_on_average = (self.success_need as f64) / (self.success as f64);
        }

        let num: usize = self.guessed_answers.len();
        for i in 0..num {
            for j in 0..i {
                if !compare(
                    self.guessed_answers[i].to_owned(),
                    self.guessed_answers[j].to_owned(),
                ) {
                    let tmp = self.guessed_answers[i].to_owned();
                    self.guessed_answers[i] = self.guessed_answers[j].to_owned();
                    self.guessed_answers[j] = tmp;
                }
            }
        }
    }
    pub fn print_calc(&self) {
        if self.is_tty {
            println!(
                "{}{} {}{} {}{:.2}",
                console::style("Success:").bold().red(),
                self.success,
                console::style("Failure:").bold().red(),
                self.failure,
                console::style("Success_on_average:").bold().red(),
                self.success_on_average
            );
        } else {
            println!(
                "{} {} {:.2}",
                self.success, self.failure, self.success_on_average
            );
        }
        let num: usize = self.guessed_answers.len();
        if num < 5 {
            for i in 0..num - 1 {
                print!(
                    "{} {} ",
                    self.guessed_answers[i].0, self.guessed_answers[i].1
                );
            }
            println!(
                "{} {}",
                self.guessed_answers[num - 1].0,
                self.guessed_answers[num - 1].1
            );
        } else {
            for i in 0..4 {
                print!(
                    "{} {} ",
                    self.guessed_answers[i].0, self.guessed_answers[i].1
                );
            }
            println!(
                "{} {}",
                self.guessed_answers[4].0, self.guessed_answers[4].1
            );
        }
    }
}

pub fn compare(right: (String, i32), left: (String, i32)) -> bool {
    if right.1 < left.1 {
        return true;
    }
    if right.1 > left.1 {
        return false;
    }
    if right.0 > left.0 {
        return true;
    }
    return false;
}

pub fn update_dtr(dtr: &mut DataRecord, games: &State) {
    for item in games.games.iter() {
        if item.guesses.contains(&item.answer) {
            dtr.success += 1;
            dtr.success_need += item.guesses.len() as i32;
        } else {
            dtr.failure += 1;
        }
        for subitem in item.guesses.iter() {
            if let Some(index) = dtr
                .guessed_answers
                .iter()
                .position(|(answer, _)| answer == subitem)
            {
                if let Some(entry) = dtr.guessed_answers.get_mut(index) {
                    entry.1 += 1;
                }
            } else {
                dtr.guessed_answers.push((subitem.clone(), 1));
            }
        }
    }
}
