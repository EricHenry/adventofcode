use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn valid_passport(pd: &String) -> bool {
    let required_passport_fields = vec!["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];
    let valid_fields: Vec<&str> = required_passport_fields
        .clone()
        .into_iter()
        .filter(|k| pd.contains(k))
        .collect();

    valid_fields.len() == 7
}

fn solution_1() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut hits = 0;
    let mut passport: Vec<String> = vec![];
    for line in reader.lines() {
        let l = line?;
        if l.is_empty() {
            // process passport
            let pd = passport.join(" ");
            if valid_passport(&pd) {
                hits += 1;
            }
            // reset passport
            passport = vec![];
        } else {
            passport.push(l);
        }
    }

    println!("Solution 1: {}", hits);

    Ok(())
}

enum PassportRule {
    BirthYear(String),
    IssueYear(String),
    ExpirationYear(String),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportId(String),
    CountryId(String),
    Invalid,
}

fn valid_rule(pr: PassportRule) -> bool {
    match pr {
        PassportRule::BirthYear(byr) => match byr.parse::<usize>() {
            Ok(y) => y >= 1920 && y <= 2002,
            Err(_) => false,
        },
        PassportRule::IssueYear(iyr) => match iyr.parse::<usize>() {
            Ok(y) => y >= 2010 && y <= 2020,
            Err(_) => false,
        },
        PassportRule::ExpirationYear(eyr) => match eyr.parse::<usize>() {
            Ok(y) => y >= 2020 && y <= 2030,
            Err(_) => false,
        },
        PassportRule::Height(hgt) => {
            let cm_re = Regex::new(r"^\d{3}cm$").unwrap();
            let in_re = Regex::new(r"^\d{2}in$").unwrap();
            let hgt_str = hgt.as_str();

            match (cm_re.is_match(hgt_str), in_re.is_match(hgt_str)) {
                (true, false) => match hgt_str.replace("cm", "").parse::<usize>() {
                    Ok(h) => h >= 150 && h <= 193,
                    Err(_) => false,
                },
                (false, true) => match hgt_str.replace("in", "").parse::<usize>() {
                    Ok(h) => h >= 59 && h <= 76,
                    Err(_) => false,
                },
                _ => false,
            }
        }
        PassportRule::HairColor(hcl) => {
            let re = Regex::new(r"^#(\d|[a-f]){6}$").unwrap();
            re.is_match(hcl.as_str())
        }
        PassportRule::EyeColor(ecl) => {
            let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            re.is_match(ecl.as_str())
        }
        PassportRule::PassportId(pid) => {
            let re = Regex::new(r"^\d{9}$").unwrap();
            re.is_match(pid.as_str())
        }
        PassportRule::CountryId(_) => true,
        PassportRule::Invalid => false,
    }
}

fn solution_2() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut hits = 0;
    let mut passport: Vec<String> = vec![];
    for line in reader.lines() {
        let l = line?;
        if l.is_empty() {
            // process passport
            let pd = passport.join(" ");
            if valid_passport(&pd) {
                let valid_passport_data = pd
                    .split(" ")
                    .map(|kv_str| {
                        let mut kv_pair = kv_str.split(":");
                        let k = kv_pair.nth(0).unwrap_or("");
                        let v = kv_pair.nth(0).unwrap_or("");
                        match k {
                            "byr" => valid_rule(PassportRule::BirthYear(v.to_string())),
                            "iyr" => valid_rule(PassportRule::IssueYear(v.to_string())),
                            "eyr" => valid_rule(PassportRule::ExpirationYear(v.to_string())),
                            "hgt" => valid_rule(PassportRule::Height(v.to_string())),
                            "hcl" => valid_rule(PassportRule::HairColor(v.to_string())),
                            "ecl" => valid_rule(PassportRule::EyeColor(v.to_string())),
                            "pid" => valid_rule(PassportRule::PassportId(v.to_string())),
                            "cid" => valid_rule(PassportRule::CountryId(v.to_string())),
                            _ => valid_rule(PassportRule::Invalid),
                        }
                    })
                    .fold(true, |acc, v| acc && v);
                if valid_passport_data {
                    hits += 1;
                }
            }
            // reset passport
            passport = vec![];
        } else {
            passport.push(l);
        }
    }

    println!("Solution 2: {}", hits);

    Ok(())
}

fn main() {
    solution_1().unwrap();
    solution_2().unwrap();
}
