use std::fs;

fn get_marker(file: &str, marker_count: usize) -> Option<usize> {
    let characters: Vec<char> = fs::read_to_string(file)
        .expect("File not present")
        .chars()
        .collect();
    let characters_length = &characters.len();

    for i in marker_count - 1..*characters_length {
        let mut is_unique = true;
        'outer: for j in 0..marker_count {
            for k in 0..marker_count {
                if j == k {
                    continue;
                }
                if &characters[i - j] == &characters[i - k] {
                    is_unique = false;
                    break 'outer;
                }
            }
        }
        if is_unique {
            return Some(i + 1);
        }
    }
    return None;
}
fn main() {
    let marker_count = 14;
    println!("T0: {}", get_marker("input.t0.txt", marker_count).unwrap());
    println!("T1: {}", get_marker("input.t1.txt", marker_count).unwrap());
    println!("T2: {}", get_marker("input.t2.txt", marker_count).unwrap());
    println!("T3: {}", get_marker("input.t3.txt", marker_count).unwrap());
    println!("T4: {}", get_marker("input.t4.txt", marker_count).unwrap());
    println!("Answer: {}", get_marker("input.txt", marker_count).unwrap());
}
