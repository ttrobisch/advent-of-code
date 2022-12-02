use std::collections::HashMap;
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
        let mut map: HashMap<&str, i32> = HashMap::new();
        // Rock     - 1 X
        // Paper    - 2 Y
        // Scissors - 3 Z
        map.insert("A X", 1 + 3);
        map.insert("B X", 1);
        map.insert("C X", 1 + 6);
        map.insert("A Y", 2 + 6);
        map.insert("B Y", 2 + 3);
        map.insert("C Y", 2);
        map.insert("A Z", 3);
        map.insert("B Z", 3 + 6);
        map.insert("C Z", 3 + 3);
        let mut res = 0;
        for line in lines {
            if let Ok(line) = line {
                if let Some(val) = map.get(line.as_str()) {
                    res += val;
                }
            }
        }

        println!("Result: {}", res);
    }
}
