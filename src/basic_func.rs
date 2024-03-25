use crate::com;
use crate::data_record;
use crate::functions;
use crate::use_json;
use text_io::read;

pub fn basic(
    com: &com::Com,
    games: &mut use_json::State,
    used_answers: &mut Vec<String>,
    dtr: &mut data_record::DataRecord,
    set_num: &mut i32,
    is_tty: bool,
) {
    loop {
        if com.state != "ungiven" {
            com.read_from_exist_json(&com.state, games);
        }

        functions::start_a_game(used_answers, dtr, &com, set_num, games);

        if com.state != "ungiven" {
            let t = com.write_to_json(&com.state, &games);
            t.unwrap();
        }

        if com.stats {
            dtr.calc();
            dtr.print_calc();
        }
        if com.word == "ungiven" {
            if is_tty {
                println!("Start a new game.(Y/N)");
            }
            let yes_or_no: String = read!();
            if yes_or_no != "Y" {
                break;
            }
        } else {
            break;
        }
    }
}
