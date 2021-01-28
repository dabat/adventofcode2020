use std::collections::HashMap;
use std::vec;
/*
--- Day 4: Passport Processing ---

You arrive at the airport only to realize that you grabbed your North Pole Credentials instead of your passport.
While these documents are extremely similar, North Pole Credentials aren't issued by a country
and therefore aren't actually valid documentation for travel in most of the world.

It seems like you're not the only one having problems, though;
a very long line has formed for the automatic passport scanners, and the delay could upset your travel itinerary.

Due to some questionable network security, you realize you might be able to solve both of these problems at the same time.

The automatic passport scanners are slow because they're having trouble detecting which passports have all required fields.
The expected fields are as follows:

    byr (Birth Year)
    iyr (Issue Year)
    eyr (Expiration Year)
    hgt (Height)
    hcl (Hair Color)
    ecl (Eye Color)
    pid (Passport ID)
    cid (Country ID)

Passport data is validated in batch files (your puzzle input).
Each passport is represented as a sequence of key:value pairs separated by spaces or newlines.
Passports are separated by blank lines.

Here is an example batch file containing four passports:

ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in

The first passport is valid - all eight fields are present.
The second passport is invalid - it is missing hgt (the Height field).
The third passport is interesting; the only missing field is cid, so it looks like data from North Pole Credentials,
not a passport at all! Surely, nobody would mind if you made the system temporarily ignore missing cid fields.
Treat this "passport" as valid.
The fourth passport is missing two fields, cid and byr. Missing cid is fine, but missing any other field is not,
so this passport is invalid.
According to the above rules, your improved system would report 2 valid passports.

Count the number of valid passports - those that have all required fields.
Treat cid as optional. In your batch file, how many passports are valid?
https://adventofcode.com/2020/day/4
total passports: 285
valid passports: 208
*/

pub fn count_valid_passports(use_rules: bool) {
    // import the passport input file into a vector of Passport structs
    let passport_strings = read_passport_file();
    // println!("{:?}", passport_strings);
    let mut passports: Vec<Passport> = vec![];
    for passport_string in passport_strings {
        let passport = passport_from_info(&passport_string);
        // println!("{:?}", passport);
        passports.push(passport);
    }
    // iterate through the vector and count the valid passports
    let mut valid_passport_count = 0;
    for passport in &passports {
        if use_rules {
            if passport.is_valid_rules() {
                valid_passport_count += 1;
            }
        } else {
            if passport.is_valid() {
                valid_passport_count += 1;
            }
        }
    }

    // print out the count of valid passports
    println!("total passports: {}", &passports.len());
    println!("valid passports: {}", valid_passport_count);
}

#[derive(Debug)]
pub struct Passport {
    byr: Option<String>, //Birth Year
    cid: Option<String>, //Country ID
    ecl: Option<String>, //Eye Color
    eyr: Option<String>, //Expiration Year
    hcl: Option<String>, //Hair Color
    hgt: Option<String>, //Height
    iyr: Option<String>, //Issue Year
    pid: Option<String>, //Passport ID
}

impl Passport {
    fn new(
        byr: Option<String>,
        cid: Option<String>,
        ecl: Option<String>,
        eyr: Option<String>,
        hcl: Option<String>,
        hgt: Option<String>,
        iyr: Option<String>,
        pid: Option<String>,
    ) -> Passport {
        Passport {
            byr,
            cid,
            ecl,
            eyr,
            hcl,
            hgt,
            iyr,
            pid,
        }
    }
    fn is_valid(&self) -> bool {
        if self.byr.is_some()
            && self.ecl.is_some()
            && self.eyr.is_some()
            && self.hcl.is_some()
            && self.hgt.is_some()
            && self.iyr.is_some()
            && self.pid.is_some()
        {
            return true;
        }
        false
    }
    fn is_valid_rules(&self) -> bool {
        let mut is_valid = false;
        let mut byr_is_valid = false;
        let mut iyr_is_valid = false;
        let mut eyr_is_valid = false;
        let mut hgt_is_valid = false;
        let mut hcl_is_valid = false;

        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if self.byr.is_some() {
            let birth_year: i32 = self.byr.to_owned().unwrap().parse().unwrap_or_default();
            match birth_year {
                1920..=2002 => byr_is_valid = true,
                _ => byr_is_valid = false,
            }
        }
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if self.iyr.is_some() {
            let issue_year: i32 = self.iyr.to_owned().unwrap().parse().unwrap_or_default();
            match issue_year {
                2010..=2020 => iyr_is_valid = true,
                _ => iyr_is_valid = false,
            }
        }
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if self.eyr.is_some() {
            let expiration_year: i32 = self.eyr.to_owned().unwrap().parse().unwrap_or_default();
            match expiration_year {
                2020..=2030 => eyr_is_valid = true,
                _ => eyr_is_valid = false,
            }
        }
        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        if self.hgt.is_some() {
            let height: i32;
            let hgt = self.hgt.to_owned().unwrap();

            if hgt.contains("cm") {
                height = hgt[0..hgt.find("cm").unwrap()].parse().unwrap();
                match height {
                    150..=193 => hgt_is_valid = true,
                    _ => hgt_is_valid = false,
                }
            } else if hgt.contains("in") {
                height = hgt[0..hgt.find("in").unwrap()].parse().unwrap();
                match height {
                    59..=76 => hgt_is_valid = true,
                    _ => hgt_is_valid = false,
                };
            }
        }
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        if self.hcl.is_some() {
            let hcl = self.hcl.to_owned().unwrap();
            let characters_allowed = vec![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
            ];
            if hcl.chars().nth(0).unwrap() == '#' {
                let hair_color = &hcl[1..];
                if hair_color.len() == 6 {
                    for character in hair_color.chars() {
                        match characters_allowed.contains(&character) {
                            true => hcl_is_valid = true,
                            false => {
                                hcl_is_valid = false;
                                break;
                            }
                        }
                    }
                }
            }
            // println!("{} valid=={}", hcl, hcl_is_valid);
        }
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        // cid (Country ID) - ignored, missing or not.

        if byr_is_valid && iyr_is_valid && eyr_is_valid && hgt_is_valid && hcl_is_valid {
            is_valid = true;
        }
        is_valid
    }
}

fn read_passport_file() -> Vec<String> {
    std::fs::read_to_string("day4_input.txt")
        .unwrap()
        .split("\n\n")
        .into_iter()
        .map(String::from)
        .map(|s| s.replace("\n", " "))
        .collect()
}

fn passport_from_info(info: &str) -> Passport {
    // accept a string with passport info
    // e.g. info = "eyr:2027 hcl:#7d3b0c pid:377701492 ecl:gry byr:1971 hgt:174cm iyr:2023"
    // split the string into parts and collect the parts into a hashmap
    // https://stackoverflow.com/questions/65373328/split-key-value-pairs-from-string-into-hashmap-in-one-line
    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=43f5f4565d37fdb7d79bcddbf1b758c9
    // string.split_whitespace().map(|s| s.split_at(s.find(":").unwrap())).map(|(key, val)| (key, &val[1..])).collect()

    let data: HashMap<String, String> = info
        .split_whitespace()
        .map(|s| s.split_at(s.find(":").unwrap()))
        .map(|(key, value)| (String::from(key), String::from(&value[1..])))
        .collect();

    // create a passport using the hashmap of parts
    Passport::new(
        match data.get("byr") {
            Some(s) => Some(s.to_owned()),
            None => None,
        },
        match data.get("cid") {
            Some(s) => Some(s.to_owned()),
            None => None,
        },
        match data.get("ecl") {
            Some(s) => Some(s.to_owned()),
            None => None,
        },
        match data.get("eyr") {
            Some(s) => Some(s.to_owned()),
            None => None,
        },
        match data.get("hcl") {
            Some(s) => Some(s.to_owned()),
            None => None,
        },
        match data.get("hgt") {
            Some(s) => Some(s.to_owned()),
            None => None,
        },
        match data.get("iyr") {
            Some(s) => Some(s.to_owned()),
            None => None,
        },
        match data.get("pid") {
            Some(s) => Some(s.to_owned()),
            None => None,
        },
    )
}

/*
--- Part Two ---

The line is moving more quickly now,
but you overhear airport security talking about how passports with invalid data are getting through.
Better add some data validation, quick!

You can continue to ignore the cid field,
but each other field has strict rules about what values are valid for automatic validation:

    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.

Your job is to count the passports where all required fields are both present and valid according to the above rules.
Count the number of valid passports - those that have all required fields and valid values.
Continue to treat cid as optional.
In your batch file, how many passports are valid?

implemented `is_valid_rules` method on Passport to solve this problem

*/
