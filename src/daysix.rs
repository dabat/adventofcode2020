/*
--- Day 6: Custom Customs ---

As your flight approaches the regional airport where you'll switch to a much larger plane,
customs declaration forms are distributed to the passengers.

The form asks a series of 26 yes-or-no questions marked a through z.
All you need to do is identify the questions for which anyone in your group answers "yes".
Since your group is just you, this doesn't take very long.

However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help.
For each of the people in their group, you write down the questions for which they answer "yes", one per line.
For example:

abcx
abcy
abcz

In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z.
(Duplicate answers to the same question don't count extra; each question counts at most once.)

Another group asks for your help, then another, and eventually you've collected answers from every group on the plane
(your puzzle input). Each group's answers are separated by a blank line, and within each group,
each person's answers are on a single line. For example:

abc

a
b
c

ab
ac

a
a
a
a

b

This list represents answers from five groups:

    The first group contains one person who answered "yes" to 3 questions: a, b, and c.
    The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
    The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
    The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
    The last group contains one person who answered "yes" to only 1 question, b.

In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.

For each group, count the number of questions to which anyone answered "yes".
What is the sum of those counts?
6782

--- Part Two ---

As you finish the last group's customs declaration, you notice that you misread one word in the instructions:

You don't need to identify the questions to which anyone answered "yes";
you need to identify the questions to which everyone answered "yes"!

Using the same example as above:

abc

a
b
c

ab
ac

a
a
a
a

b

This list represents answers from five groups:

    In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
    In the second group, there is no question to which everyone answered "yes".
    In the third group, everyone answered yes to only 1 question, a.
        Since some people did not answer "yes" to b or c, they don't count.
    In the fourth group, everyone answered yes to only 1 question, a.
    In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.

In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.

For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?
counts=[19, 14, 5, 17, 12, 1, 10, 6, 0, 1, 19, 13, 18, 22, 13, 19, 3, 1, 4, 13, 3, 21, 2, 17, 13, 7, 13, 22, 3, 4, 4, 2, 0, 5, 0, 1, 4, 24, 0, 13, 1, 18, 2, 4, 8, 7, 6, 17, 11, 6, 5, 2, 25, 12, 11, 1, 12, 5, 15, 18, 5, 1, 10, 0, 20, 4, 7, 16, 7, 3, 18, 8, 19, 1, 4, 5, 5, 9, 12, 5, 5, 6, 0, 4, 7, 22, 14, 11, 1, 25, 21, 4, 13, 6, 7, 16, 9, 0, 5, 24, 5, 17, 10, 5, 0, 16, 24, 1, 14, 8, 5, 8, 15, 6, 9, 9, 5, 4, 4, 16, 15, 8, 15, 5, 9, 16, 1, 9, 2, 2, 14, 19, 2, 5, 6, 22, 7, 13, 14, 6, 14, 0, 2, 2, 2, 7, 10, 5, 3, 3, 21, 7, 3, 1, 17, 18, 4, 10, 12, 4, 3, 9, 1, 9, 14, 12, 24, 3, 3, 14, 6, 0, 2, 1, 2, 1, 8, 1, 12, 10, 0, 7, 8, 17, 11, 8, 1, 5, 20, 6, 5, 12, 3, 2, 3, 11, 14, 10, 4, 1, 9, 2, 16, 4, 7, 13, 3, 8, 0, 4, 6, 3, 7, 2, 11, 5, 9, 3, 12, 20, 2, 2, 22, 6, 15, 9, 3, 16, 11, 12, 0, 10, 13, 0, 5, 0, 12, 16, 0, 5, 1, 5, 12, 3, 1, 19, 13, 0, 9, 13, 8, 4, 19, 18, 13, 14, 4, 5, 4, 0, 1, 7, 9, 8, 13, 5, 12, 3, 6, 1, 6, 26, 0, 4, 2, 7, 1, 6, 2, 13, 2, 9, 7, 16, 0, 14, 1, 4, 1, 14, 3, 8, 0, 19, 17, 22, 0, 21, 13, 1, 17, 5, 8, 9, 24, 8, 2, 7, 14, 7, 12, 6, 6, 1, 0, 10, 19, 8, 1, 6, 17, 17, 15, 1, 3, 1, 5, 2, 9, 0, 6, 7, 17, 5, 7, 7, 17, 8, 2, 0, 1, 7, 12, 5, 20, 2, 17, 4, 2, 2, 8, 12, 1, 1, 20, 0, 6, 8, 13, 1, 3, 2, 0, 15, 7, 13, 5, 1, 0, 24, 2, 16, 1, 7, 4, 7, 13, 20, 6, 2, 8, 1, 15, 19, 5, 0, 1, 4, 2, 7, 2, 1, 8, 5, 6, 13, 3, 3, 15, 6, 14, 8, 5, 2, 4, 2, 10, 24, 12, 2, 2, 11, 2, 0, 2, 11, 14, 5, 0, 1, 10, 21, 1, 10, 3, 13, 7, 4, 11, 7, 7, 12, 20, 4, 19, 4, 2, 0, 2, 9, 1, 6, 7, 1, 4, 8, 8, 7, 13, 0, 8, 1, 14, 2, 5, 0]
sum=3596
*/
#![allow(unused)]
use std::collections::{HashMap, HashSet};

pub fn day6_part1() {
    let groups = read_customs_file_groups();
    // println!("{:?}", groups);
    let group_counts: Vec<usize> = count_group_answers(groups);
    // println!("{:?}", group_counts);
    let group_count_sum: usize = group_counts.iter().sum();
    println!("Day 6 Part 1");
    println!("For each group, count the number of questions to which anyone answered \"yes\". What is the sum of those counts?");
    println!("sum={:?}", group_count_sum);
}

pub fn day6_part2() {
    let groups = read_customs_file_groups();
    let counts = count_group_answers_all(groups);
    let sum: usize = counts.iter().sum();
    // println!("counts={:?}", counts);
    println!("Day 6 Part 2");
    println!("For each group, count the number of questions to which everyone answered \"yes\". What is the sum of those counts?");
    println!("sum={}", sum);
}

fn read_customs_file_groups() -> Vec<String> {
    std::fs::read_to_string("day6_input.txt")
        .unwrap()
        .split("\n\n")
        .into_iter()
        .map(String::from)
        .map(|s| s.replace("\n", " "))
        .collect()
}

fn count_group_answers(customs_groups: Vec<String>) -> Vec<usize> {
    let mut list: Vec<usize> = Vec::new();

    for group in customs_groups {
        let mut set: HashSet<char> = HashSet::new();
        let characters: Vec<char> = group.chars().filter(|c| !c.is_whitespace()).collect();
        for character in characters {
            set.insert(character);
        }
        list.push(set.len());
    }

    list
}

fn count_group_answers_all(customs_groups: Vec<String>) -> Vec<usize> {
    let mut list: Vec<usize> = Vec::new();

    for group in customs_groups {
        let mut map: HashMap<char, u32> = HashMap::new();
        let people: Vec<&str> = group.split_whitespace().collect();
        let mut group_count: usize = 0;

        // println!("group={:?}", group);
        // println!("people={:?}", people);

        for person_answers in &people {
            // println!("person_answers={}", person_answers);
            for answer in person_answers.chars() {
                if map.contains_key(&answer) {
                    let count = map.get_mut(&answer).unwrap();
                    *count += 1;
                } else {
                    map.insert(answer.clone(), 1);
                }
            }
        }
        // println!("map={:?}", map);
        // println!("person_count={}", &people.len());
        for (_answer, count) in map.iter() {
            if *count == people.len() as u32 {
                group_count += 1;
            }
        }
        list.push(group_count);
        // println!("group_count={}", group_count);
    }

    list
}
