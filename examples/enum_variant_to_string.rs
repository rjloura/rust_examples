extern crate strum;
#[macro_use]
extern crate strum_macros;

use strum::ParseError::VariantNotFound;
use std::string::ToString;
use std::str::FromStr;

#[derive(Display, EnumString, Debug, PartialEq)]
#[strum(serialize_all = "snake_case")]
enum Greeting {
	HelloThere,
	WhatsUpGuy,
	Yo,
}

fn variants_to_strings() {
    let hello = Greeting::HelloThere;
    let sup_guy = Greeting::WhatsUpGuy;
    let yo = Greeting::Yo;

    println!("{}\n{}\n{}",
             hello.to_string(),
             sup_guy.to_string(),
             yo.to_string());


    assert_eq!(String::from("hello_there"), hello.to_string());
    assert_eq!(String::from("whats_up_guy"), sup_guy.to_string());
    assert_eq!(String::from("yo"), yo.to_string());

    let sup_guy_variant = Greeting::from_str(&sup_guy.to_string());
    dbg!(&sup_guy_variant);
    assert_eq!(sup_guy_variant.unwrap(), Greeting::WhatsUpGuy);

    let not_found_variant = Greeting::from_str(&"How_ya_doin");
    assert_eq!(not_found_variant.unwrap_err(), VariantNotFound);

}

fn main() {
    variants_to_strings();
}
