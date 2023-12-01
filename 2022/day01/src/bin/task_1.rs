use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut elf_calories = 0;
        let mut rich_elf_calories = 0;
        for line in lines {
            if let Ok(value) = line {
                if value.is_empty() {
                    if elf_calories > rich_elf_calories {
                        rich_elf_calories = elf_calories;
                    }
                    elf_calories = 0;
                } else {
                    elf_calories += value.parse::<i32>().unwrap();
                }
            }
        }
        println!(
            "The richest elf carries {} calories.",
            rich_elf_calories
        );
    }
}
