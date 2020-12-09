/*
--- Day 2: Password Philosophy ---

Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc

Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?
https://adventofcode.com/2020/day/2
*/
use crate::utils::*;

pub fn find_answer() {
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
    println!("passing count: {}", passing_count);
    println!("failing count: {}", failing_count);
}
