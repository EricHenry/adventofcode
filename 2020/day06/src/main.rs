use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn count_plz(s: String) -> usize {
    let bytes = s.as_bytes();
    (b'a'..=b'z')
        .filter(|c| bytes.contains(c))
        .collect::<Vec<u8>>()
        .len()
}

fn solution_1() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut counts = 0;
    let mut ys: Vec<String> = vec![];
    for line in reader.lines() {
        let l = line?;
        if l.is_empty() {
            let yd = ys.join("");
            let count = count_plz(yd);
            counts += count;
            ys = vec![];
        } else {
            ys.push(l);
        }
    }

    println!("Solution 1: {}", counts);

    Ok(())
}

fn solution_2() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut counts = 0;
    let mut ys: Vec<String> = vec![];
    for line in reader.lines() {
        let l = line?;
        if l.is_empty() {
            let string = ys.join("");
            let mut char_hits: HashMap<char, usize> = HashMap::new();
            for c in string.chars() {
                let hits = char_hits.entry(c).or_insert(0);
                *hits += 1;
            }
            let count = char_hits
                .iter()
                .filter(|(_, &value)| value == ys.len())
                .count();
            counts += count;
            ys = vec![];
        } else {
            ys.push(l);
        }
    }

    println!("Solution 2: {}", counts);

    Ok(())
}

fn main() {
    solution_1().unwrap();
    solution_2().unwrap();
}
