fn main() {
    let file = include_str!("../../input.txt")
        .split("\n\n")
        .collect::<Vec<_>>();

    let mut stack = file[0]
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            line.split("")
                .skip(2)
                .step_by(4)
                .enumerate()
                .collect::<Vec<_>>()
        })
        .fold(vec![], |mut acc: Vec<String>, line| {
            for (index, char) in line {
                while acc.get(index).is_none() {
                    acc.push(String::new());
                }
                if char != " " {
                    acc[index].push_str(char);
                }
            }
            acc
        });
    let instructions = file[1]
        .lines()
        .map(|line| {
            let arr = line.split(" ").collect::<Vec<_>>();
            (
                arr[1].parse::<usize>().unwrap(),
                arr[3].parse::<usize>().unwrap() - 1,
                arr[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect::<Vec<_>>();

    for (q, a, b) in instructions {
        let mut arr = vec![];
        for _ in 0..q {
            arr.push(stack[a].pop().unwrap());
        }
        for item in arr.iter().rev() {
            stack[b].push(*item);
        }
    }

    for mut element in stack {
        print!("{}", element.pop().unwrap());
    }
    println!("");
}
