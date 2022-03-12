use std::collections::BTreeMap;
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

fn day_1<P: AsRef<Path> + Clone>(p: P) {
    let mut lines = 0;
    let path = p.clone();
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
        lines += 1;
    }
    println!("the increase count is {}", count);

    let file = fs::File::open(path).unwrap();
    // Part 2
    let mut count = 0;
    let mut num: i32 = 0;
    let mut map = BTreeMap::<i32, i32>::new();
    for line in io::BufReader::new(file).lines() {
        if num == lines {
            break;
        }
        let line = &line.unwrap();
        let val = line.trim().parse::<i32>().unwrap();
        if num < 3 {
            let cnt = num % 3;
            for i in 0..=cnt {
                if let Some(v) = map.get(&(num - i)) {
                    map.insert(num - i, val + *v);
                } else {
                    map.insert(num - i, val);
                }
            }
        } else if num >= lines - 3 {
            let cnt = lines - num;
            let end = lines - 3;
            for i in 0..cnt {
                if let Some(v) = map.get(&(end - i)) {
                    map.insert(end - i, *v + val);
                } else {
                    map.insert(end - i, val);
                }
            }
        } else {
            let cnt = 3;
            for i in 0..cnt {
                if let Some(v) = map.get(&(num - i)) {
                    map.insert(num - i, val + *v);
                } else {
                    map.insert(num - i, val);
                }
            }
        }
        num += 1;
    }
    let mut pre = Option::<&i32>::None;
    let mut cur = pre;
    let mut count = 0;
    for (k, v) in &map {
        print!("key is {}, value is {}\n", *k, *v);
        pre = cur;
        cur = Some(v);
        match (pre, cur) {
            (Option::None, Option::Some(_)) => continue,
            (Option::Some(a), Option::Some(b)) => {
                if *b > *a {
                    count += 1;
                }
            }
            _ => panic!(),
        }
    }
    println!("the increasing count is {}", count);
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
