use std::env;

fn main() {
    if let Some(input) = env::args().nth(1) {
        let result = input.chars().enumerate().fold(0, |acc, (count, x)| {
            let next = match input.chars().nth(count + 1) {
                Some(next) => next,
                None => input.chars().nth(0).unwrap(),
            };
            if x == next {
                acc + x.to_digit(10).unwrap()
            } else {
                acc
            }
        });
        println!("Result: {}", result);

        // Part 2

        let length = input.chars().count();
        let halfway = length / 2;

        let result = input.chars().enumerate().fold(0, |acc, (count, x)| {
            let index = (count + halfway) % length;
            let next = match input.chars().nth(index) {
                Some(next) => next,
                None => input.chars().nth(0).unwrap(),
            };
            if x == next {
                acc + x.to_digit(10).unwrap()
            } else {
                acc
            }
        });
        println!("Result: {}", result);


    } else {
        println!("No input!");
    }
}
