/*
--- Day 7: Handy Haversacks ---

You land at the regional airport in time for your next flight.
In fact, it looks like you'll even have time to grab some food:
all flights are currently delayed due to issues in luggage processing.

Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents;
bags must be color-coded and must contain specific quantities of other color-coded bags.
Apparently, nobody responsible for these regulations considered how long they would take to enforce!

For example, consider the following rules:

light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.

These rules specify the required contents for 9 bag types.
In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black),
and so on.

You have a shiny gold bag.
If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag?
(In other words: how many colors can, eventually, contain at least one shiny gold bag?)

In the above rules, the following options would be available to you:

    A bright white bag, which can hold your shiny gold bag directly.
    A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
    A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
    A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.

So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.

How many bag colors can eventually contain at least one shiny gold bag?
(The list of rules is quite long; make sure you get all of it.)
..... Day 7 - Part 1 .....
How many bag colors can eventually contain at least one `shiny gold` bag?
229

--- Part Two ---

It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!

Consider again your shiny gold bag and the rules from the above example:

    faded blue bags contain 0 other bags.
    dotted black bags contain 0 other bags.
    vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
    dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.

So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!

Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!

Here's another example:

shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.

In this example, a single shiny gold bag must contain 126 other bags.

How many individual bags are required inside your single shiny gold bag?

*/
use crate::utils::*;
use std::collections::{HashMap, HashSet};

pub fn day7_part1() {
    /*
    need to split the lines into a bag and it's content. can split on string ' bags contain '.
       bag name      content delimiter          bag content
      ____|_____   _____|____   _________________|_________________________________________________________
    /           \ /          \ /                                                                           \
    mirrored teal bags contain 5 clear brown bags, 4 bright magenta bags, 1 drab brown bag, 2 dull gold bags.

    need to split the bag content into individual bags, can split on ', '. need to remove the bag/bags from the name.
      content item         content item          content item      content item
    ________|________    _________|__________   ______|________   _______|_______
    /                 \ /                    \ /               \ /               \
    5 clear brown bags, 4 bright magenta bags, 1 drab brown bag, 2 dull gold bags.
    */

    let bags = read_input_file("day7_input.txt");
    let hashed_bags = hash_bag_contents(bags);

    // for bag in hashed_bags {
    //     println!("{:?}", bag);
    // }

    // let mut bag = HashMap::new();
    // bag.insert(
    //     "mirrored teal".to_string(),
    //     "5 clear brown bags, 4 bright magenta bags, 1 drab brown bag, 2 dull gold bags."
    //         .to_string(),
    // );
    // bag.insert(
    //     "bright gray".to_string(),
    //     "5 pale aqua bags, 3 shiny gold bags, 1 clear olive bag, 1 dull fuchsia bag.".to_string(),
    // );
    // bag.insert(
    //     "dotted black".to_string(),
    //     "2 vibrant white bags.".to_string(),
    // );
    // println!("bag: {:?}", bag);
    // let hashed_bag = hash_bag_contents(bag);
    // println!("hashed_bag: {:?}", hashed_bag);

    let mut colors: HashSet<String> = HashSet::new();
    let mut colors_to_find: Vec<String> = vec!["shiny gold".to_string()];
    let mut find_more = true;

    while find_more {
        colors_to_find = find_bags_containing(colors_to_find, hashed_bags.to_owned());

        for color in &colors_to_find {
            colors.insert(color.to_owned());
        }

        // ahh!! crazy numeric conversions!!!
        // let colors_found_count: i32 = colors_to_find.len() as i32;
        // if colors_found_count == 0 {
        // if &colors_to_find.len() == &usize::try_from(0).unwrap() {
        if colors_to_find.len() as i32 == 0 {
            find_more = false;
        }
    }
    println!("({}) colors: {:?}", colors.len(), colors);

    print_answer(
        7,
        1,
        "How many bag colors can eventually contain at least one `shiny gold` bag?",
        colors.len().to_string().as_str(),
    );
}

pub fn day7_part2() {
    let bags = read_input_file("day7_input.txt");
    let bag_hash = hash_bag_contents(bags);

    print_answer(
        7,
        2,
        "How many individual bags are required inside your single shiny gold bag?",
        "answer",
    );
}

fn read_input_file(filename: &str) -> HashMap<String, String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.split_at(s.find(" bags contain ").unwrap()))
        .map(|(key, value)| (key.to_string(), String::from(&value[14..])))
        .collect()
}

fn hash_bag_contents(bag: HashMap<String, String>) -> HashMap<String, HashMap<String, u32>> {
    let mut bag_hash: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for (bagname, contents) in bag {
        let content_list: Vec<String> = contents
            .split(",")
            .map(|s| {
                s.replace("bags", "")
                    .replace("bag", "")
                    .replace(".", "")
                    .trim()
                    .to_string()
            })
            .collect();
        let mut map: HashMap<String, u32> = HashMap::new();

        for element in content_list {
            let bag: (&str, &str) = element.split_at(element.find(" ").unwrap());
            map.insert(
                bag.1.trim().to_string(),
                bag.0.trim().parse::<u32>().unwrap_or_default(),
            );
        }
        bag_hash.insert(bagname.to_string(), map);
    }

    bag_hash
}

fn find_bags_containing(
    bag_colors: Vec<String>,
    bags: HashMap<String, HashMap<String, u32>>,
) -> Vec<String> {
    let mut bags_of_interest: Vec<String> = Vec::new();
    for color in bag_colors {
        for bag in &bags {
            if bag.1.contains_key(&color) {
                bags_of_interest.push(bag.0.to_string());
            }
        }
    }
    bags_of_interest
}

fn count_bags(bag_colors: Vec<String>, bags: HashMap<String, HashMap<String, u32>>) -> u32 {
    for bag in bags {
        if bag.0 == bag_color {
            bags_to_count.append(bag.1.clone().keys().collect::<Vec<String>>());
        }
    }
}

fn count_bags_in(bag_map: HashMap<String, u32>) -> u32 {
    let mut bag_counts: HashMap<String, u32> = HashMap::new();
    for bag in bag_map {
        bag_counts.insert(bag.0.clone(), bag.1);
    }
    bag_counts.values().sum()
}
