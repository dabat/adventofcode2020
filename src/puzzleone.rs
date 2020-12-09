/*
  day one puzzle one.
  Find the two entries that sum to 2020; what do you get if you multiply them together?
*/
pub fn find_answer() {
    let list = std::fs::read_to_string("day1_input.txt").unwrap();
    let vlist: Vec<&str> = list
        .split('\n')
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();
    let mut index = 0;
    let mut next_index = 1;
    let mut answer_is_found = false;
    while index < vlist.len() && !answer_is_found {
        let this_item = vlist[index].parse::<i32>().unwrap_or(0);
        while next_index < vlist.len() && !answer_is_found {
            let next_item = vlist[next_index].parse::<i32>().unwrap_or(0);
            let sum = this_item + next_item;
            println!(
                "{}:{} {} + {} == {}",
                index, next_index, this_item, next_item, sum
            );
            if sum == 2020 {
                println!(
                    "GOAL!!! {}:{} {} + {} == 2020",
                    index, next_index, this_item, next_item
                );
                let product = this_item * next_item;
                println!("the answer is {} * {} == {}", this_item, next_item, product);
                answer_is_found = true;
            }
            next_index += 1;
        }
        index += 1;
        next_index = index + 1;
    }

    if !answer_is_found {
        println!("uh oh... no winner found 😬");
    }

    /*
    GOAL!!! 36:147 586 + 1434 == 2020
    the answer is 586 * 1434 == 840324
    */
}
