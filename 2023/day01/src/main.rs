/*
--- Day 1: Trebuchet?! ---
Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?


*/

use std::{ops::Index, collections::{HashMap, BTreeSet, BTreeMap}};

fn step01() {
    let count = include_str!("./input01.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .fold(0, |acc, x: Vec<u32>| {
            let first = x.first().expect("");
            let last = x.last().expect("");
            acc + first * 10 + last 
        });

    println!("Step 01 Count: {}", count);
}

fn step02() {
    let count: u32 = include_str!("./input01.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|l| {
            let mut tree = BTreeMap::new();

            let nums = vec![
                ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
                ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
            ];

            for (lit, num) in nums {
                if let Some(x) = l.find(lit) {
                    tree.insert(x, num);
                }
                if let Some(x) = l.rfind(lit) {
                    tree.insert(x, num);
                }
            }

            let (_, a) = tree.iter().next().expect("");
            let (_, b) = tree.iter().next_back().expect("");

            a * 10 + b
        })
    .sum();
    println!("Step 02 Count: {}", count);
}

fn main() {
    let start = std::time::Instant::now();
    step01();
    let duration = start.elapsed();
    println!("Step 1 ran in {:?}", duration);
    let start = std::time::Instant::now();
    step02();
    let duration = start.elapsed();
    println!("Step 2 ran in {:?}", duration);

}
