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
        let mut elfs = vec![0, 0, 0];

        for line in lines {
            if let Ok(value) = line {
                if !value.is_empty() {
                    elf_calories += value.parse::<u32>().unwrap();
                } else {
                    if let Some(first) = elfs.first() {
                        if first.lt(&elf_calories) {
                            elfs.remove(0);
                            elfs.push(elf_calories.clone());
                            elfs.sort();
                        }
                    }
                    elf_calories = 0;
                }
            }
        }

        let sum: u32 = elfs.iter().sum();
        println!("The 3 richest elfs carry {} calories.", sum);
    }
}
