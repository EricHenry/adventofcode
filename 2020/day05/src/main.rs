use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn get_row(xs: &str) -> usize {
    let mut rows: Vec<usize> = (0..128).collect();
    for x in xs.chars().into_iter() {
        let half = rows.len() / 2;
        let (front, back) = rows.split_at(half);
        rows = match x {
            'F' => front.iter().cloned().collect::<Vec<usize>>(),
            _ => back.iter().cloned().collect::<Vec<usize>>(),
        };
    }

    match rows.first() {
        Some(&x) => x,
        None => {
            println!("error empty row");
            0
        }
    }
}

fn get_seat(ys: &str) -> usize {
    let mut rows: Vec<usize> = (0..8).collect();
    for y in ys.chars().into_iter() {
        let half = rows.len() / 2;
        let (left, right) = rows.split_at(half);
        rows = match y {
            'L' => left.iter().cloned().collect::<Vec<usize>>(),
            _ => right.iter().cloned().collect::<Vec<usize>>(),
        };
    }

    match rows.first() {
        Some(&x) => x,
        None => {
            println!("error empty row");
            0
        }
    }
}

fn solution_1() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut ids: Vec<usize> = vec![];
    for line in reader.lines() {
        let l = line?;
        if l.is_empty() {
            continue;
        }
        let (xs, ys) = l.split_at(7);
        let row = get_row(xs);
        let seat = get_seat(ys);
        let id = row * 8 + seat;
        //println!("({},{}) -> {}", row, seat, id);
        ids.push(id)
    }

    match ids.iter().max() {
        Some(i) => println!("Solution 1: {}", i),
        _ => println!("Solution 1: somethign went wrong, no max id"),
    }

    Ok(())
}

fn solution_2() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut ids: Vec<usize> = vec![];
    for line in reader.lines() {
        let l = line?;
        if l.is_empty() {
            continue;
        }
        let (xs, ys) = l.split_at(7);
        let row = get_row(xs);
        let seat = get_seat(ys);
        let id = row * 8 + seat;
        ids.push(id)
    }
    ids.sort();
    let (my_id, _) = ids.iter().fold((0, false), |acc, &id| match acc {
        (_, true) => acc,
        (c, false) if c == 0 => (id, false),
        (c, false) if c + 1 != id => (c + 1, true),
        _ => (id, false),
    });
    println!("Solution 2: {}", my_id);

    Ok(())
}

fn main() {
    solution_1().unwrap();
    solution_2().unwrap();
}
