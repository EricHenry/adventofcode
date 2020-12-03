use std::fs::File;
use std::io::{BufRead, BufReader, Error};

/*
 *
 * 1-3 a: abcde
 * 1-3 b: cdefg
 * 2-9 c: ccccccccc
 *
 * Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
 *
 * In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.
 *
 *
 */
fn process(line: String) -> (usize, usize, String, String) {
    let entry: Vec<&str> = line.split(" ").collect();
    let policy = entry[0];
    let key = entry[1].replace(':', "");
    let password = entry[2];
    let range: Vec<usize> = policy
        .split("-")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let min = range[0];
    let max = range[1];

    (min, max, key, password.to_owned())
}

fn solution_1() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut hits = 0;
    for line in reader.lines() {
        let l = line?;
        if l.is_empty() {
            continue;
        }
        let (min, max, key, password) = process(l);
        let key_count = password
            .chars()
            .filter(|&x| x.to_string() == key)
            .collect::<String>()
            .len();

        if key_count >= min && key_count <= max {
            hits += 1;
        }
    }

    println!("Solution 1: {}", hits);

    Ok(())
}

/*
 *
 * While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.
 *
 * The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.
 *
 * Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
 *
 * Given the same example list from above:
 * 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
 * 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
 * 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
 *
 * How many passwords are valid according to the new interpretation of the policies?
 *
 */
fn solution_2() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut hits = 0;
    for line in reader.lines() {
        let l = line?;
        if l.is_empty() {
            continue;
        }
        let (min, max, key, password) = process(l);
        let password_chars: Vec<char> = password.chars().collect();
        let elem_1 = password_chars[min - 1].to_string();
        let elem_2 = password_chars[max - 1].to_string();
        match (elem_1 == key, elem_2 == key) {
            (true, false) => hits += 1,
            (false, true) => hits += 1,
            _ => (),
        }
    }

    println!("Solution 2: {}", hits);

    Ok(())
}

fn main() {
    solution_1().unwrap();
    solution_2().unwrap();
}
