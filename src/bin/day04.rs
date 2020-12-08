use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Field {
    BirthYear,
    CountryID,
    ExpirationYear,
    EyeColor,
    HairColor,
    Height,
    IssueYear,
    PassportID,
}

impl Field {
    fn new(name: &str) -> Option<Self> {
        match name {
            "byr" => Some(Self::BirthYear),
            "cid" => Some(Self::CountryID),
            "ecl" => Some(Self::EyeColor),
            "eyr" => Some(Self::ExpirationYear),
            "hcl" => Some(Self::HairColor),
            "hgt" => Some(Self::Height),
            "iyr" => Some(Self::IssueYear),
            "pid" => Some(Self::PassportID),
            _ => None,
        }
    }
}

type Entry = HashMap<Field, String>;

fn valid_part1(entry: &Entry) -> bool {
    let cid = entry.contains_key(&Field::CountryID);
    let count = entry.keys().count();
    match (count, cid) {
        (8, _) => true,
        (7, false) => true,
        _ => false,
    }
}

fn valid_height(input: &str) -> Option<bool> {
    let re = Regex::new(r"^(?P<value>[[:digit:]]+)(?P<unit>[[:alpha:]]+)$").unwrap();
    let captures = re.captures(input)?;

    let value: i32 = captures.name("value")?.as_str().parse().ok()?;
    let unit = captures.name("unit")?.as_str();

    match unit {
        "cm" => Some(150 <= value && value <= 193),
        "in" => Some(59 <= value && value <= 76),
        _ => None,
    }
}

fn valid_hair_color(input: &str) -> Option<bool> {
    let re = Regex::new(r"^#(?P<color>[[:digit:][a-f]]{6}$)").unwrap();
    let captures = re.captures(input)?;

    captures.name("color").map(|_| true)
}

fn valid_eye_color(input: &str) -> Option<bool> {
    Some(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&input))
}

fn valid_passport_id(input: &str) -> Option<bool> {
    let re = Regex::new(r"^(?P<pid>[[:digit:]]{9}$)").unwrap();
    let captures = re.captures(input)?;

    captures.name("pid").map(|_| true)
}

fn valid_part2(entry: &Entry) -> bool {
    let byr = entry
        .get(&Field::BirthYear)
        .and_then(|year| year.parse().ok())
        .map(|year| 1920 <= year && year <= 2002);

    let iyr = entry
        .get(&Field::IssueYear)
        .and_then(|year| year.parse().ok())
        .map(|year| 2010 <= year && year <= 2020);

    let eyr = entry
        .get(&Field::ExpirationYear)
        .and_then(|year| year.parse().ok())
        .map(|year| 2020 <= year && year <= 2030);

    let hgt = entry
        .get(&Field::Height)
        .and_then(|height| valid_height(height));

    let hcl = entry
        .get(&Field::HairColor)
        .and_then(|color| valid_hair_color(color));

    let ecl = entry
        .get(&Field::EyeColor)
        .and_then(|color| valid_eye_color(color));

    let pid = entry
        .get(&Field::PassportID)
        .and_then(|id| valid_passport_id(id));

    // cid ignored

    byr == Some(true)
        && iyr == Some(true)
        && eyr == Some(true)
        && hgt == Some(true)
        && hcl == Some(true)
        && ecl == Some(true)
        && pid == Some(true)
}

fn parse_line(line: &str) -> Option<Entry> {
    let line = line.trim();

    let mut ret = Entry::new();

    for parts in line.split(' ') {
        let mut kv = parts.split(':');
        let field = kv.next()?;
        let value = kv.next()?;

        let field = Field::new(field)?;
        ret.insert(field, value.to_string());
    }

    Some(ret)
}

fn parse(lines: &[&str]) -> Vec<Entry> {
    let mut entries = Vec::<Entry>::new();

    let groups = lines.split(|&line| line.is_empty());

    for group in groups {
        let mut cur = Entry::new();
        for line in group {
            let kv = parse_line(&line).unwrap_or_default();
            for (field, value) in kv {
                cur.insert(field, value);
            }
        }
        entries.push(cur);
    }

    entries
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day04.txt")?;
    let lines: Vec<&str> = file.lines().collect();

    let entries = parse(lines.as_slice());

    println!(
        "Part 1 {:?}",
        entries.iter().filter(|&e| valid_part1(e)).count()
    );
    println!(
        "Part 2 {:?}",
        entries.iter().filter(|&e| valid_part2(e)).count()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let entry = Entry::new();
        assert_eq!(valid_part1(&entry), false);
    }

    #[test]
    fn complete_record() {
        let mut entry = Entry::new();

        entry.insert(Field::BirthYear, "1937".to_string());
        entry.insert(Field::CountryID, "147".to_string());
        entry.insert(Field::ExpirationYear, "2020".to_string());
        entry.insert(Field::EyeColor, "gry".to_string());
        entry.insert(Field::HairColor, "#fffffd".to_string());
        entry.insert(Field::Height, "183cm".to_string());
        entry.insert(Field::IssueYear, "2017".to_string());
        entry.insert(Field::PassportID, "860033327".to_string());

        assert_eq!(valid_part1(&entry), true);
    }

    #[test]
    fn missing_height() {
        let mut entry = Entry::new();

        entry.insert(Field::BirthYear, "1937".to_string());
        entry.insert(Field::CountryID, "147".to_string());
        entry.insert(Field::ExpirationYear, "2020".to_string());
        entry.insert(Field::EyeColor, "gry".to_string());
        entry.insert(Field::HairColor, "#fffffd".to_string());
        entry.insert(Field::IssueYear, "2017".to_string());
        entry.insert(Field::PassportID, "860033327".to_string());

        assert_eq!(valid_part1(&entry), false);
        assert_eq!(valid_part2(&entry), false);
    }

    #[test]
    fn missing_cid() {
        let mut entry = Entry::new();

        entry.insert(Field::BirthYear, "1937".to_string());
        entry.insert(Field::ExpirationYear, "2020".to_string());
        entry.insert(Field::EyeColor, "gry".to_string());
        entry.insert(Field::HairColor, "#fffffd".to_string());
        entry.insert(Field::Height, "183cm".to_string());
        entry.insert(Field::IssueYear, "2017".to_string());
        entry.insert(Field::PassportID, "860033327".to_string());

        assert_eq!(valid_part1(&entry), true);
        assert_eq!(valid_part2(&entry), true);
    }

    #[test]
    fn parse_empty_line() {
        assert_eq!(parse_line(""), None);
    }

    #[test]
    fn parse_record_fragment() {
        let mut entry = Entry::new();

        entry.insert(Field::BirthYear, "1937".to_string());
        entry.insert(Field::ExpirationYear, "2020".to_string());
        entry.insert(Field::EyeColor, "gry".to_string());

        assert_eq!(parse_line("byr:1937 ecl:gry eyr:2020"), Some(entry));
    }

    #[test]
    fn height_parser() {
        assert_eq!(valid_height("180cm"), Some(true));
        assert_eq!(valid_height("200cm"), Some(false));
        assert_eq!(valid_height("not height"), None);
    }

    #[test]
    fn hair_color_parser() {
        assert_eq!(valid_hair_color("#123456"), Some(true));
        assert_eq!(valid_hair_color("#abcfef"), Some(true));
    }

    #[test]
    fn pid_parser() {
        assert_eq!(valid_passport_id("000000001"), Some(true));
        assert_eq!(valid_passport_id("0123456789"), None);
    }

    #[test]
    fn part2_valid_passports() {
        let text: Vec<_> = "
            pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f

            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022

            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        "
        .lines()
        .collect();

        assert_eq!(parse(&text).iter().filter(|&e| valid_part2(e)).count(), 4);
    }

    #[test]
    fn part2_invalid_passports() {
        let text: Vec<_> = "
            eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946

            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007
        "
        .lines()
        .collect();

        assert_eq!(parse(&text).iter().filter(|&e| valid_part2(e)).count(), 0);
    }
}
