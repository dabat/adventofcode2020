/*
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
use crate::utils::*;

pub fn find_answer() {
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
    println!("{}", "#".repeat(50));
    println!("passing count: {}", passing_count);
    println!("failing count: {}", failing_count);
    println!("{}", "#".repeat(50));
}
