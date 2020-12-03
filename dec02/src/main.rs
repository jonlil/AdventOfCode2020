use std::io::{self, BufRead};

#[derive(Debug)]
struct PasswordPolicy {
    letter: char,
    min: usize,
    max: usize,
}

#[derive(Debug)]
struct Password {
    policy: PasswordPolicy,
    value: String,
}

impl Password {
    fn is_valid(&self) -> bool {
        let mut iter = self.value.chars();
        let mut counter: usize = 0;

        let first = &iter.nth(self.policy.min - 1).unwrap();
        if first == &self.policy.letter {
            counter = counter + 1;
        }
        let mut iter = self.value.chars();
        let second = &iter.nth(self.policy.max - 1).unwrap();
        if second == &self.policy.letter {
            counter = counter + 1;
        }

        counter == 1
    }
}

fn split(input: &str, delimiter: &str) -> Vec<String> {
    input.split(delimiter)
        .map(|s| s.to_string())
        .collect()
}

impl std::str::FromStr for Password {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = split(input, ": ");
        let policy_parts = split(&parts[0], " ");
        let length_parts = split(&policy_parts[0], "-");

        Ok(Password {
            value: parts[1].to_string(),
            policy: PasswordPolicy {
                letter: policy_parts[1].chars().nth(0).unwrap(),
                min: length_parts[0].parse::<usize>().unwrap(),
                max: length_parts[1].parse::<usize>().unwrap(),
            },
        })
    }
}

fn main() {
    let stdin = io::stdin();
    let passwords: Vec<Password> = stdin.lock()
        .lines()
        .map(|value| value.unwrap().parse().unwrap())
        .collect();

    let valid_passwords: Vec<Password> = passwords.into_iter()
        .filter(|password| password.is_valid())
        .collect();

    for password in &valid_passwords {
        eprintln!("{:?} - letter: {} ({}, {})", password.value, password.policy.letter, password.policy.min, password.policy.max);
    }

    eprintln!("{}", valid_passwords.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate_password() {
        let valid_password: Password = "2-9 c: ccccccccc".parse().unwrap();
        assert_eq!(valid_password.is_valid(), false);
        assert_eq!(valid_password.value, "ccccccccc");
        assert_eq!(valid_password.policy.letter, 'c');
        assert_eq!(valid_password.policy.min, 2);
        assert_eq!(valid_password.policy.max, 9);

        let invalid_password = Password {
            value: "cdefg".to_string(),
            policy: PasswordPolicy {
                min: 1,
                max: 3,
                letter: 'b',
            }
        };

        assert_eq!(invalid_password.is_valid(), false);
    }
}
