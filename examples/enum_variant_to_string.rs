extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::string::ToString;

#[derive(Display, Debug)]
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
}

fn main() {
    variants_to_strings();
}
