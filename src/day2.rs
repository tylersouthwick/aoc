use crate::inputs::read_input;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Clone, Debug, PartialEq)]
struct TobogganCorporatePasswordPolicy {
    position1 : usize,
    position2 : usize,
    character : char
}

#[derive(Clone, Debug, PartialEq)]
struct DownTheStreatPasswordPolicy {
    min: i32,
    max: i32,
    character : char
}

trait PasswordPolicy : Clone {
    fn is_valid(self, password: String) -> bool;
}

impl FromStr for TobogganCorporatePasswordPolicy {
    type Err = ParseIntError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let tokens : Vec<&str> = s.split(" ").collect();
        let range_result : Vec<Result<usize, ParseIntError>> = tokens[0].split("-")
            .map(|x| x.to_string().parse::<usize>())
            .collect();
        let range = transpose_vec_result(range_result)?;
        Ok(TobogganCorporatePasswordPolicy {
            position1: range[0],
            position2: range[1],
            character: tokens[1].chars().next().unwrap(),
        })
    }
}
impl PasswordPolicy for TobogganCorporatePasswordPolicy {
    fn is_valid(self, password : String) -> bool {
        let chars : Vec<char> = password.chars().collect();
        let position1 = self.position1 - 1;
        let position2 = self.position2 - 1;
        if chars[position1] == self.character && chars[position2] == self.character {
            return false;
        } else if chars[position1] == self.character || chars[position2] == self.character {
            return true;
        } else {
            return false;
        }
    }
}

impl FromStr for DownTheStreatPasswordPolicy {
    type Err = ParseIntError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let tokens : Vec<&str> = s.split(" ").collect();
        let range_result : Vec<Result<i32, ParseIntError>> = tokens[0].split("-")
            .map(|x| x.to_string().parse::<i32>())
            .collect();
        let range : Vec<i32> = transpose_vec_result(range_result)?;
        Ok(DownTheStreatPasswordPolicy {
            min: range[0],
            max: range[1],
            character: tokens[1].chars().next().unwrap(),
        })
    }
}
impl PasswordPolicy for DownTheStreatPasswordPolicy {
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
    policy: TobogganCorporatePasswordPolicy,
}
impl FromStr for Password {
    type Err= ParseIntError;

    fn from_str(s : &str) -> Result<Self, ParseIntError> {
        let tokens : Vec<&str> = s.split(":").collect();
        Ok(Password {
            policy: TobogganCorporatePasswordPolicy::from_str(tokens[0])?,
            password: tokens[1].trim().to_string(),
        })
    }
}

impl Password {
    fn is_valid(self) -> bool {
        self.policy.is_valid(self.password)
    }
}

fn transpose_vec_result<T, E>(v:Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    let mut results : Vec<T> = vec![];
    for result in v.iter() {
        let t = result?;
        results.push(t);
    }
    Ok(results)
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = read_input(2)?;

    let lines : Vec<Result<Password, ParseIntError>> = contents.split("\n")
        .filter(|x| !x.is_empty())
        .map(Password::from_str)
        .collect();

    let valid_lines : Vec<&Password> = transpose_vec_result(lines)?
        .iter()
        .filter(|x| x.clone().is_valid())
        .collect();

    println!("found {} valid lines", valid_lines.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_password_policy() {
        assert_eq!(Password::from_str("1-3 a: abcde"), Password {
            password: "abcde".to_string(),
            policy: TobogganCorporatePasswordPolicy {
                position1: 1,
                position2: 3,
                character: 'a'
            }
        })
    }

    #[test]
    fn validate_valid_password_policy() {
        let password = Password {
            password: "abcde".to_string(),
            policy: TobogganCorporatePasswordPolicy {
                position1: 1,
                position2: 3,
                character: 'a'
            }
        };
        assert_eq!(password.is_valid(), true)
    }

    #[test]
    fn validate_invalid_password_policy() {
        let password1 = Password {
            password: "cdefg".to_string(),
            policy: TobogganCorporatePasswordPolicy {
                position1: 1,
                position2: 3,
                character: 'b'
            }
        };
        assert_eq!(password1.is_valid(), false);

        let password2 = Password {
            password: "ccccccccc".to_string(),
            policy: TobogganCorporatePasswordPolicy {
                position1: 1,
                position2: 3,
                character: 'b'
            }
        };
        assert_eq!(password2.is_valid(), false)
    }
}
