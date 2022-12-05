fn split(value: &str, char: char) -> (&str, &str) {
    let array: Vec<&str> = value.split(char).collect();
    (array[0], array[1])
}

fn main() {
    let count = include_str!("../../input.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| split(line, ','))
        .map(|(first, second)| (split(first, '-'), split(second, '-')))
        .fold(0, |acc, ((min_a, max_a), (min_b, max_b))| {
            let min_a = min_a.parse::<i32>().expect("Not a number");
            let max_a = max_a.parse::<i32>().expect("Not a number");
            let min_b = min_b.parse::<i32>().expect("Not a number");
            let max_b = max_b.parse::<i32>().expect("Not a number");
            let mut inc = 0;
            if (min_a >= min_b && max_a <= max_b) || (min_b >= min_a && max_b <= max_a) {
                inc = 1;
            }
            acc + inc
        });

    println!("There are {} contained pairs.", count);
}
