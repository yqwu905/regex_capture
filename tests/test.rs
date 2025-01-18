use regex::Regex;
use regex_capture::RegexCapture;
use std::error::Error;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(PartialEq, Debug)]
enum Gender {
    Male,
    Female,
}

impl FromStr for Gender {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            _ => Err("invalid gender".to_string()),
        }
    }
}

fn gender_parser(s: &str) -> Result<Gender, String> {
    match s {
        "male" => Ok(Gender::Male),
        "female" => Ok(Gender::Female),
        _ => Err("invalid gender".to_string()),
    }
}

#[derive(RegexCapture)]
#[converter(regex = r"name=(?P<name>.+?), age=(?P<age>\d+), gender=(?P<gender>male|female)")]
struct Person {
    name: String,
    age: u32,
    #[converter(func = gender_parser)]
    gender: Gender,
}

#[test]
fn custom_parser_test() {
    static PERSON_STR: &str = "name=Chihaya Anon, age=15, gender=female";
    let pre = Person::from_str(PERSON_STR).unwrap();
    assert_eq!(pre.gender, Gender::Female);
    assert_eq!(pre.age, 15);
    assert_eq!(pre.name, "Chihaya Anon");
}

#[test]
fn trait_parser_test() {
    static PERSON_STR: &str = "name=Chihaya Anon, age=15, gender=female";
    let pre = Person::from_str(PERSON_STR).unwrap();
    assert_eq!(pre.gender, Gender::Female);
    assert_eq!(pre.age, 15);
    assert_eq!(pre.name, "Chihaya Anon");
}
