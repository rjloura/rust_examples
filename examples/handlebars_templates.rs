use handlebars::Handlebars;
use serde::Deserialize;
use serde_json::json;

use std::io::{BufReader, Error, Write};
use std::fs::File;


static TEMPLATE_PATH: &str = "./examples/template.hbs";
static TEST_CONFIG_FILE: &str = "./examples/config.json";

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Shard {
    pub host: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
struct Config {
    pub domain_name: String,
    pub snaplinks_cleanup_required: Option<bool>,
}
    
fn main() -> Result<(), Error> {
    // Ignore missing file.
    std::fs::remove_file(TEST_CONFIG_FILE).unwrap_or(());

	let config_reg = Handlebars::new();

	let vars = json!({
		"DOMAIN_NAME": "fake.joyent.us",
		"SNAPLINKS_CLEANUP_REQUIRED": false
	});

	let template = std::fs::read_to_string(TEMPLATE_PATH)?;

    let config_data = config_reg.render_template(&template, &vars)
		.expect("Config data");

    println!("{}", config_data);

	let mut file = File::create(TEST_CONFIG_FILE)?;

	file.write_all(config_data.as_bytes())?;//.expect("write all to file");
    drop(file);

    let file = File::open(TEST_CONFIG_FILE).expect("open file");
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).expect("deserialize");

    dbg!(config);
    Ok(())
}
