use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::string;
use std::{fs, panic};
fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    match input.trim().parse::<i32>().unwrap() {
        1 => {
            day_1(r"E:\code\rust\aoc\src\input");
        }
        _ => unimplemented!("not implemented yet!"),
    }
}

enum State {
    Empty,
    Value(i32),
}

fn day_1<P: AsRef<Path>>(p: P) {
    // Part 1
    let file = fs::File::open(p).unwrap();
    let mut count = 0;
    let mut pre = State::Empty;
    let mut cur = pre;
    for line in io::BufReader::new(file).lines() {
        let line = &line.unwrap();
        pre = cur;
        cur = State::Value(line.trim().parse::<i32>().unwrap());
        if is_greater(&pre, &cur) {
            count += 1;
        }
    }
    println!("the increase count is {}", count)

    // Part 2
}

fn is_greater(pre: &State, cur: &State) -> bool {
    if let State::Empty = pre {
        false
    } else {
        match (pre, cur) {
            (State::Value(i), State::Value(j)) => *j > *i,
            _ => panic!(),
        }
    }
}
