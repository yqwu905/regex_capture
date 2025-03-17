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

fn convert_to_option_u32(src: &str) -> Result<Option<u32>, String> {
    if src.trim().is_empty() {
        return Ok(None);
    }
    match src.parse::<u32>() {
        Ok(val) => Ok(Some(val)),
        Err(_) => Err(format!("Failed to convert {} to interger", src)),
    }
}

#[derive(RegexCapture)]
#[converter(
    regex = r"(?P<port_type>\S*GE)((?P<chassis>\d+)\/)?(?P<slot>\d+)\/(?P<card>\d+)\/(?P<panel_id>\d+)(:(?P<split>\d+))?"
)]
struct PortName {
    port_type: String,
    #[converter(func = convert_to_option_u32)]
    chassis: Option<u32>,
    slot: u32,
    card: u32,
    panel_id: u32,
    #[converter(func = convert_to_option_u32)]
    split: Option<u32>,
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
fn port_name_test() {
    let port1 = PortName::from_str("100GE1/7/0/14:3");
    assert!(port1.is_ok());
    let port1 = port1.unwrap();
    assert_eq!(port1.port_type, "100GE");
    assert_eq!(port1.chassis, Some(1));
    assert_eq!(port1.slot, 7);
    assert_eq!(port1.card, 0);
    assert_eq!(port1.panel_id, 14);
    assert_eq!(port1.split, Some(3));

    let port2 = PortName::from_str("GE1/0/14");
    assert!(port2.is_ok());
    let port2 = port2.unwrap();
    assert_eq!(port2.port_type, "GE");
    assert_eq!(port2.chassis, None);
    assert_eq!(port2.slot, 1);
    assert_eq!(port2.card, 0);
    assert_eq!(port2.panel_id, 14);
    assert_eq!(port2.split, None);

    let port3 = PortName::from_str("MultiGE2/0/1:1");
    assert!(port3.is_ok());
    let port3 = port3.unwrap();
    assert_eq!(port3.port_type, "MultiGE");
    assert_eq!(port3.chassis, None);
    assert_eq!(port3.slot, 2);
    assert_eq!(port3.card, 0);
    assert_eq!(port3.panel_id, 1);
    assert_eq!(port3.split, Some(1));

    let port4 = PortName::from_str("MultiGE1/2/0/1");
    assert!(port4.is_ok());
    let port4 = port4.unwrap();
    assert_eq!(port4.port_type, "MultiGE");
    assert_eq!(port4.chassis, Some(1));
    assert_eq!(port4.slot, 2);
    assert_eq!(port4.card, 0);
    assert_eq!(port4.panel_id, 1);
    assert_eq!(port4.split, None);
}
