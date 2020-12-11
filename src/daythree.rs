/*
--- Day 3: Toboggan Trajectory ---

With the toboggan login problems resolved, you set off toward the airport.
While travel by toboggan might be easy, it's certainly not safe:
there's very minimal steering and the area is covered in trees.
You'll need to see which angles will take you near the fewest trees.

Due to the local geology, trees in this area only grow on exact integer coordinates in a grid.
You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:

..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#

These aren't the only trees, though;
due to something you read about once involving arboreal genetics and biome stability,
the same pattern repeats to the right many times:

..##.........##.........##.........##.........##.........##.......  --->
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

You start on the open square (.) in the top-left corner and need to reach the bottom
(below the bottom-most row on your map).

The toboggan can only follow a few specific slopes
(you opted for a cheaper model that prefers rational numbers);
start by counting all the trees you would encounter for the slope right 3, down 1:

From your starting position at the top-left, check the position that is right 3 and down 1.
Then, check the position that is right 3 and down 1 from there,
and so on until you go past the bottom of the map.

The locations you'd check in the above example are marked here with O where there was an open square
and X where there was a tree:

..##.........##.........##.........##.........##.........##.......  --->
#..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........X.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...#X....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

In this example, traversing the map using this slope would cause you to encounter 7 trees.

Starting at the top-left corner of your map and following a slope of right 3 and down 1,
how many trees would you encounter?
https://adventofcode.com/2020/day/3
32 is not correct. had the rows mixed up.
43 is not correct. had the row iteration all messed up 🤦‍♂️ need to check the character in each row except the first one, but i was skipping even rows
the answer you found! 🚀 ... 220
*/
use crate::utils::*;

pub fn puzzlefive_count_trees() {
    // import the input file
    let list = read_input_file("day3_input.txt");
    let mut tree_count = 0;
    let mut position = 0;
    let tree = '#';

    // iterate through each line and
    for (row_number, line) in list.iter().enumerate() {
        let mut pattern_text = line.clone();
        // we need to ensure that the pattern_text is long enough to evaluate
        // if the move 3 right would exceed the length of the line then extend the line contents
        while position >= pattern_text.len() {
            pattern_text.push_str(&line.clone());
        }
        // in the first row we just move to the right and then down without checking for a tree
        if row_number == 0 {
            position += 3;
            continue;
        }
        // check the character at the position
        // if the character is a tree (#) then increment the counter
        let character = pattern_text.chars().nth(position).unwrap_or_default();
        if character == tree {
            tree_count += 1;
        }
        // move right and down (i.e. skip to the next line/row)
        position += 3;
    }
    // print out the final count of trees
    print_answer(&tree_count.to_string());
}
