use crate::inputs::read_input;
use std::str::FromStr;
use std::{error::Error, fmt};
use std::collections::HashMap;

#[cfg(test)]
use rstest::rstest;

#[derive(Debug, PartialEq, Clone)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

#[derive(Debug)]
struct PassportParseError;

impl Error for PassportParseError {}
impl fmt::Display for PassportParseError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse passport")
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        let fields = vec![
            self.birth_year.as_ref(),
            self.issue_year.as_ref(),
            self.expiration_year.as_ref(),
            self.height.as_ref(),
            self.hair_color.as_ref(),
            self.eye_color.as_ref(),
            self.passport_id.as_ref(),
            //no cid is ok as that means its from the north pole
            //self.cid.as_ref(),
        ];
        fields.iter()
            .map(|x| x.is_some())
            .fold(true, |a, b| a && b)
    }
}
impl FromStr for Passport {
    type Err = PassportParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens : HashMap<&str, &str> = s.split_whitespace()
            .map(|x| x.split(":").collect::<Vec<&str>>())
            .map(|x| (x[0], x[1]))
            .collect();
        Ok(Passport{
            birth_year: tokens.get("byr").map(|x| x.to_string()),
            issue_year: tokens.get("iyr").map(|x| x.to_string()),
            expiration_year: tokens.get("eyr").map(|x| x.to_string()),
            height: tokens.get("hgt").map(|x| x.to_string()),
            hair_color: tokens.get("hcl").map(|x| x.to_string()),
            eye_color: tokens.get("ecl").map(|x| x.to_string()),
            passport_id: tokens.get("pid").map(|x| x.to_string()),
            country_id: tokens.get("cid").map(|x| x.to_string()),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Passports {
    passports : Vec<Passport>
}

impl Passports {
    fn valid_passports(&self) -> usize {
        self.passports.iter()
            .filter(|passport| passport.is_valid())
            .collect::<Vec<&Passport>>()
            .len()
    }

    #[cfg(test)]
    fn find_by_id(&self, id : &str) -> Option<&Passport> {
        let matches : Vec<&Passport> = self.passports.iter()
            .filter(|passport| match &passport.passport_id {
                None => false,
                Some(passport_id) => *passport_id == id.to_string(),
            })
            .collect();
        if matches.len() == 0 {
            None
        } else {
            Some(matches[0])
        }
    }
}
impl FromStr for Passports {
    type Err = PassportParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passports : Vec<Passport> = vec![];

        let mut current : Vec<String> = vec![];

        for line in s.split("\n") {
            if line.is_empty() {
                let passport = Passport::from_str(current.join("\n").as_str())?;
                passports.push(passport);
                current.clear();
            } else {
                current.push(line.to_string());
            }
        }
        if current.len() > 0 {
            let passport = Passport::from_str(current.join("\n").as_str())?;
            passports.push(passport);
        }
        Ok(Passports{
            passports: passports,
        })
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = read_input(4)?;

    let passports = Passports::from_str(contents.as_str())?;

    println!("part1: found {} valid passports", passports.valid_passports());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PASSPORTS_SAMPLE :&str= "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
    const PASSPORT_SAMPLE :&str= "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";

    #[test]
    fn parse_passport() {
        assert_eq!(Passport::from_str(PASSPORT_SAMPLE).unwrap(), Passport {
            birth_year: Some("1937".to_string()),
            issue_year: Some("2017".to_string()),
            expiration_year: Some("2020".to_string()),
            height: Some("183cm".to_string()),
            hair_color: Some("#fffffd".to_string()),
            eye_color: Some("gry".to_string()),
            passport_id: Some("860033327".to_string()),
            country_id: Some("147".to_string()),
        })
    }

    #[test]
    fn parse_passports() {
        assert_eq!(Passports::from_str(PASSPORTS_SAMPLE).unwrap(), Passports {
            passports: vec![
                Passport {
                    birth_year: Some("1937".to_string()),
                    issue_year: Some("2017".to_string()),
                    expiration_year: Some("2020".to_string()),
                    height: Some("183cm".to_string()),
                    hair_color: Some("#fffffd".to_string()),
                    eye_color: Some("gry".to_string()),
                    passport_id: Some("860033327".to_string()),
                    country_id: Some("147".to_string()),
                },
                Passport {
                    birth_year: Some("1929".to_string()),
                    issue_year: Some("2013".to_string()),
                    expiration_year: Some("2023".to_string()),
                    height: None,
                    hair_color: Some("#cfa07d".to_string()),
                    eye_color: Some("amb".to_string()),
                    passport_id: Some("028048884".to_string()),
                    country_id: Some("350".to_string()),
                },
                Passport {
                    birth_year: Some("1931".to_string()),
                    issue_year: Some("2013".to_string()),
                    expiration_year: Some("2024".to_string()),
                    height: Some("179cm".to_string()),
                    hair_color: Some("#ae17e1".to_string()),
                    eye_color: Some("brn".to_string()),
                    passport_id: Some("760753108".to_string()),
                    country_id: None,
                },
                Passport {
                    birth_year: None,
                    issue_year: Some("2011".to_string()),
                    expiration_year: Some("2025".to_string()),
                    height: Some("59in".to_string()),
                    hair_color: Some("#cfa07d".to_string()),
                    eye_color: Some("brn".to_string()),
                    passport_id: Some("166559648".to_string()),
                    country_id: None,
                },
            ],
        });
    }

    #[rstest(passport_id, is_valid,
        case("860033327", true),
        case("028048884", false),
        case("760753108", true),
        case("166559648", false),
        ::trace
    )]
    fn validate_passports(passport_id : &str, is_valid : bool) {
        let passports = Passports::from_str(PASSPORTS_SAMPLE).unwrap();

        assert_eq!(passports.find_by_id(passport_id).unwrap().is_valid(), is_valid);
    }

    #[test]
    fn valid_passport_count() {
        let passports = Passports::from_str(PASSPORTS_SAMPLE).unwrap();

        assert_eq!(passports.valid_passports(), 2);
    }

}
