/*
--- Day 5: Binary Boarding ---
You board your plane only to discover a new problem: you dropped your boarding pass!
You aren't sure which seat is yours,
and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.

You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your puzzle input);
perhaps you can find your seat through process of elimination.

Instead of zones or groups, this airline uses binary space partitioning to seat people.
A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".

The first 7 characters will either be F or B;
these specify exactly one of the 128 rows on the plane (numbered 0 through 127).
Each letter tells you which half of a region the given seat is in.
Start with the whole list of rows;
the first letter indicates whether the seat is in the front (0 through 63) or the back (64 through 127).
The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.

For example, consider just the first seven characters of FBFBBFFRLR:

    Start by considering the whole range, rows 0 through 127.
    F means to take the lower half, keeping rows 0 through 63.
    B means to take the upper half, keeping rows 32 through 63.
    F means to take the lower half, keeping rows 32 through 47.
    B means to take the upper half, keeping rows 40 through 47.
    B keeps rows 44 through 47.
    F keeps rows 44 through 45.
    The final F keeps the lower of the two, row 44.

The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7).
The same process as above proceeds again, this time with only three steps.
L means to keep the lower half, while R means to keep the upper half.

For example, consider just the last 3 characters of FBFBBFFRLR:

    Start by considering the whole range, columns 0 through 7.
    R means to take the upper half, keeping columns 4 through 7.
    L means to take the lower half, keeping columns 4 through 5.
    The final R keeps the upper of the two, column 5.

So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.

Every seat also has a unique seat ID: multiply the row by 8, then add the column.
In this example, the seat has ID 44 * 8 + 5 = 357.

Here are some other boarding passes:

    BFFFBBFRRR: row 70, column 7, seat ID 567.
    FFFBBBFRRR: row 14, column 7, seat ID 119.
    BBFFBBFRLL: row 102, column 4, seat ID 820.

As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass?
highest seat id: 935

--- Part Two ---
Ding! The "fasten seat belt" signs have turned on. Time to find your seat.
It's a completely full flight, so your seat should be the only missing boarding pass in your list.
However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft,
so they'll be missing from your list as well.

Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.

What is the ID of your seat?
my seat id: 743
 */
use crate::utils::*;

pub fn day5_part1() {
    let seat_ids = find_seat_ids();
    println!("Day 5 Part 1");
    println!("As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass?");
    println!("highest seat id: {:?}", seat_ids.last().unwrap());
}

pub fn day5_part2() {
    let my_seat_id = find_my_seat_id();
    println!("Day 5 Part 2");
    println!("What is the ID of your seat?");
    println!("my seat id: {}", my_seat_id);
}

fn find_my_seat_id() -> u32 {
    let seat_ids = find_seat_ids();
    let mut my_seat_id: u32 = 0;

    for index in 0..seat_ids.len() {
        let id_this = seat_ids.get(index).unwrap();
        let id_next_expected: u32 = id_this + 1;
        let id_next_actual: u32 = *seat_ids.get((index + 1) as usize).unwrap();

        if id_next_expected != id_next_actual {
            my_seat_id = id_next_expected;
            break;
        }
    }

    my_seat_id
}

fn find_seat_ids() -> Vec<u32> {
    let passes = read_input_file("day5_input.txt");
    let mut seat_ids: Vec<u32> = vec![];

    for pass in passes {
        // println!("{}", pass);
        let pass_decoded = decode(&pass);
        // println!("{:?}", pass_decoded);
        seat_ids.push(pass_decoded.seat_id);
    }
    seat_ids.sort();
    // println!("seat_ids: {:?}", seat_ids);
    seat_ids
}

fn decode(coded_boarding_pass: &str) -> BoardingPass {
    let row = decode_row(coded_boarding_pass);
    let column = decode_column(coded_boarding_pass);
    let seat_id = row * 8 + column;

    BoardingPass::new(row, column, seat_id)
}

fn decode_row(coded_boarding_pass: &str) -> u32 {
    let rows: Vec<u32> = (0..=127).collect();
    let row_code = &coded_boarding_pass[0..=6];
    let row: u32;
    let mut row_maybe: &[u32] = &rows;

    for code in row_code.chars() {
        let mid = row_maybe.len() / 2;
        let (rows_low, rows_high) = row_maybe.split_at(mid);

        row_maybe = match code {
            'F' => rows_low,
            'B' => rows_high,
            _ => &[], //no-op to silence compiler error about non-exhaustive match arms
        };

        // println!("code: {} rows: {:?}", code, &row_maybe);
    }

    row = *row_maybe.first().unwrap();
    // println!("rows=={:?}", row_maybe);
    // println!("row=={:?}", row);
    row
}

fn decode_column(coded_boarding_pass: &str) -> u32 {
    let columns: Vec<u32> = (0..=7).collect();
    let mut column_maybe: &[u32] = &columns;
    let column_code = &coded_boarding_pass[7..=9];
    let column: u32;

    for code in column_code.chars() {
        let mid = column_maybe.len() / 2;
        let (columns_low, columns_high) = column_maybe.split_at(mid);

        column_maybe = match code {
            'L' => columns_low,
            'R' => columns_high,
            _ => &[],
        };

        // println!("code: {} columns: {:?}", code, &column_maybe);
    }

    column = *column_maybe.first().unwrap();
    // println!("columns=={:?}", column_maybe);
    // println!("column=={:?}", column);

    column
}

#[derive(Debug)]
struct BoardingPass {
    row: u32,
    column: u32,
    seat_id: u32,
}
impl BoardingPass {
    fn new(row: u32, column: u32, seat_id: u32) -> BoardingPass {
        BoardingPass {
            row,
            column,
            seat_id,
        }
    }
}
