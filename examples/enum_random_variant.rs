extern crate strum;
#[macro_use]
extern crate strum_macros;

use strum::IntoEnumIterator;

use rand::Rng;

#[derive(Display, EnumIter, Debug)]
enum Greeting {
    HelloThere,
    WhatsUpGuy,
    Yo,
}

// Good for testing
fn random_variant() {
    let mut rng = rand::thread_rng();
    let n: usize = rng.gen::<usize>() % Greeting::iter().count();
    let random = Greeting::iter().nth(n);
    dbg!(random);
}

fn main() {
    random_variant();
}
