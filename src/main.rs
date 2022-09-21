use rand::prelude::*;
use clipboard_win::{set_clipboard, formats};
fn main() {
    let mut rng = rand::thread_rng();
    let mut res: Vec<char> = vec![];
    push_random_letter(&mut res, &mut rng);
    for _ in 0..5 {
        if rng.gen_bool(10. / 36.) {
            push_random_num(&mut res, &mut rng);
        } else {
            push_random_letter(&mut res, &mut rng);
        }
    }
    let res_string: String = res.into_iter().collect();

    set_clipboard(formats::Unicode, &res_string).unwrap();
}

fn push_random_letter(vec: &mut Vec<char>, rng: &mut ThreadRng) {
    let letter = rng.gen_range(b'a'..b'z') as char;
    vec.push(letter);
}
fn push_random_num(vec: &mut Vec<char>, rng: &mut ThreadRng) {
    let num = rng.gen_range(0..10).to_string().chars().next().unwrap();
    vec.push(num);
}
