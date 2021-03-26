/*
--- Day 9: Encoding Error ---

With your neighbor happily enjoying their video game, you turn your attention to an open data port on the little screen in the seat in front of you.

Though the port is non-standard, you manage to connect it to your computer through the clever use of several paperclips.
Upon connection, the port outputs a series of numbers (your puzzle input).

The data appears to be encrypted with the eXchange-Masking Addition System (XMAS) which, conveniently for you, is an old cypher with an important weakness.

XMAS starts by transmitting a preamble of 25 numbers. After that, each number you receive should be the sum of any two of the 25 immediately previous numbers.
The two numbers will have different values, and there might be more than one such pair.

For example, suppose your preamble consists of the numbers 1 through 25 in a random order. To be valid, the next number must be the sum of two of those numbers:

    26 would be a valid next number, as it could be 1 plus 25 (or many other pairs, like 2 and 24).
    49 would be a valid next number, as it is the sum of 24 and 25.
    100 would not be valid; no two of the previous 25 numbers sum to 100.
    50 would also not be valid; although 25 appears in the previous 25 numbers, the two numbers in the pair must be different.

Suppose the 26th number is 45, and the first number (no longer an option, as it is more than 25 numbers ago) was 20.
Now, for the next number to be valid, there needs to be some pair of numbers among 1-19, 21-25, or 45 that add up to it:

    26 would still be a valid next number, as 1 and 25 are still within the previous 25 numbers.
    65 would not be valid, as no two of the available numbers sum to it.
    64 and 66 would both be valid, as they are the result of 19+45 and 21+45 respectively.

Here is a larger example which only considers the previous 5 numbers (and has a preamble of length 5):

35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576

In this example, after the 5-number preamble, almost every number is the sum of two of the previous 5 numbers;
the only number that does not follow this rule is 127.

The first step of attacking the weakness in the XMAS data is to find the first number in the list
    (after the preamble) which is not the sum of two of the 25 numbers before it.
What is the first number that does not have this property?
542529149

--- Part Two ---
The final step in breaking the XMAS encryption relies on the invalid number you just found:
you must find a contiguous set of at least two numbers in your list which sum to the invalid number from step 1.

Again consider the above example:

35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576

In this list, adding up all of the numbers from 15 through 40 produces the invalid number from step 1, 127.
(Of course, the contiguous set of numbers in your actual list might be much longer.)

To find the encryption weakness, add together the smallest and largest number in this contiguous range;
in this example, these are 15 and 47, producing 62.

What is the encryption weakness in your XMAS-encrypted list of numbers?
75678618
*/
#![allow(unused)]
use std::vec;

use crate::utils::*;
use itertools::Itertools;

pub fn day9_part1(verbose: bool) {
    let mut searcher = Searcher::new( 1,
       "The first step of attacking the weakness in the XMAS data is to find the first number in the list (after the preamble) which is not the sum of two of the 25 numbers before it.\nWhat is the first number that does not have this property?".to_string(),
       None, Some(true))
       .find_outlier();
}

pub fn day9_part2(verbose: bool) {
    let mut searcher = Searcher::new(
        2,
        "What is the encryption weakness in your XMAS-encrypted list of numbers?".to_string(),
        None,
        Some(true),
    )
    .find_weakness();
}

#[derive(Debug)]
struct Searcher {
    day: u32,
    part: u32,
    question: String,
    answer: Option<String>,
    numbers: Vec<usize>,
    number: usize,
    start_index: usize,
    end_index: usize,
    preamble_length: usize,
    combination_length: usize,
    verbose_output: Option<bool>,
}

impl Searcher {
    pub fn new(
        part: u32,
        question: String,
        answer: Option<String>,
        verbose: Option<bool>,
    ) -> Searcher {
        let mut searcher = Searcher {
            day: 9,
            part: part,
            question: question,
            answer: answer,
            numbers: Vec::new(),
            number: 0,
            start_index: 0,
            end_index: 25,
            preamble_length: 25,
            combination_length: 2,
            verbose_output: verbose,
        };
        searcher.read_input("day9_input.txt");
        searcher
    }

    fn read_input(&mut self, file_name: &str) {
        self.numbers = std::fs::read_to_string(file_name)
            .unwrap()
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        match self.verbose_output {
            Some(true) => println!("{:?}", &self.numbers),
            _ => {}
        }
    }

    pub fn find_outlier(&mut self) {
        let mut outlier_is_found = false;
        while !outlier_is_found {
            let elements_to_check = &self.numbers[self.start_index..self.end_index];
            self.number = self.numbers[self.end_index];
            let combinations = elements_to_check
                .into_iter()
                .combinations(self.combination_length);
            match self.verbose_output {
                Some(true) => {
                    println!("{}:{}", self.start_index, self.number);
                    println!("elements_to_check:{:?}", elements_to_check);
                }
                _ => {}
            }
            let mut match_is_found = false;
            for combination in combinations {
                let sum = combination.to_owned().into_iter().sum::<usize>();
                if sum == self.number {
                    match_is_found = true;
                    match self.verbose_output {
                        Some(true) => {
                            println!("combination:{:?}=={}", combination, sum);
                        }
                        _ => {}
                    }
                    break;
                }
            }
            if !match_is_found {
                outlier_is_found = true;
            } else {
                self.list_shift(1);
            }
        }
        self.print_answer();
    }

    fn print_answer(&self) {
        print_answer(
            self.day,
            self.part,
            &self.question,
            &self.number.to_string(),
        );
    }

    fn list_shift(&mut self, by: usize) {
        self.start_index += by;
        self.end_index += by;
    }

    fn end_index_increment(&mut self, by: usize) {
        self.end_index = self.start_index + by;
    }

    pub fn find_weakness(&mut self) {
        // find a contiguous set of 2 or more numbers that sum to 542529149 and sum the lowest and highest values
        let target: usize = 542529149;
        let mut by: usize = 1;
        let mut length: usize = 1;
        self.end_index_increment(length);

        while self.number == 0 {
            if self.end_index >= self.numbers.len() {
                length += 1;
                self.start_index = 0;
                self.end_index_increment(length);
            }

            let mut numbers = self.numbers[self.start_index..=self.end_index].to_owned();

            if self.verbose_output == Some(true) {
                println!("{:?}", numbers);
            }

            if numbers.iter().sum::<usize>() == target {
                numbers.sort();
                self.number = numbers.first().unwrap() + numbers.last().unwrap();
                if self.verbose_output == Some(true) {
                    println!("");
                    println!("{:?}", numbers);
                }
                break;
            } else {
                self.list_shift(by);
            }
        }
        self.print_answer();
    }
}
