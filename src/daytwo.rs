/*
--- Day 2: Password Philosophy ---

Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
"Something's wrong with our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted:
some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database)
and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc

Each line gives the password policy and then the password.
The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid.
For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid.
The middle password, cdefg, is not; it contains no instances of b, but needs at least 1.
The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?
https://adventofcode.com/2020/day/2

-- part two --
Each policy actually describes two positions in the password,
where 1 means the first character, 2 means the second character, and so on.
(Be careful; Toboggan Corporate Policies have no concept of "index zero"!)
Exactly one of these positions must contain the given letter.
Other occurrences of the letter are irrelevant for the purposes of policy enforcement.

Given the same example list from above:

    1-3 a: abcde is valid: position 1 contains a and position 3 does not.
    1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
    2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.

How many passwords are valid according to the new interpretation of the policies?
https://adventofcode.com/2020/day/2#part2
##################################################
passing count: 708
failing count: 292
##################################################
*/
#![allow(unused)]
use crate::utils::*;
#[allow(unused_variables)]
pub fn day2_part1() {
    // import the input file and define the passing counter
    let list = read_input_file("day2_input.txt");
    let mut passing_count = 0;
    let mut failing_count = 0;
    // iterate through the list and
    for item in list {
        // split each line into the three parts: min_max, character, test_value
        let (min, max, character, test_value) = split_item(&item);
        // test each test_value against the character and the min and max numbers
        let match_count = test_value.matches(&character).count();
        println!(
            "{} {} {} {} --> {}",
            min, max, character, test_value, match_count
        );
        // if the test_value meets the criteria then increment the counter of passing test_values
        if match_count >= min.parse::<usize>().unwrap_or(0)
            && match_count <= max.parse::<usize>().unwrap_or(0)
        {
            passing_count += 1;
        } else {
            failing_count += 1;
        }
    }
    // print out the results
    print_answer(
        2,
        1,
        "How many passwords are valid according to their policies?",
        &passing_count.to_string(),
    );
}

#[allow(unused_variables)]
pub fn day2_part2() {
    // import the input file and define the passing counter
    let list = read_input_file("day2_input.txt");
    let mut passing_count = 0;
    let mut failing_count = 0;

    // iterate through the list and
    for item in list {
        // split each line into the four parts: position_one, position_two, character, test_value
        let (position_one, position_two, character, test_value) = split_item(&item);
        // check position_one and position_two for matches of character in test_value
        let position1 = position_one.parse::<usize>().unwrap() - 1;
        let instance_one = test_value.get(position1..=position1);
        let position2 = position_two.parse::<usize>().unwrap() - 1;
        let instance_two = test_value.get(position2..=position2);
        println!(
            "{}=={:?} {}=={:?}",
            position1,
            instance_one.unwrap_or_default(),
            position2,
            instance_two.unwrap_or_default()
        );
        let mut match_count = 0;
        if instance_one.unwrap_or_default() == character {
            match_count += 1;
        }
        if instance_two.unwrap_or_default() == character {
            match_count += 1;
        }
        println!(
            "{} {} {} {} --> {}",
            test_value, character, position_one, position_two, match_count
        );
        // if the test_value has character in one of the positions then increment passing_count
        // else increment failing_count
        if match_count == 1 {
            passing_count += 1;
        } else {
            failing_count += 1;
        }
    }
    // print out the results
    print_answer(
        2,
        1,
        "How many passwords are valid according to the new interpretation of the policies?",
        &passing_count.to_string(),
    );
}
