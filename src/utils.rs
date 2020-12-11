pub fn print_answer(answer: &str) {
    println!("{}", "#".repeat(50));
    if !answer.is_empty() {
        println!("the answer you found! 🚀 ... {}", answer);
    } else {
        println!("sorry... no answer found! 😬");
    }
    println!("{}", "#".repeat(50));
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

pub fn split_item(item: &str) -> (String, String, String, String) {
    let item_list: Vec<&str> = item.split(" ").collect();
    let min_max: Vec<String> = item_list[0].split("-").map(String::from).collect();
    let min: String = min_max[0].to_owned();
    let max: String = min_max[1].to_owned();
    let character: String = item_list[1].replace(":", "");
    let test_value: String = item_list[2].to_owned();
    (min, max, character, test_value)
}
