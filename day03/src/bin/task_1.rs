use std::collections::HashMap;

fn main() {

    let lines: Vec<&str> = include_str!("../../input.txt").split("\n").into_iter().collect();
    let mut priority_sum: usize = 0;

    let mut key_code_map: HashMap<char, usize> = HashMap::new();
    for (i, char) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate() {
        key_code_map.insert(char, i + 1);
    }

    for ele in lines {
        let mut map: HashMap<char, usize> = HashMap::new();
        let (first, second) = ele.split_at(ele.len() / 2);

        for c in first.chars() {
            map.insert(c, 1);
        }

        for c in second.chars() {
            if map.contains_key(&c) {
                priority_sum += key_code_map.get(&c).unwrap();
                break;
            }
        
        }

    }

    println!("{}", priority_sum);
    
}
