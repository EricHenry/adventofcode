//use regex::Regex;
//use std::collections::HashSet;
use std::{fs, println};

fn solution_1(i: usize, contents: &Vec<(&str, i32)>, seen: &mut Vec<bool>) -> usize {
    if i >= contents.len() {
        return 0;
    }
    match (seen[i], contents[i]) {
        (false, (op, arg)) if op == "acc" => {
            seen[i] = true;
            (arg + (solution_1(i + 1, contents, seen) as i32)) as usize
        }
        (false, (op, arg)) if op == "jmp" => {
            seen[i] = true;
            solution_1((arg + i as i32) as usize, contents, seen)
        }
        (false, (op, _)) if op == "nop" => {
            seen[i] = true;
            solution_1(i + 1, contents, seen)
        }
        _ => 0,
    }
}

fn solution_2(instructions: &Vec<(&str, i32)>) -> usize {
    let mut result = 0;
    for (i, (op, arg)) in instructions.iter().enumerate() {
        let mut test_instructions = instructions.to_vec();
        let mut s2_seen = vec![false; instructions.len()];
        match *op {
            "nop" => test_instructions[i] = ("jmp", *arg),
            "jmp" => test_instructions[i] = ("nop", *arg),
            _ => continue,
        };

        let test_result = solution_1(0, &test_instructions, &mut s2_seen);
        if s2_seen[instructions.len() - 1] == true {
            result = test_result;
            break;
        }
    }
    result
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("something went wrong");
    let instructions: Vec<(&str, i32)> = file
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let instruction_set: Vec<&str> = x.split(" ").collect();
            (
                instruction_set[0],
                instruction_set[1].parse::<i32>().unwrap(),
            )
        })
        .collect();
    let mut seen = vec![false; instructions.len()];

    let s1 = solution_1(0, &instructions, &mut seen);
    println!("solution_1 -> {}", s1);

    let s2 = solution_2(&instructions.to_vec());
    println!("solution_2 -> {}", s2);
}
