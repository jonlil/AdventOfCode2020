use std::io::{self, BufRead};
use std::collections::BTreeMap;

fn main() {
    let stdin = io::stdin();

    eprintln!(
        "{:?}",
        tokenize_passports(stdin.lock())
            .valid()
            .count(),
    );
}

#[derive(Debug)]
struct Passport(BTreeMap<String, String>);

fn four_digit_year_validator(input: Option<&String>) -> Result<u32, &'static str> {
    if let Some(value) = input {
        if value.len() != 4 {
            return Err("Invalid length");
        }

        value.parse::<u32>().map_err(|_| "Invalid integer")
    } else {
        Err("Invalid value")
    }
}

type ValidationResult = Result<(), &'static str>;
impl Passport {
    fn add(&mut self, key: String, value: String) {
        self.0.insert(key, value);
    }

    fn validate_birthyear(&self) -> ValidationResult {
        self.validate_year_within_range(&"byr".to_string(), 1920..=2002)
    }

    fn validate_expiration_year(&self) -> ValidationResult {
        self.validate_year_within_range(&"eyr".to_string(), 2020..=2030)
    }

    fn validate_year_within_range(
        &self,
        key: &String,
        range: std::ops::RangeInclusive<u32>
    ) -> ValidationResult {
        let year = four_digit_year_validator(self.0.get(key))?;
        if range.contains(&year) {
            Ok(())
        } else {
            Err("Not within range")
        }
    }

    fn validate_issued_year(&self) -> Result<(), &'static str> {
        self.validate_year_within_range(&"iyr".to_string(), 2010..=2020)
            .map_err(|err| {
                if err == "Not within range" {
                    "Passport is expired"
                } else {
                    err
                }
            })
    }

    fn get(&self, key: &str) -> Result<String, &'static str> {
        match self.0.get(&key.to_string()) {
            Some(value) => Ok(value.to_string()),
            None => Err("Key not found"),
        }
    }

    fn validate_eye_color(&self) -> ValidationResult {
        let eye_color = self.get(&"ecl").map_err(|_| "Eye color not found")?;

        let valid_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        if valid_eye_colors.contains(&eye_color.as_str()) {
            Ok(())
        } else {
            Err("Invalid eye color")
        }
    }

    fn validate_height(&self) -> ValidationResult {
        let height = self.get(&"hgt").map_err(|_| "Height not found")?;
        if height.ends_with("cm") {
            let height = height.strip_suffix("cm")
                .unwrap()
                .parse::<u8>()
                .map_err(|_| "Invalid input")?;
            if (150..=193).contains(&height) {
                Ok(())
            } else {
                Err("Invalid length")
            }
        } else if height.ends_with("in") {
            let height = height.strip_suffix("in")
                .unwrap()
                .parse::<u8>()
                .map_err(|_| "Invalid input")?;
            if (59..=76).contains(&height) {
                Ok(())
            } else {
                Err("Invalid length")
            }
        } else {
            Err("Invalid format")
        }
    }

    fn validate_hair_color(&self) -> ValidationResult {
        let hair_color = self.get(&"hcl").map_err(|_| "Hair color not found")?;
        if !hair_color.starts_with("#") || hair_color.len() != 7 {
            return Err("Invalid hex color");
        }

        let chars = hair_color[1..=6].as_bytes();
        for chunk in chars.chunks(2) {
            let potential_hex = std::str::from_utf8(chunk)
                .map_err(|_| "Invalid hex color")?;
            u8::from_str_radix(potential_hex, 16)
                .map_err(|_| "Invalid hex color")?;
        }

        Ok(())
    }

    fn validate_passport_number(&self) -> ValidationResult {
        let pid = self.get(&"pid")?;
        if pid.len() != 9 {
            return Err("Invalid passport number");
        }
        let trimed_passport = pid.trim_start_matches('0');
        let parsed_pid = trimed_passport.parse::<u64>().map_err(|_| "Invalid passport number")?;

        if trimed_passport.len() != parsed_pid.to_string().len() {
            return Err("Invalid passport number");
        }

        Ok(())
    }

    fn validate_fields<'a>(&self) -> Result<(), &'a str> {
        self.validate_birthyear()?;
        self.validate_issued_year()?;
        self.validate_expiration_year()?;
        self.validate_height()?;
        self.validate_hair_color()?;
        self.validate_eye_color()?;
        self.validate_passport_number()?;

        Ok(())
    }

    fn valid(&self) -> bool {
        let num_fields = self.0.len();

        match self.validate_fields() {
            Ok(_) => {},
            Err(_) => return false,
        };

        // Verify that we don't have a extra field
        if num_fields == 7 {
            return true;
        }

        // Verify that the optional key is "cid"
        if self.0.contains_key(&"cid".to_string()) {
            return true;
        }

        false
    }
}

struct PassportCollection(Vec<Passport>);

impl PassportCollection {
    fn new(passports: Vec<Passport>) -> Self {
        PassportCollection(passports)
    }

    fn valid(self) -> impl Iterator<Item = Passport> {
        self.0.into_iter().filter(|p| p.valid())
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

fn tokenize_passports<T: BufRead>(reader: T) -> PassportCollection {
    let mut passport_buffer: Vec<String> = vec![];
    let mut passports: Vec<Vec<String>> = vec![];

    for line in reader.lines() {
        let value = line.unwrap();
        if &value == &"" {
            passports.push(passport_buffer);
            passport_buffer = vec![];
        } else {
            passport_buffer.push(value.to_string());
        }
    }

    if passport_buffer.len() > 0 {
        passports.push(passport_buffer);
    }

    PassportCollection::new(passports.iter().map(|p| {
        let mut passport = Passport(BTreeMap::new());
        for parts in p {
            parts.split(" ").for_each(|v| {
                let key_pair = v.split(":")
                    .map(|v| v)
                    .collect::<Vec<&str>>();

                let key = key_pair[0];
                let value = key_pair[1];

                passport.add(key.to_string(), value.to_string());
            });
        }
        passport
    })
    .collect())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\r\n\
                              byr:1937 iyr:2017 cid:147 hgt:183cm\r\n\
                              \r\n\
                              iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\r\n\
                              hcl:#cfa07d byr:1929\r\n\
                              \r\n\
                              hcl:#ae17e1 iyr:2013\r\n\
                              eyr:2024\r\n\
                              ecl:brn pid:760753108 byr:1931\r\n\
                              hgt:179cm\r\n\
                              \r\n\
                              hcl:#cfa07d eyr:2025 pid:166559648\r\n\
                              iyr:2011 ecl:brn hgt:59in";

    fn get_test_input() -> io::Cursor<&'static str> {
        io::Cursor::new(TEST_INPUT)
    }

    #[test]
    fn test_parsing_input() {
        let passports = tokenize_passports(get_test_input());

        assert_eq!(passports.len(), 4);
    }

    #[test]
    fn test_valid_passports() {
        assert_eq!(tokenize_passports(get_test_input()).valid().count(), 2);
    }

    #[test]
    fn test_validate_birthyear() {
        let mut passport = Passport(BTreeMap::new());
        passport.add("byr".to_string(), "1919".to_string());
        assert_eq!(passport.validate_birthyear(), Err("Not within range"));

        passport.add("byr".to_string(), "1920".to_string());
        assert_eq!(passport.validate_birthyear(), Ok(()));

        passport.add("byr".to_string(), "2002".to_string());
        assert_eq!(passport.validate_birthyear(), Ok(()));

        passport.add("byr".to_string(), "11920".to_string());
        assert_eq!(passport.validate_birthyear(), Err("Invalid length"));
    }

    #[test]
    fn test_validate_issue_year() {
        let mut passport = Passport(BTreeMap::new());
        passport.add("iyr".to_string(), "1919".to_string());
        assert_eq!(passport.validate_issued_year(), Err("Passport is expired"));

        passport.add("iyr".to_string(), "2031".to_string());
        assert_eq!(passport.validate_issued_year(), Err("Passport is expired"));

        passport.add("iyr".to_string(), "2010".to_string());
        assert_eq!(passport.validate_issued_year(), Ok(()));

        passport.add("iyr".to_string(), "11920".to_string());
        assert_eq!(passport.validate_issued_year(), Err("Invalid length"));
    }

    #[test]
    fn test_validate_height() {
        let mut passport = Passport(BTreeMap::new());
        passport.add("hgt".to_string(), "170cm".to_string());
        assert_eq!(passport.validate_height(), Ok(()));

        passport.add("hgt".to_string(), "59cm".to_string());
        assert_eq!(passport.validate_height(), Err("Invalid length"));

        passport.add("hgt".to_string(), "59in".to_string());
        assert_eq!(passport.validate_height(), Ok(()));

        passport.add("hgt".to_string(), "77in".to_string());
        assert_eq!(passport.validate_height(), Err("Invalid length"));

        passport.add("hgt".to_string(), "77".to_string());
        assert_eq!(passport.validate_height(), Err("Invalid format"));
    }

    #[test]
    fn test_validate_hair_color() {
        let mut passport = Passport(BTreeMap::new());
        passport.add("hcl".to_string(), "#HHHHHH".to_string());
        assert_eq!(passport.validate_hair_color(), Err("Invalid hex color"));

        passport.add("hcl".to_string(), "#FFFFFF".to_string());
        assert_eq!(passport.validate_hair_color(), Ok(()));

        passport.add("hcl".to_string(), "#FFF".to_string());
        assert_eq!(passport.validate_hair_color(), Err("Invalid hex color"));

        passport.add("hcl".to_string(), "FFFFFF".to_string());
        assert_eq!(passport.validate_hair_color(), Err("Invalid hex color"));
    }

    #[test]
    fn test_validate_eye_color() {
        let mut passport = Passport(BTreeMap::new());

        passport.add("ecl".to_string(), "wat".to_string());
        assert_eq!(passport.validate_eye_color(), Err("Invalid eye color"));

        passport.add("ecl".to_string(), "brn".to_string());
        assert_eq!(passport.validate_eye_color(), Ok(()));
    }

    #[test]
    fn test_passport_number() {
        let mut passport = Passport(BTreeMap::new());

        passport.add("pid".to_string(), "000000001".to_string());
        assert_eq!(passport.validate_passport_number(), Ok(()));

        passport.add("pid".to_string(), "0123456789".to_string());
        assert_eq!(passport.validate_passport_number(), Err("Invalid passport number"));

        passport.add("pid".to_string(), "0A2345678".to_string());
        assert_eq!(passport.validate_passport_number(), Err("Invalid passport number"));
    }
}
