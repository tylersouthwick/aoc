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
    fn validate_byr(s : &str) -> bool {
        match s.parse::<i32>() {
            Ok(i) => i >= 1920 && i <= 2002,
                _ => false,
        }
    }
    fn validate_iyr(s : &str) -> bool {
        match s.parse::<i32>() {
            Ok(i) => i >= 2010 && i <= 2020,
                _ => false,
        }
    }
    fn validate_eyr(s : &str) -> bool {
        match s.parse::<i32>() {
            Ok(i) => i >= 2020 && i <= 2030,
                _ => false,
        }
    }
    fn validate_hgt(s : &str) -> bool {
        if s.ends_with("cm") {
            match s[0..(s.len() - 2)].parse::<i32>() {
                Ok(cm) => cm >= 150 && cm <= 193,
                _ => false,
            }
        } else if s.ends_with("in") {
            match s[0..(s.len() - 2)].parse::<i32>() {
                Ok(inches) => inches >= 59 && inches <= 76,
                _ => false,
            }
        } else {
            false
        }
    }
    fn is_hex(c : char) -> bool {
        match c {
            '0'..='9' => true,
            'A'..='F' => true,
            'a'..='f' => true,
            _ => false,
        }
    }

    fn validate_hcl(s : &str) -> bool {
        if s.len() != 7 || !s.starts_with("#") {
            false
        } else {
            s[1..s.len()].chars().map(Passport::is_hex).fold(true, |a, b| a && b)
        }
    }
    fn validate_ecl(s : &str) -> bool {
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s)
    }
    fn validate_pid(s : &str) -> bool {
        if s.len() != 9 {
            false
        } else {
            s.chars().map(char::is_numeric).fold(true, |a, b| a && b)
        }
    }
    fn is_valid_part1(&self) -> bool {
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

    fn is_valid_part2(&self) -> bool {
        if !self.is_valid_part1() {
            false
        } else {
        let fields : Vec<(Option<&String>, fn(&str) -> bool)> = vec![
            (self.birth_year.as_ref(), Passport::validate_byr),
            (self.issue_year.as_ref(), Passport::validate_iyr),
            (self.expiration_year.as_ref(), Passport::validate_eyr),
            (self.height.as_ref(), Passport::validate_hgt),
            (self.hair_color.as_ref(), Passport::validate_hcl),
            (self.eye_color.as_ref(), Passport::validate_ecl),
            (self.passport_id.as_ref(), Passport::validate_pid)
        ];
        fields.iter()
            .map( |(field, validator)|  match field {
                Some(s) => validator(s.as_str()),
                None => false,
            })
        .fold(true, |a, b| a && b)
        }
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
    fn valid_passports_part1(&self) -> usize {
        self.passports.iter()
            .filter(|passport| passport.is_valid_part1())
            .collect::<Vec<&Passport>>()
            .len()
    }

    fn valid_passports_part2(&self) -> usize {
        self.passports.iter()
            .filter(|passport| passport.is_valid_part2())
            .collect::<Vec<&Passport>>()
            .len()
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.passports.len()
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

    println!("part1: found {} valid passports", passports.valid_passports_part1());
    println!("part2: found {} valid passports", passports.valid_passports_part2());

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

    mod part1 {
        use super::*;
        #[rstest(passport_id, is_valid,
            case("860033327", true),
            case("028048884", false),
            case("760753108", true),
            case("166559648", false),
            ::trace
        )]
            fn validate_passports_part1(passport_id : &str, is_valid : bool) {
                let passports = Passports::from_str(PASSPORTS_SAMPLE).unwrap();

                assert_eq!(passports.find_by_id(passport_id).unwrap().is_valid_part1(), is_valid);
            }

        #[test]
        fn valid_passport_count() {
            let passports = Passports::from_str(PASSPORTS_SAMPLE).unwrap();

            assert_eq!(passports.valid_passports_part1(), 2);
        }
    }

    mod part2 {
        use super::*;

        mod field_validating {
            use paste::paste;
            use super::super::*;

            macro_rules! field_tests {
                ($($field:ident $valid:ident: $value:expr,)*) => {
                    paste! {
                        $(
                            #[test]
                            fn [<$field _ $valid>]() {
                                let input = $value;
                                let is_valid = stringify!($valid).starts_with("valid");

                                assert_eq!(Passport::[<validate_ $field>](input), is_valid);
                            }
                        )*
                    }
                }
            }

            field_tests! {
                byr valid:   "2002",
                byr invalid: "2003",
                byr invalid2: "a003",

                hgt valid:   "60in",
                hgt valid2:   "190cm",
                hgt invalid: "190in",
                hgt invalid2: "190",

                hcl valid:   "#123abc",
                hcl invalid: "#123abz",
                hcl invalid2: "123abc",

                ecl valid:   "brn",
                ecl invalid: "wat",

                pid valid:   "000000001",
                pid invalid: "0123456789",
            }
        }
        const INVALID_PART2_PASSPORTS : &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        #[test]
        fn invalid_passports() {
            let passports = Passports::from_str(INVALID_PART2_PASSPORTS).unwrap();
            assert_eq!(passports.len(), 4);
            assert_eq!(passports.valid_passports_part2(), 0);
        }

        const VALID_PART2_PASSPORTS : &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        #[test]
        fn valid_passports() {
            let passports = Passports::from_str(VALID_PART2_PASSPORTS).unwrap();
            assert_eq!(passports.len(), 4);
            assert_eq!(passports.valid_passports_part2(), 4);
        }
    }
}
