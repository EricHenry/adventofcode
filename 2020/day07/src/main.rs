use regex::Regex;
use std::collections::HashSet;
use std::fs;

fn solution_1() {
    let mut hits = 0;
    let contents = fs::read_to_string("input.txt").expect("something went wrong");
    let mut cs: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    let mut find_bags: HashSet<String> = HashSet::new();
    find_bags.insert("shiny gold".to_string());
    loop {
        let bags = find_bags
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join("|");
        let formatted = format!("\\d ({})", bags);
        println!("\n{}", formatted);
        let re = Regex::new(formatted.as_str()).unwrap();
        let mut new_bags: HashSet<String> = HashSet::new();
        let mut new_cs: Vec<String> = vec![];
        for line in &cs {
            if re.is_match(line.as_str()) {
                println!("{}", line);
                let bag = line.split(" bags contain").nth(0);
                match bag {
                    Some(bag) => {
                        new_bags.insert(bag.to_string());
                    }
                    _ => (),
                };
                hits += 1;
            } else {
                new_cs.push(line.as_str().to_string());
            }
        }
        if new_bags.len() == 0 {
            break;
        }
        find_bags = new_bags;
        cs = new_cs;
    }

    println!("Solution 1: {}", hits);
}

fn solution_2(bag: &str, contents: &Vec<String>) -> usize {
    let search_term = format!("{} bags contain", bag);
    let line_search_re = Regex::new(search_term.as_str()).unwrap();
    let containing_bags_re = Regex::new(r"\d\s[a-z]+\s[a-z]+").unwrap();
    let digit_re = Regex::new(r"\d").unwrap();
    let bag_re = Regex::new(r"[a-z]+\s[a-z]+").unwrap();
    let r: usize = contents
        .iter()
        .filter(|l| line_search_re.is_match(l))
        .flat_map(|l| containing_bags_re.find_iter(l))
        .map(|m| {
            let number = digit_re
                .find(m.as_str())
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let bag = bag_re.find(m.as_str()).unwrap().as_str();
            let r = (number, solution_2(bag, contents));
            r
        })
        .fold(0, |acc, (bags, multiplier)| {
            acc + (bags + (bags * multiplier))
        });
    r
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("something went wrong");
    let cs: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    //solution_1();
    let s2 = solution_2("shiny gold", &cs);
    println!("solution_2 -> {}", s2);
}
