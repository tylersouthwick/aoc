use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct PasswordPolicy {
    min : i32,
    max : i32,
    character : char
}

impl PasswordPolicy {
    fn from_str(s : &str) -> PasswordPolicy {
        let tokens : Vec<&str> = s.split(" ").collect();
        let range: Vec<i32> = tokens[0].split("-")
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect();
        PasswordPolicy {
            min: range[0],
            max: range[1],
            character: tokens[1].chars().next().unwrap(),
        }
    }

    fn is_valid(self, password : String) -> bool {
        let mut count = 0;
        for c in password.chars() {
            if c == self.character {
                count = count + 1;
            }
        }
        //println!("{} has {} {} times", password, self.character, count);
        count <= self.max && count >= self.min
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Password {
    password: String,
    policy: PasswordPolicy
}
impl Password {
    fn from_str(s : &str) -> Password {
        let tokens : Vec<&str> = s.split(":").collect();
        Password {
            policy: PasswordPolicy::from_str(tokens[0]),
            password: tokens[1].trim().to_string(),
        }
    }

    fn is_valid(self) -> bool {
        self.policy.is_valid(self.password)
    }
}
fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines : Vec<Password> = contents.split("\n")
        .filter(|x| !x.is_empty())
        .map(Password::from_str)
        .filter(|x| x.clone().is_valid())
        .collect();

    println!("found {} valid lines", lines.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_password_policy() {
        assert_eq!(Password::from_str("1-3 a: abcde"), Password {
            password: "abcde".to_string(),
            policy: PasswordPolicy {
                min: 1,
                max: 3,
                character: 'a'
            }
        })
    }

    #[test]
    fn validate_valid_password_policy() {
        let password = Password {
            password: "abcde".to_string(),
            policy: PasswordPolicy {
                min: 1,
                max: 3,
                character: 'a'
            }
        };
        assert_eq!(password.is_valid(), true)
    }

    #[test]
    fn validate_invalid_password_policy() {
        let password = Password {
            password: "cdefg".to_string(),
            policy: PasswordPolicy {
                min: 1,
                max: 3,
                character: 'b'
            }
        };
        assert_eq!(password.is_valid(), false)
    }
}
