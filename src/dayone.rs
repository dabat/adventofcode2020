/*
  part one.
  Find the two entries that sum to 2020; what do you get if you multiply them together?
  GOAL!!! 36:147 586 + 1434 == 2020
  the answer is 586 * 1434 == 840324


  part two
  what is the product of the three entries that sum to 2020?
  https://adventofcode.com/2020/day/1#part2

########## GOAL!!! ###############################
[
    910,
    903,
    207,
] == 2020
the answer is [
    910,
    903,
    207,
] == 170098110
##################################################
*/
use itertools::Itertools;

pub fn day_one_part_one() {
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
}

pub fn day_one_part_two() {
    let list = std::fs::read_to_string("day1_input.txt").unwrap();
    let vlist: Vec<i32> = list
        .split('\n')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    let mut answer_is_found = false;

    // iterate through the list grabbing three-item combinations and check for the answer
    let combinations = vlist.into_iter().combinations(3);

    'search: for chunk in combinations {
        if check_for_answer(chunk, 2020) {
            answer_is_found = true;
            break 'search;
        }
    }

    // iterate through the list selecting the next two items in each iteration
    // 'search: for (index_one, item) in vlist.iter().enumerate() {
    //     let item_one = item.parse::<i32>().unwrap_or(0);
    //     let mut index_two = index_one + 1;
    //     let mut index_three = index_two + 1;
    //     while index_three < vlist.len() {
    //         let item_two = vlist[index_two].parse().unwrap_or(0);
    //         let item_three = vlist[index_three].parse().unwrap_or(0);
    //         let sum = item_one + item_two + item_three;
    //         println!(
    //             "{}:{}:{} {} + {} + {} == {}",
    //             index_one, index_two, index_three, item_one, item_two, item_three, sum
    //         );
    //         if sum == 2020 {
    //             println!("{} GOAL!!! {}", "#".repeat(10), "#".repeat(31));
    //             println!(
    //                 "{}:{}:{} {} + {} + {} == {}",
    //                 index_one, index_two, index_three, item_one, item_two, item_three, sum
    //             );
    //             let product = item_one * item_two * item_three;
    //             println!(
    //                 "the answer is {} * {} * {} == {}",
    //                 item_one, item_two, item_three, product
    //             );
    //             println!("{}", "#".repeat(50));
    //             answer_is_found = true;
    //             break 'search;
    //         }
    //
    //         index_two += 1;
    //         index_three = index_two + 1;
    //     }
    // }

    // grab three at a time
    // while index_one < vlist.len() {
    //     let item_one = vlist[index_one].parse::<i32>().unwrap_or(0);
    //     let item_two = vlist[index_two].parse::<i32>().unwrap_or(0);
    //     let item_three = vlist[index_three].parse::<i32>().unwrap_or(0);
    //     let sum = item_one + item_two + item_three;
    //     println!(
    //         "{}:{}:{} {} + {} + {} == {}",
    //         index_one, index_two, index_three, item_one, item_two, item_three, sum
    //     );
    //     if sum == 2020 {
    //         println!("{} GOAL!!! {}", "#".repeat(10), "#".repeat(31));
    //         println!(
    //             "{}:{}:{} {} + {} + {} == {}",
    //             index_one, index_two, index_three, item_one, item_two, item_three, sum
    //         );
    //         let product = item_one * item_two * item_three;
    //         println!(
    //             "the answer is {} * {} * {} == {}",
    //             item_one, item_two, item_three, product
    //         );
    //         println!("{}", "#".repeat(50));
    //         answer_is_found = true;
    //         break;
    //     }
    //     index_two += 1;
    //
    //     index_one += 1;
    //     index_two = index_one + 1;
    // }

    // while index < vlist.len() && !answer_is_found {
    //     // println!("{}",index);
    //     let this_item = vlist[index].parse::<i32>().unwrap_or(0);
    //     // println!("{}:{} ",index,this_item);
    //     while next_index < vlist.len() && !answer_is_found {
    //         let next_item = vlist[next_index].parse::<i32>().unwrap_or(0);
    //         let sum = this_item + next_item;
    //         println!(
    //             "{}:{} {} + {} == {}",
    //             index, next_index, this_item, next_item, sum
    //         );
    //         if sum == 2020 {
    //             println!(
    //                 "GOAL!!! {}:{} {} + {} == 2020",
    //                 index, next_index, this_item, next_item
    //             );
    //             let product = this_item * next_item;
    //             println!("the answer is {} * {} == {}", this_item, next_item, product);
    //             answer_is_found = true;
    //         }
    //         next_index += 1;
    //     }
    //     index += 1;
    //     next_index = index + 1;
    // }

    if answer_is_found {
        println!("you found the answer! 🚀");
    } else {
        println!("sorry... no answer found! 😬");
    }
}

fn check_for_answer(list: Vec<i32>, answer: i32) -> bool {
    let sum: i32 = list.iter().sum();
    if sum == answer {
        println!("{} GOAL!!! {}", "#".repeat(10), "#".repeat(31));
        println!("the sum of these elements {:#?} == {}", list, sum);
        let product = list.iter().product::<i32>();
        println!("the answer from {:#?} is {}", list, product);
        println!("{}", "#".repeat(50));
        return true;
    }
    return false;
}
