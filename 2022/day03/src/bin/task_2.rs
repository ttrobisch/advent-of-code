use std::collections::HashMap;

fn main() {

    let lines: Vec<&str> = include_str!("../../input.txt").split("\n").collect();
    let groups: Vec<&[&str]> = lines.chunks(3).collect();
    let mut priority_sum: usize = 0;

    let mut priority_map: HashMap<char, usize> = HashMap::new();
    for (i, char) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate() {
        priority_map.insert(char, i + 1);
    }

    for lines in groups {
        let mut group_map: HashMap<char, usize> = HashMap::new();
        for line in lines {
            let mut line_map: HashMap<char, bool> = HashMap::new();
            for char in line.chars() {
                if line_map.contains_key(&char) {
                    continue;
                }
                if let Some(count) = group_map.get(&char) {
                    group_map.insert(char, count + 1);
                } else {
                    group_map.insert(char, 1);
                }
                line_map.insert(char, true);
            }
        }

        let max = group_map.iter().max_by(| (_, a), (_, b) | { a.cmp(b) });
        if let Some((ele, _)) = max {
            priority_sum += priority_map.get(&ele).unwrap();
        }
    }

    println!("{}", priority_sum);
    
}
