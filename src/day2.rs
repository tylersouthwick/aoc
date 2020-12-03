use crate::inputs::read_input;

#[derive(Clone, Debug, PartialEq)]
struct PasswordPolicy {
    position1 : usize,
    position2 : usize,
    character : char
}

impl PasswordPolicy {
    fn from_str(s : &str) -> PasswordPolicy {
        let tokens : Vec<&str> = s.split(" ").collect();
        let range: Vec<usize> = tokens[0].split("-")
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect();
        PasswordPolicy {
            position1: range[0],
            position2: range[1],
            character: tokens[1].chars().next().unwrap(),
        }
    }

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

pub fn main() -> std::io::Result<()> {
    let contents = read_input(2)?;

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
            policy: PasswordPolicy {
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
            policy: PasswordPolicy {
                position1: 1,
                position2: 3,
                character: 'b'
            }
        };
        assert_eq!(password1.is_valid(), false);

        let password2 = Password {
            password: "ccccccccc".to_string(),
            policy: PasswordPolicy {
                position1: 1,
                position2: 3,
                character: 'b'
            }
        };
        assert_eq!(password2.is_valid(), false)
    }
}
