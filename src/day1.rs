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
#![allow(unused)]
use crate::utils::print_answer;
use itertools::Itertools;

pub fn day1_part1() {
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
            // println!(
            //     "{}:{} {} + {} == {}",
            //     index, next_index, this_item, next_item, sum
            // );
            if sum == 2020 {
                // println!(
                //     "GOAL!!! {}:{} {} + {} == 2020",
                //     index, next_index, this_item, next_item
                // );
                let product = this_item * next_item;
                let answer = format!("{} * {} == {}", this_item, next_item, product);
                // println!("the answer is {} * {} == {}", this_item, next_item, product);
                print_answer(1, 1, "Find the two entries that sum to 2020; what do you get if you multiply them together?", &answer.to_string());
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

pub fn day1_part2() {
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

    if !answer_is_found {
        println!("sorry... no answer found! 😬");
    }
}

fn check_for_answer(list: Vec<i32>, answer: i32) -> bool {
    let sum: i32 = list.iter().sum();
    if sum == answer {
        let product = list.iter().product::<i32>();
        let answer = format!(
            "{:#?} sum to {} and their product is {}",
            list, sum, product
        );
        print_answer(
            1,
            2,
            "what is the product of the three entries that sum to 2020?",
            &answer,
        );
        return true;
    }
    return false;
}
