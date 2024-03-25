use crate::cm_dtr_wd;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

pub struct Entro {
    word: String,
    value: f64,
}

impl PartialEq for Entro {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Entro {}

impl PartialOrd for Entro {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Entro {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn set_entropy(
    possible_try: &HashSet<String>,
    entropy: &mut BinaryHeap<Entro>,
    letters_tried: &[char; 26],
) {
    let vec = vec!['R', 'G', 'Y'];
    for word in possible_try.iter() {
        let mut e: f64 = 0.0;
        for first in vec.iter() {
            for second in vec.iter() {
                for third in vec.iter() {
                    for fourth in vec.iter() {
                        for fifth in vec.iter() {
                            let mut tmp = possible_try.clone();
                            let v = vec![[*first, *second, *third, *fourth, *fifth]];
                            cm_dtr_wd::set_possible_try(&v, &mut tmp, word, letters_tried);
                            if tmp.len() != 0 {
                                let p = tmp.len() as f64 / possible_try.len() as f64;
                                e += p * (1.0 / p).log2();
                            }
                        }
                    }
                }
            }
        }
        entropy.push(Entro {
            word: word.clone(),
            value: e,
        });
    }
}

pub fn print_entropy_suggest(entropy: &BinaryHeap<Entro>) {
    let mut num = 0;
    if entropy.len() > 5 {
        for item in entropy.iter() {
            println!(
                "Suggest Word{}: {}, value: {:.2}",
                num, item.word, item.value
            );
            if num > 4 {
                break;
            }
            num += 1;
        }
    } else {
        for item in entropy.into_iter() {
            println!(
                "Suggest Word{}: {}, value: {:.2}",
                num, item.word, item.value
            );
            num += 1;
        }
    }
}
