pub fn print_answer_status(answer_is_found: bool) {
    if answer_is_found {
        println!("you found the answer! 🚀");
    } else {
        println!("sorry... no answer found! 😬");
    }
}

pub fn read_input_file(file_name: &str) -> Vec<String> {
    std::fs::read_to_string(file_name)
        .unwrap()
        .split('\n')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}
