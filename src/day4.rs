use crate::day::Day;
use std::collections::HashMap;
use regex::Regex;

pub struct Day4 {}

const REQUIRED_FIELDS:[&str;7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn byr_valid(input:&str) -> bool {
    match input.parse::<i32>() {
        Ok(n) => n >= 1920 && n <= 2002,
        Err(_) => false
    }
}

fn iyr_valid(input:&str) -> bool {
    match input.parse::<i32>() {
        Ok(n) => n >= 2010 && n <= 2020,
        Err(_) => false
    }
}

fn eyr_valid(input:&str) -> bool {
    match input.parse::<i32>() {
        Ok(n) => n >= 2020 && n <= 2030,
        Err(_) => false
    }
}


fn hgt_valid(input:&str) -> bool {
    let height_regex = Regex::new(r"([0-9]+)(cm|in)").unwrap();
    match height_regex.captures(input) {
        Some(n) => match n.get(2).unwrap().as_str() {
            "cm" => match n.get(1).unwrap().as_str().parse::<i32>() {
                Ok(m) => m >= 150 && m <=193,
                Err(_) => false
            },
            "in" => match n.get(1).unwrap().as_str().parse::<i32>() {
                Ok(m) => m >= 59 && m <=76,
                Err(_) => false
            },
            _ => false
        }
        None => false
    }
}

fn hcl_valid(input:&str) -> bool {
    let hair_color_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    hair_color_regex.is_match(input)
}

const EYE_COLORS:[&str;7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
fn ecl_valid(input:&str) -> bool {
    EYE_COLORS.contains(&input)
}

fn pid_valid(input:&str) -> bool {
    let passport_id_regex = Regex::new(r"^[0-9]{9}$").unwrap();
    passport_id_regex.is_match(input)
}

impl Day for Day4 {
    fn part1(&self, input: &str) -> String {
        let mut present_fields = Vec::new();
        let mut valid = 0;
        for line in input.lines() {
            if line.trim().len() == 0 {
                log::debug!("{:?}", present_fields);
                if REQUIRED_FIELDS.iter().map(|f| present_fields.contains(f)).fold(true, |a,b| a && b) {
                    log::debug!("valid");
                    valid += 1;
                }
                present_fields = Vec::new();
            } else {
                for field in line.split_whitespace() {
                    present_fields.push(field.split(":").next().unwrap());
                }
            }
        }
        if REQUIRED_FIELDS.iter().map(|f| present_fields.contains(f)).fold(true, |a,b| a && b) {
            log::debug!("valid");
            valid += 1;
        }
        valid.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut present_fields:HashMap<&str, &str> = HashMap::new();
        let mut valid = 0;
        let mut line_number = 0;
        for line in input.lines() {
            if line.trim().len() == 0 {
                if REQUIRED_FIELDS.iter().map(|f| present_fields.contains_key(f)).fold(true, |a,b| a && b) {
                    log::debug!("{} byr: {}", line_number, byr_valid(present_fields.get("byr").unwrap()));
                    log::debug!("{} iyr: {}", line_number, iyr_valid(present_fields.get("iyr").unwrap()));
                    log::debug!("{} eyr: {}", line_number, eyr_valid(present_fields.get("eyr").unwrap()));
                    log::debug!("{} hgt: {}", line_number, hgt_valid(present_fields.get("hgt").unwrap()));
                    log::debug!("{} hcl: {}", line_number, hcl_valid(present_fields.get("hcl").unwrap()));
                    log::debug!("{} ecl: {}", line_number, ecl_valid(present_fields.get("ecl").unwrap()));
                    log::debug!("{} pid: {}", line_number, pid_valid(present_fields.get("pid").unwrap()));
                    if byr_valid(present_fields.get("byr").unwrap())
                        && iyr_valid(present_fields.get("iyr").unwrap())
                        && eyr_valid(present_fields.get("eyr").unwrap())
                        && hgt_valid(present_fields.get("hgt").unwrap())
                        && hcl_valid(present_fields.get("hcl").unwrap())
                        && ecl_valid(present_fields.get("ecl").unwrap())
                        && pid_valid(present_fields.get("pid").unwrap()) {
                        log::debug!("{} valid", line_number);
                        valid += 1;
                    }
                }
                present_fields = HashMap::new();  // <---- I had been missing this /facepalm
            } else {
                for field in line.split_whitespace() {
                    present_fields.insert(field.split(":").next().unwrap(), field.split(":").nth(1).unwrap());
                }
            }
            line_number += 1;
        }
        if REQUIRED_FIELDS.iter().map(|f| present_fields.contains_key(f)).fold(true, |a,b| a && b) {
            if byr_valid(present_fields.get("byr").unwrap())
                && iyr_valid(present_fields.get("iyr").unwrap())
                && eyr_valid(present_fields.get("eyr").unwrap())
                && hgt_valid(present_fields.get("hgt").unwrap())
                && hcl_valid(present_fields.get("hcl").unwrap())
                && ecl_valid(present_fields.get("ecl").unwrap())
                && pid_valid(present_fields.get("pid").unwrap()) {
                valid += 1;
            }
        }

        valid.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day4{}.part1("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"), "2")
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day4{}.part2("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"), "0")
    }

    #[test]
    fn part2_test2() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"), "4")
    }

    #[test]
    fn part2_test_valid() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1920
hcl:#623a2f"), "1")
    }

    #[test]
    fn part2_test_byr_too_low() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1919
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_byr_too_high() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:2003
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_iyr_too_low() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2009 eyr:2030 byr:1921
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_iyr_too_high() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2021 eyr:2030 byr:1921
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_eyr_too_low() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2019 byr:1921
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_eyr_too_high() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2031 byr:1921
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_eyr_invalid() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:abc byr:1921
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_hgt_in_too_low() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:58in ecl:grn iyr:2012 eyr:2030 byr:1921
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_hgt_in_too_high() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:77in ecl:grn iyr:2012 eyr:2030 byr:1921
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_hgt_cm_too_low() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:149cm ecl:grn iyr:2012 eyr:2030 byr:1921
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_hgt_cm_too_high() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:194cm ecl:grn iyr:2012 eyr:2030 byr:1921
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_hgt_invalid() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:cm ecl:grn iyr:2012 eyr:2030 byr:1921
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_hgt_invalid_2() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:13 ecl:grn iyr:2012 eyr:2030 byr:1921
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_hcl_invalid() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1920
hcl:#623a2"), "0")
    }

    #[test]
    fn part2_test_hcl_invalid_2() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1920
hcl:#623a2fa"), "0")
    }

    #[test]
    fn part2_test_hcl_invalid_3() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1920
hcl:#x623a2f"), "0")
    }

    #[test]
    fn part2_test_hcl_invalid_4() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1920
hcl:#62ga2f"), "0")
    }

    #[test]
    fn part2_test_hcl_invalid_5() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1920
hcl:#62Aa2f"), "0")
    }

    #[test]
    fn part2_test_ecl_invalid() {
        assert_eq!(Day4{}.part2("pid:087499704 hgt:74in ecl:aaa iyr:2012 eyr:2030 byr:1920
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_pid_invalid() {
        assert_eq!(Day4{}.part2("pid:087499704a hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1920
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_pid_invalid_2() {
        assert_eq!(Day4{}.part2("pid:a087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1920
hcl:#623a2f"), "0")
    }

    #[test]
    fn part2_test_missing_field() {
        assert_eq!(Day4{}.part2("pid:087499704a hgt:77in iyr:2012 eyr:2030 byr:1921
hcl:#623a2f"), "0")
    }
}
