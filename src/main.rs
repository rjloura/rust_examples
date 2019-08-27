use reqwest;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Quote {
    quote: String,
    author: String,
}

impl Default for Quote {
    fn default() -> Self {
        Self {
            quote: String::from("It’s not the love you make. It’s the love you give."),
            author: String::from("Nikola Tesla"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Quotes {
    quotes: Vec<Quote>,
}

#[derive(Debug, Clone, Deserialize)]
struct Response {
    contents: Quotes,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            contents: Quotes {
                quotes: vec![Quote::default()],
            },
        }
    }
}

fn main() {
    let mut res = reqwest::get("http://quotes.rest/qod.json").expect("GET error");
    let quote = res.json::<Response>().unwrap_or_default();

    let quote = match quote.contents.quotes.first() {
        Some(q) => q.to_owned(),
        None => Quote::default(),
    };

    println!(
        "These are not the examples you are looking for.\n\
         You need to run \n\n `cargo run --example <example_name>`\n\n\
         But for now here is an inspirational quote to keep you motivated:\n\n"
    );

    println!("{}\n- {}", quote.quote, quote.author)
}
