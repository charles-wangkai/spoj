use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    loop {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        let value = split.next().unwrap().parse().unwrap();
        if value == 42 {
            break;
        }

        println!("{}", solve(value));
    }
}

fn solve(value: i32) -> i32 {
    value
}
